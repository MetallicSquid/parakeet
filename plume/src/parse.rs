use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::path::PathBuf;
use std::{fmt, fs};
use std::fs::canonicalize;
use std::io::Read;
use sqlx::Acquire;
use sqlx::sqlite::SqlitePool;

// Traverse the provided models directory and extract the relevant files
pub fn traverse_models_dir(
    path: &PathBuf,
    valid_model: bool,
) -> Result<Vec<(PathBuf, PathBuf, PathBuf)>, Box<dyn Error>> {
    let mut image_path: PathBuf = PathBuf::new();
    let mut scad_path: PathBuf = PathBuf::new();
    let mut info_path: PathBuf = PathBuf::new();
    let mut model_vec: Vec<(PathBuf, PathBuf, PathBuf)> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry_path = entry?.path();
        if entry_path.is_dir() && !valid_model {
            let entry_contents = traverse_models_dir(&entry_path, true)?;
            model_vec.extend(entry_contents);
        } else if entry_path.extension().unwrap() == "jpg" && valid_model {
            image_path = canonicalize(entry_path)?;
        } else if entry_path.extension().unwrap() == "scad" && valid_model {
            scad_path = canonicalize(entry_path)?;
        } else if entry_path.extension().unwrap() == "json" && valid_model {
            info_path = canonicalize(entry_path)?;
        }
    }

    if valid_model {
        Ok(vec![(image_path, scad_path, info_path)])
    } else {
        Ok(model_vec)
    }
}

// Errors related to parameter parsing
#[derive(Debug)]
enum ParamError {
    InvalidFormatting(String),
    DoesNotExist(String),
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParamError::InvalidFormatting(name) => {
                write!(f, "invalid parameter formatting for '{}'", name)
            }
            ParamError::DoesNotExist(name) => write!(f, "the '{}' parameter does not exist", name),
        }
    }
}

impl Error for ParamError {}

#[derive(Debug)]
struct TypeError(String);

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unsupported parameter type for '{}'", self.0)
    }
}

impl Error for TypeError {}

#[derive(Debug)]
enum RestrictionError {
    InvalidRange(String),
    InvalidList(String),
}

impl fmt::Display for RestrictionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RestrictionError::InvalidRange(name) => {
                write!(f, "invalid parameter restriction range for '{}'", name)
            }
            RestrictionError::InvalidList(name) => {
                write!(f, "invalid parameter list for '{}'", name)
            }
        }
    }
}

impl Error for RestrictionError {}

