// TODO: Implement file deletion algorithm (based on usage and time alive)

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use serde_json::Value;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize, Debug)]
pub struct PathConfig {
    pub models_path: PathBuf,
    pub public_path: PathBuf,
    pub source_path: PathBuf,
}

impl ::std::default::Default for PathConfig {
    fn default() -> Self {
        Self {
            models_path: PathBuf::new(),
            public_path: PathBuf::new(),
            source_path: PathBuf::new(),
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
    pub config: PathConfig,
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
        let full_scad_path: PathBuf = Path::join(&self.config.public_path, &scad_path);

        self.command_string = format!("use <{}>;{}", full_scad_path.to_str().unwrap(), module_scad);
    }

    pub fn create_stl(&self) -> Result<(), Box<dyn Error>> {
        if !self.does_stl_exist()? {
            let stl_path: PathBuf = Path::join(&self.config.public_path, &self.get_identifier());

            let command: Output = Command::new("sh")
                .arg("-c")
                .arg(format!("echo \"{}\" | openscad -o {} /dev/stdin", &self.command_string, stl_path.to_str().unwrap()))
                .output()
                .expect("Could not generate .stl output");

            if !command.status.success() {
                Err(ModelError::ScadError(self.id.to_string()))?
            }
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

        let stl_path: PathBuf = Path::join(&self.config.public_path, &self.get_identifier());

        if stl_path.exists() {
            return Ok(true)
        }
        Ok(false)
    }
}