use std::error::Error;
use std::{fmt, fs};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use fs::read_to_string;
use serde_json::Value;
use rocket::{Rocket, Build, futures};
use rocket::fairing::{self, AdHoc};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{sqlx, Database, Connection};

use futures::{stream::TryStreamExt, future::TryFutureExt};

type DbResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParakeetConfig {
    pub models_path: PathBuf,
    pub build_path: PathBuf,
}

impl ::std::default::Default for ParakeetConfig {
    fn default() -> Self {
        Self {
            models_path: PathBuf::new(),
            build_path: PathBuf::new(),
        }
    }
}

#[derive(Debug)]
enum ModelError {
    ScadError(String),
    NotConfigured(String)
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModelError::ScadError(id) => write!(f, "could not generate .stl model (model id: {})", id),
            ModelError::NotConfigured(id) => write!(f, "model '{}' has not been fully configured", id)
        }
    }
}

impl Error for ModelError {}

#[derive(Debug)]
pub enum ParamType {
    BoolParam(bool),
    IntParam(i64),
    FloatParam(f64),
    StringParam(String),
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub value: ParamType,
}

pub struct STLModel {
    pub id: String,
    pub config: ParakeetConfig,
    pub parameters: Vec<Parameter>,
    pub command_string: String,
    pub usages: i64
}

impl STLModel {
    pub fn parse_parameters(&mut self, parameters_json: &Value, params: &Json<Value>) {
        for parameter in parameters_json.as_array().unwrap() {
            let mut value: ParamType;
            let raw_value = &params.0[parameter["id"].to_string()];
            if raw_value.is_i64() {
                value = ParamType::IntParam(raw_value.as_i64().unwrap());
            } else if raw_value.is_f64() {
                value = ParamType::FloatParam(raw_value.as_f64().unwrap());
            } else if raw_value.is_boolean() {
                value = ParamType::BoolParam(raw_value.as_bool().unwrap());
            } else {
                value = ParamType::StringParam(raw_value.as_str().unwrap().to_string());
            }

            self.parameters.push(Parameter {
                name: parameter["name"].as_str().unwrap().to_string(),
                value,
            })
        }
    }

    pub fn gen_command_string(&mut self, module_name: String, scad_path: String) {
        let mut parameter_string: String = String::new();
        for parameter in &self.parameters {
            if let ParamType::IntParam(value) = &parameter.value {
                parameter_string.push_str(&format!("{}={}, ", parameter.name, value))
            } else if let ParamType::FloatParam(value) = &parameter.value {
                parameter_string.push_str(&format!("{}={}, ", parameter.name, value))
            } else if let ParamType::BoolParam(value) = &parameter.value {
                parameter_string.push_str(&format!("{}={}, ", parameter.name, value))
            } else if let ParamType::StringParam(value) = &parameter.value {
                parameter_string.push_str(&format!("{}={}, ", parameter.name, value))
            }
        }
        parameter_string = parameter_string[0..&parameter_string.len() - 2].to_string();

        let module_scad: String = format!("{}({});", module_name, parameter_string);
        let full_scad_path: PathBuf = Path::join(&self.config.build_path, &scad_path);

        self.command_string = format!("use <{}>;{}", full_scad_path.to_str().unwrap(), module_scad);
    }

    pub async fn create_stl(&self, db: Connection<Db>) -> Result<(), Box<dyn Error>> {
        if !self.does_stl_exist()? {
            let stl_path: PathBuf = Path::join(&self.config.build_path, &self.get_identifier());

            let command: Output = Command::new("sh")
                .arg("-c")
                .arg(format!("echo \"{}\" | openscad -o {} /dev/stdin", &self.command_string, stl_path.to_str().unwrap()))
                .output()
                .expect("Could not generate .stl output");

            let bare = BareModel {
                id: self.id.to_string(),
                path: self.get_identifier(),
                usages: 0
            };

            create(db, bare)
                .await?;

            if !command.status.success() {
                Err(ModelError::ScadError(self.id.to_string()))?
            }
        } else {
            increment_usages(db, self.get_identifier())
                .await?;
        }

        Ok(())
    }