// Parse the json parameters and validate their types and restrictions
pub async fn parse_parameters(
    pool: &SqlitePool,
    parameters: &Vec<Value>,
    part_id: i64,
    model_name: &str
) -> Result<(), Box<dyn Error>> {
    for parameter in parameters {
        if parameter["default"].is_boolean() {
            // Bool parameter
            if parameter["lower"].is_null()
                && parameter["upper"].is_null()
                && parameter["upper"].is_null()
                && parameter["length"].is_null()
            {
                db_add_bool_parameter(
                    pool,
                    parameter["name"].as_str().unwrap(),
                    parameter["default"].as_bool().unwrap(),
                    part_id
                ).await?;
            } else {
                Err(ParamError::InvalidFormatting(
                    parameter["name"].as_str().unwrap().to_string(),
                ))?;
            }
        } else if parameter["default"].is_i64() {
            // Integer parameter
            if parameter["lower"].is_i64()
                && parameter["upper"].is_i64()
                && parameter["allowed"].is_null()
                && parameter["length"].is_null()
            {
                // Range restricted
                if parameter["lower"].as_i64() < parameter["upper"].as_i64() {
                    db_add_int_range_parameter(
                        pool,
                        parameter["name"].as_str().unwrap(),
                        parameter["default"].as_i64().unwrap(),
                        parameter["lower"].as_i64().unwrap(),
                        parameter["upper"].as_i64().unwrap(),
                        part_id
                    ).await?;
                } else if parameter["lower"].as_i64() > parameter["upper"].as_i64() {
                    println!("Warning: 'lower' and 'upper' fields for the '{}' parameter in the '{}' model have been swapped", parameter["name"], model_name);
                    db_add_int_range_parameter(
                        pool,
                        parameter["name"].as_str().unwrap(),
                        parameter["default"].as_i64().unwrap(),
                        parameter["upper"].as_i64().unwrap(),
                        parameter["lower"].as_i64().unwrap(),
                        part_id
                    ).await?;
                } else {
                    Err(RestrictionError::InvalidRange(
                        parameter["name"].as_str().unwrap().to_string(),
                    ))?;
                }
            } else if parameter["allowed"].is_array()
                && parameter["lower"].is_null()
                && parameter["upper"].is_null()
                && parameter["length"].is_null()
            {
                // List restricted
                let parameter_id: i64 = db_add_int_list_parameter(
                    pool,
                    parameter["name"].as_str().unwrap(),
                    parameter["default"].as_i64().unwrap(),
                    part_id
                ).await?;

                let mut allowed: Vec<i64> = Vec::new();
                for element in parameter["allowed"].as_array().unwrap() {
                    if element.is_i64() {
                        let int_element = element.as_i64().unwrap();
                        if !allowed.contains(&int_element) {
                            db_add_int_list_item(
                                pool,
                                int_element,
                                parameter_id
                            ).await?;
                        } else {
                            println!("Warning: ignored duplicate value of '{}' in the 'allowed' field for the '{}' parameter in the '{}' model", int_element, parameter["name"], model_name);
                        }
                    } else {
                        Err(RestrictionError::InvalidList(
                            parameter["name"].as_str().unwrap().to_string(),
                        ))?;
                    }
                }
            } else {
                Err(ParamError::InvalidFormatting(
                    parameter["name"].as_str().unwrap().to_string(),
                ))?;
            }
        } else if parameter["default"].is_f64() {
            // Float parameter
            if (parameter["lower"].is_f64() || parameter["lower"].is_i64())
                && (parameter["upper"].is_f64() || parameter["upper"].is_i64())
                && parameter["allowed"].is_null()
                && parameter["length"].is_null()
            {
                // Range restricted
                if parameter["lower"].as_f64() < parameter["upper"].as_f64() {
                    db_add_float_range_parameter(
                        pool,
                        parameter["name"].as_str().unwrap(),
                        parameter["default"].as_f64().unwrap(),
                        parameter["lower"].as_f64().unwrap(),
                        parameter["upper"].as_f64().unwrap(),
                        part_id
                    ).await?;
                } else if parameter["lower"].as_f64() > parameter["upper"].as_f64() {
                    println!("Warning: 'lower' and 'upper' fields for the '{}' parameter in the '{}' model have been swapped", parameter["name"], model_name);
                    db_add_float_range_parameter(
                        pool,
                        parameter["name"].as_str().unwrap(),
                        parameter["default"].as_f64().unwrap(),
                        parameter["upper"].as_f64().unwrap(),
                        parameter["lower"].as_f64().unwrap(),
                        part_id
                    ).await?;
                } else {
                    Err(RestrictionError::InvalidRange(
                        parameter["name"].as_str().unwrap().to_string(),
                    ))?;
                }
            } else if parameter["allowed"].is_array()
                && parameter["lower"].is_null()
                && parameter["upper"].is_null()
                && parameter["length"].is_null()
            {
                // List restricted
                let parameter_id: i64 = db_add_float_list_parameter(
                    pool,
                    parameter["name"].as_str().unwrap(),
                    parameter["default"].as_f64().unwrap(),
                    part_id
                ).await?;

                let mut allowed: Vec<f64> = Vec::new();
                for element in parameter["allowed"].as_array().unwrap() {
                    if element.is_f64() || element.is_i64() {
                        let float_element = element.as_f64().unwrap();
                        if !allowed.contains(&float_element) {
                            db_add_float_list_item(
                                pool,
                                float_element,
                                parameter_id
                            ).await?;
                        } else {
                            println!("Warning: ignored duplicate value of '{}' in the 'allowed' field for the '{}' parameter in the '{}' model", float_element, parameter["name"], model_name);
                        }
                    } else {
                        Err(RestrictionError::InvalidList(
                            parameter["name"].as_str().unwrap().to_string(),
                        ))?;
                    }
                }
            } else {
                Err(ParamError::InvalidFormatting(
                    parameter["name"].as_str().unwrap().to_string(),
                ))?;
            }
        } else if parameter["default"].is_string() {
            // String parameter
            if parameter["length"].is_i64()
                && parameter["upper"].is_null()
                && parameter["lower"].is_null()
                && parameter["allowed"].is_null()
            {
                // Length restricted
                if parameter["length"].as_i64().unwrap() > 0 {
                    db_add_string_length_parameter(
                        pool,
                        parameter["name"].as_str().unwrap(),
                        parameter["default"].as_str().unwrap(),
                        parameter["length"].as_i64().unwrap(),
                        part_id
                    ).await?;
                } else {
                    Err(RestrictionError::InvalidRange(
                        parameter["name"].as_str().unwrap().to_string(),
                    ))?;
                }
            } else if parameter["allowed"].is_array()
                && parameter["lower"].is_null()
                && parameter["upper"].is_null()
                && parameter["length"].is_null()
            {
                // List restricted
                let parameter_id: i64 = db_add_string_list_parameter(
                    pool,
                    parameter["name"].as_str().unwrap(),
                    parameter["default"].as_str().unwrap(),
                    part_id
                ).await?;

                let mut allowed: Vec<&str> = Vec::new();
                for element in parameter["allowed"].as_array().unwrap() {
                    if element.is_string() {
                        let string_element = element.as_str().unwrap();
                        if !allowed.contains(&string_element) {
                            db_add_string_list_item(
                                pool,
                                string_element,
                                parameter_id
                            ).await?;
                        } else {
                            println!("Warning: ignored duplicate value of '{}' in the 'allowed' field for the '{}' parameter in the '{}' model", string_element, parameter["name"], model_name);
                        }
                    } else {
                        Err(RestrictionError::InvalidList(
                            parameter["name"].as_str().unwrap().to_string(),
                        ))?;
                    }
                }
            } else {
                Err(ParamError::InvalidFormatting(
                    parameter["name"].as_str().unwrap().to_string(),
                ))?;
            }
        } else {
            Err(TypeError(parameter["name"].as_str().unwrap().to_string()))?;
        }
    }

    Ok(())
}

