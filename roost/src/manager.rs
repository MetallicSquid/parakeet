use std::error::Error;
use std::{fmt, fs};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use fs::read_to_string;
use std::fs::ReadDir;
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParakeetConfig {
    pub models_path: PathBuf,
    pub build_path: PathBuf,
    pub database_path: PathBuf,
    pub model_limit: i64,
}

impl ::std::default::Default for ParakeetConfig {
    fn default() -> Self {
        Self {
            models_path: PathBuf::new(),
            build_path: PathBuf::new(),
            database_path: PathBuf::new(),
            model_limit: 100
        }
    }
}

#[derive(Debug)]
enum InstanceError {
    ScadError(String),
    NotConfigured(i64, i64)
}

impl fmt::Display for InstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstanceError::ScadError(path) => write!(f, "could not generate part instance (path: {})", path),
            InstanceError::NotConfigured(model_id, part_id) => write!(f, "part instance (model id: {}, part id: {}) has not been fully configured", model_id, part_id)
        }
    }
}

impl Error for InstanceError {}

pub enum ParamType {
    BoolParam(bool),
    IntParam(i64),
    FloatParam(f64),
    StringParam(String),
}

pub struct STLInstance {
    pub model_id: i64,
    pub part_id: i64,
    pub parameters:Vec<(String, ParamType)>,
    pub command_string: String
}

impl STLInstance {
    pub fn gen_command_string(&mut self, part_name: String, scad_path: String) {
        let mut parameter_string: String = String::new();
        for parameter in &self.parameters {
            if let ParamType::IntParam(value) = &parameter.1 {
                parameter_string.push_str(&format!("{}={}, ", parameter.0, value))
            } else if let ParamType::FloatParam(value) = &parameter.1 {
                parameter_string.push_str(&format!("{}={}, ", parameter.0, value))
            } else if let ParamType::BoolParam(value) = &parameter.1 {
                parameter_string.push_str(&format!("{}={}, ", parameter.0, value))
            } else if let ParamType::StringParam(value) = &parameter.1 {
                parameter_string.push_str(&format!("{}={}, ", parameter.0, value))
            }
        }
        parameter_string = parameter_string[0..&parameter_string.len() - 2].to_string();

        let part_scad: String = format!("{}({});", part_name, parameter_string);

        self.command_string = format!("use <{}>;{}", scad_path, part_scad);
    }

    pub fn create_stl(&self, build_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let stl_path: PathBuf = Path::join(build_path, &self.get_identifier());

        let command: Output = Command::new("sh")
            .arg("-c")
            .arg(format!("echo \"{}\" | openscad -o {} /dev/stdin", &self.command_string, stl_path.to_str().unwrap()))
            .output()
            .expect("Could not generate .stl output");

        if !command.status.success() {
            Err(InstanceError::ScadError(stl_path.to_str().unwrap().to_string()))?
        }

        Ok(())
    }

    pub fn get_identifier(&self) -> String {
        let mut value_string = String::new();
        for parameter in &self.parameters {
            if let ParamType::IntParam(value) = &parameter.1 {
                if value_string.is_empty() {
                    value_string.push_str(&value.to_string())
                } else {
                    value_string.push_str(&format!("-{}", value))
                }
            } else if let ParamType::FloatParam(value) = &parameter.1 {
                if value_string.is_empty() {
                    value_string.push_str(&value.to_string())
                } else {
                    value_string.push_str(&format!("-{}", value))
                }
            } else if let ParamType::BoolParam(value) = &parameter.1 {
                if value_string.is_empty() {
                    value_string.push_str(&value.to_string())
                } else {
                    value_string.push_str(&format!("-{}", value))
                }
            } else if let ParamType::StringParam(value) = &parameter.1 {
                if value_string.is_empty() {
                    value_string.push_str(&value)
                } else {
                    value_string.push_str(&format!("-{}", value))
                }
            }
        }

        format!("stls/{}-{}_{}.stl", self.model_id, self.part_id, value_string)
    }

    pub fn get_dimensions(&self, build_path: &PathBuf) -> Result<(f64, f64, f64), Box<dyn Error>> {
        if self.does_stl_exist(build_path) {
            let stl_path: PathBuf = Path::join(build_path, &self.get_identifier());
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

    pub fn is_enough_space(&self, build_path: &PathBuf, model_limit: i64) -> Result<bool, Box<dyn Error>> {
        let stl_dir: ReadDir = fs::read_dir(Path::join(build_path, "stls/"))?;
        if (stl_dir.count() as i64) < model_limit {
            return Ok(true)
        }
        Ok(false)
    }

    pub fn does_stl_exist(&self, build_path: &PathBuf) -> bool {
        let stl_path: PathBuf = Path::join(build_path, &self.get_identifier());

        if stl_path.exists() {
            return true
        }
        false
    }
}