    pub fn get_identifier(&self) -> String {
        let mut value_string = String::new();
        for parameter in &self.parameters {
            if let ParamType::IntParam(value) = &parameter.value {
                if value_string.is_empty() {
                    value_string.push_str(&value.to_string())
                } else {
                    value_string.push_str(&format!("-{}", value))
                }
            } else if let ParamType::FloatParam(value) = &parameter.value {
                if value_string.is_empty() {
                    value_string.push_str(&value.to_string())
                } else {
                    value_string.push_str(&format!("-{}", value))
                }
            } else if let ParamType::BoolParam(value) = &parameter.value {
                if value_string.is_empty() {
                    value_string.push_str(&value.to_string())
                } else {
                    value_string.push_str(&format!("-{}", value))
                }
            } else if let ParamType::StringParam(value) = &parameter.value {
                if value_string.is_empty() {
                    value_string.push_str(&value)
                } else {
                    value_string.push_str(&format!("-{}", value))
                }
            }
        }

        format!("stls/{}_{}.stl", self.id, value_string)
    }

    pub fn does_stl_exist(&self) -> Result<bool, Box<dyn Error>> {
        if self.command_string.is_empty() {
            Err(ModelError::NotConfigured(self.id.to_string()))?
        }

        let stl_path: PathBuf = Path::join(&self.config.build_path, &self.get_identifier());

        if stl_path.exists() {
            return Ok(true)
        }
        Ok(false)
    }

    pub fn get_dimensions(&self) -> Result<(f64, f64, f64), Box<dyn Error>> {
        if self.does_stl_exist()? {
            let stl_path: PathBuf = Path::join(&self.config.build_path, &self.get_identifier());
            let stl_contents: String = read_to_string(stl_path)?;

            let mut min_x: f64 = 0.0;
            let mut min_y: f64 = 0.0;
            let mut min_z: f64 = 0.0;
            let mut max_x: f64 = 0.0;
            let mut max_y: f64 = 0.0;
            let mut max_z: f64 = 0.0;

            for line in stl_contents.lines() {
                let split_line: Vec<&str> = line.trim_start().split(" ").collect();
                if split_line[0] == "vertex" {
                    let x: f64 = split_line[1].parse::<f64>()?;
                    let y: f64 = split_line[2].parse::<f64>()?;
                    let z: f64 = split_line[3].parse::<f64>()?;

                    if x < min_x { min_x = x } else if x > max_x { max_x = x };
                    if y < min_y { min_y = y } else if y > max_y { max_y = y };
                    if z < min_z { min_z = z } else if z > max_z { max_z = z };
                }
            }
            return Ok((max_x - min_x, max_y - min_y, max_z - min_z));
        }

        // FIXME: Arguably, this should throw an error
        Ok((0.0, 0.0, 0.0))
    }
}

#[derive(Database)]
#[database("sqlx")]
pub struct Db(sqlx::SqlitePool);

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BareModel {
    pub id: String,
    pub path: String,
    pub usages: i64
}

async fn create(mut db: Connection<Db>, model: BareModel) -> DbResult<(), Box<dyn Error>> {
    sqlx::query!("INSERT INTO models (id, path, usages) VALUES (?, ?, ?)", model.id, model.path, model.usages)
        .execute(&mut *db)
        .await?;

    Ok(())
}

async fn get_usages(mut db: Connection<Db>, path: String) -> DbResult<i64, Box<dyn Error>> {
    let usages: i64 = sqlx::query!("SELECT usages FROM models WHERE path = ?", path)
        .fetch_one(&mut *db)
        .map_ok(|record| record.usages)
        .await?;

    Ok(usages)
}

async fn increment_usages(mut db: Connection<Db>, path: String) -> DbResult<(), Box<dyn Error>> {
    let usages: i64 = sqlx::query!("SELECT usages FROM models WHERE path = ?", path)
        .fetch_one(&mut *db)
        .map_ok(|record| record.usages)
        .await?
        + 1;

    sqlx::query!("UPDATE models SET usages = ? WHERE path = ?", usages, path)
        .execute(&mut *db)
        .await?;

    Ok(())
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("database/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                // FIXME: Should throw an actual error message
                println!("Failed to initialise SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage_db() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}