#[derive(Debug)]
enum PartError {
    PartNotPresent(String),
    ParameterNotPresent(String, String),
}

impl fmt::Display for PartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PartError::PartNotPresent(part) => {
                write!(f, "part '{}' not present in file", part)
            }
            PartError::ParameterNotPresent(part, parameter) => {
                write!(f, "parameter '{}' not present in part '{}'", parameter, part)
            }
        }
    }
}

impl Error for PartError {}

// Parse the json modules and the parameters that they contain ensuring existence and restrictions
pub async fn parse_parts(pool: &SqlitePool, parts: &Vec<Value>, model_name: &str, model_id: i64, _scad_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    for part in parts {
        let part_id: i64 = db_add_part(
            pool,
            part["name"].as_str().unwrap(),
            model_id
        ).await?;

        parse_parameters(pool, &part["parameters"].as_array().unwrap(), part_id, model_name).await?;
    }

    // validate_scad(&parsed_modules, scad_path)?;

    Ok(())
}

pub async fn db_add_model(pool: &SqlitePool, name: &str, creation_date: &str, description: &str, author: &str, image_path: &str, scad_path: &str) -> Result<i64, Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    let model_id: i64 = sqlx::query!("INSERT INTO Models (name, creation_date, description, author, image_path, scad_path) VALUES (?, ?, ?, ?, ?, ?)",
        name,
        creation_date,
        description,
        author,
        image_path,
        scad_path
    )
        .execute(&mut connection)
        .await?
        .last_insert_rowid();

    Ok(model_id)
}

async fn db_add_part(pool: &SqlitePool, name: &str, model_id: i64) -> Result<i64, Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    let part_id: i64 = sqlx::query!("INSERT INTO Parts (name, model_id) VALUES (?, ?)",
        name,
        model_id
    )
        .execute(&mut connection)
        .await?
        .last_insert_rowid();

    Ok(part_id)
}

async fn db_add_int_range_parameter(pool: &SqlitePool, name: &str, default_value: i64, lower: i64, upper: i64, part_id: i64) -> Result<(), Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    sqlx::query!("INSERT INTO IntRangeParameters (name, default_value, lower, upper, part_id) VALUES (?, ?, ?, ?, ?)",
        name,
        default_value,
        lower,
        upper,
        part_id
    )
        .execute(&mut connection)
        .await?;

    Ok(())
}

async fn db_add_float_range_parameter(pool: &SqlitePool, name: &str, default_value: f64, lower: f64, upper: f64, part_id: i64) -> Result<(), Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    sqlx::query!("INSERT INTO FloatRangeParameters (name, default_value, lower, upper, part_id) VALUES (?, ?, ?, ?, ?)",
        name,
        default_value,
        lower,
        upper,
        part_id
    )
        .execute(&mut connection)
        .await?;

    Ok(())
}

async fn db_add_string_length_parameter(pool: &SqlitePool, name: &str, default_value: &str, length: i64, part_id: i64) -> Result<(), Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    sqlx::query!("INSERT INTO StringLengthParameters (name, default_value, length, part_id) VALUES (?, ?, ?, ?)",
        name,
        default_value,
        length,
        part_id
    )
        .execute(&mut connection)
        .await?;

    Ok(())
}

async fn db_add_bool_parameter(pool: &SqlitePool, name: &str, default_value: bool, part_id: i64) -> Result<(), Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    sqlx::query!("INSERT INTO BoolParameters (name, default_value, part_id) VALUES (?, ?, ?)",
        name,
        default_value,
        part_id
    )
        .execute(&mut connection)
        .await?;

    Ok(())
}

async fn db_add_int_list_parameter(pool: &SqlitePool, name: &str, default_value: i64, part_id: i64) -> Result<i64, Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    let parameter_id: i64 = sqlx::query!("INSERT INTO IntListParameters (name, default_value, part_id) VALUES (?, ?, ?)",
        name,
        default_value,
        part_id
    )
        .execute(&mut connection)
        .await?
        .last_insert_rowid();

    Ok(parameter_id)
}

async fn db_add_int_list_item(pool: &SqlitePool, value: i64, parameter_id: i64) -> Result<(), Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    sqlx::query!("INSERT INTO IntListItems (value, parameter_id) VALUES (?, ?)",
        value,
        parameter_id
    )
        .execute(&mut connection)
        .await?;

    Ok(())
}

async fn db_add_float_list_parameter(pool: &SqlitePool, name: &str, default_value: f64, part_id: i64) -> Result<i64, Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    let parameter_id: i64 = sqlx::query!("INSERT INTO FloatListParameters (name, default_value, part_id) VALUES (?, ?, ?)",
        name,
        default_value,
        part_id
    )
        .execute(&mut connection)
        .await?
        .last_insert_rowid();

    Ok(parameter_id)
}

async fn db_add_float_list_item(pool: &SqlitePool, value: f64, parameter_id: i64) -> Result<(), Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    sqlx::query!("INSERT INTO FloatListItems (value, parameter_id) VALUES (?, ?)",
        value,
        parameter_id
    )
        .execute(&mut connection)
        .await?;

    Ok(())
}

async fn db_add_string_list_parameter(pool: &SqlitePool, name: &str, default_value: &str, part_id: i64) -> Result<i64, Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    let parameter_id: i64 = sqlx::query!("INSERT INTO FloatListParameters (name, default_value, part_id) VALUES (?, ?, ?)",
        name,
        default_value,
        part_id
    )
        .execute(&mut connection)
        .await?
        .last_insert_rowid();

    Ok(parameter_id)
}

async fn db_add_string_list_item(pool: &SqlitePool, value: &str, parameter_id: i64) -> Result<(), Box<dyn Error>> {
    let mut connection = pool.acquire().await?;

    sqlx::query!("INSERT INTO FloatListItems (value, parameter_id) VALUES (?, ?)",
        value,
        parameter_id
    )
        .execute(&mut connection)
        .await?;

    Ok(())
}

// Checks that the provided parameters exist and follow the described type
// fn validate_scad(
//     modules: &Vec<Module>,
//     scad_path: &PathBuf,
// ) -> Result<(), Box<dyn Error>> {
//     let mut scad_file = fs::File::open(scad_path)?;
//     let mut scad_string: String = String::new();
//     scad_file.read_to_string(&mut scad_string)?;
//
//     let scad_lines: Vec<&str> = scad_string.split("\n").collect();
//     let mut scad_modules: Vec<(&str, Vec<&str>)> = Vec::new();
//     for line in scad_lines {
//         if line.len() > 6 {
//             if &line[0..6] == "module" {
//                 let split_line: Vec<&str> = line[7..line.len()-1].split(&['(', ')'][..]).collect();
//                 let split_params: Vec<&str> = split_line[1].split(",").collect();
//
//                 let mut params: Vec<&str> = Vec::new();
//                 for param in split_params {
//                     let parameter: Vec<&str> = param.split("=").collect();
//                     // TODO: Implement type checking logic
//                     params.push(parameter[0].trim());
//                 }
//                 scad_modules.push((split_line[0].trim(), params))
//             }
//         }
//     }
//
//     for module in modules {
//         let mut present: bool = false;
//         for scad_module in &scad_modules {
//             if module.name == scad_module.0 {
//                 present = true;
//                 for parameter in &module.parameters {
//                     if !scad_module.1.contains(&parameter.name.as_str()) {
//                         Err(ModuleError::ParameterNotPresent(module.name.to_string(), parameter.name.to_string()))?;
//                     }
//                 }
//             }
//         }
//
//         if !present {
//             Err(ModuleError::ModuleNotPresent(module.name.to_string()))?;
//         }
//     }
//
//     Ok(())
// }
