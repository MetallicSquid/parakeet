use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::path::PathBuf;
use std::{fmt, fs};
use std::io::Read;

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
            image_path = entry_path;
        } else if entry_path.extension().unwrap() == "scad" && valid_model {
            scad_path = entry_path;
        } else if entry_path.extension().unwrap() == "json" && valid_model {
            info_path = entry_path;
        }
    }

    if valid_model {
        Ok(vec![(image_path, scad_path, info_path)])
    } else {
        Ok(model_vec)
    }
}

// Representations of valid parameters
#[derive(Serialize, Deserialize, Debug)]
pub enum ParamType {
    BoolParam(bool),
    IntParam(i64),
    FloatParam(f64),
    StringParam(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ParamRestriction {
    IntRangeRestriction { lower: i64, upper: i64 },
    FloatRangeRestriction { lower: f64, upper: f64 },
    StringLengthRestriction(i64),
    IntListRestriction(Vec<i64>),
    FloatListRestriction(Vec<f64>),
    StringListRestriction(Vec<String>),
    NoRestriction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    pub id: i64,
    pub name: String,
    pub default: ParamType,
    pub restriction: ParamRestriction,
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
// TODO: Check for parameter fields that are not within the spec - these should raise an error
pub fn parse_parameters(
    parameters: &Vec<Value>,
    model_name: &String,
) -> Result<Vec<Parameter>, Box<dyn Error>> {
    let mut parsed_parameters: Vec<Parameter> = Vec::new();
    let mut id_count: i64 = 0;
    for parameter in parameters {
        if parameter["default"].is_boolean() {
            // Bool parameter
            if parameter["lower"].is_null()
                && parameter["upper"].is_null()
                && parameter["upper"].is_null()
                && parameter["maximum"].is_null()
            {
                parsed_parameters.push(Parameter {
                    id: id_count,
                    name: parameter["name"].as_str().unwrap().to_string(),
                    default: ParamType::BoolParam(parameter["default"].as_bool().unwrap()),
                    restriction: ParamRestriction::NoRestriction,
                })
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
                && parameter["maximum"].is_null()
            {
                // Range restricted
                if parameter["lower"].as_i64() < parameter["upper"].as_i64() {
                    parsed_parameters.push(Parameter {
                        id: id_count,
                        name: parameter["name"].as_str().unwrap().to_string(),
                        default: ParamType::IntParam(parameter["default"].as_i64().unwrap()),
                        restriction: ParamRestriction::IntRangeRestriction {
                            lower: parameter["lower"].as_i64().unwrap(),
                            upper: parameter["upper"].as_i64().unwrap(),
                        },
                    });
                } else if parameter["lower"].as_i64() > parameter["upper"].as_i64() {
                    println!("Warning: 'lower' and 'upper' fields for the '{}' parameter in the '{}' model have been swapped", parameter["name"], model_name);
                    parsed_parameters.push(Parameter {
                        id: id_count,
                        name: parameter["name"].as_str().unwrap().to_string(),
                        default: ParamType::IntParam(parameter["default"].as_i64().unwrap()),
                        restriction: ParamRestriction::IntRangeRestriction {
                            lower: parameter["upper"].as_i64().unwrap(),
                            upper: parameter["lower"].as_i64().unwrap(),
                        },
                    });
                } else {
                    Err(RestrictionError::InvalidRange(
                        parameter["name"].as_str().unwrap().to_string(),
                    ))?;
                }
            } else if parameter["allowed"].is_array()
                && parameter["lower"].is_null()
                && parameter["upper"].is_null()
                && parameter["maximum"].is_null()
            {
                // List restricted
                let mut allowed: Vec<i64> = Vec::new();
                for element in parameter["allowed"].as_array().unwrap() {
                    if element.is_i64() {
                        let int_element = element.as_i64().unwrap();
                        if !allowed.contains(&int_element) {
                            allowed.push(int_element)
                        } else {
                            println!("Warning: ignored duplicate value of '{}' in the 'allowed' field for the '{}' parameter in the '{}' model", int_element, parameter["name"], model_name);
                        }
                    } else {
                        Err(RestrictionError::InvalidList(
                            parameter["name"].as_str().unwrap().to_string(),
                        ))?;
                    }
                }
                parsed_parameters.push(Parameter {
                    id: id_count,
                    name: parameter["name"].as_str().unwrap().to_string(),
                    default: ParamType::IntParam(parameter["default"].as_i64().unwrap()),
                    restriction: ParamRestriction::IntListRestriction(allowed),
                });
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
                && parameter["maximum"].is_null()
            {
                // Range restricted
                if parameter["lower"].as_f64() < parameter["upper"].as_f64() {
                    parsed_parameters.push(Parameter {
                        id: id_count,
                        name: parameter["name"].as_str().unwrap().to_string(),
                        default: ParamType::FloatParam(parameter["default"].as_f64().unwrap()),
                        restriction: ParamRestriction::FloatRangeRestriction {
                            lower: parameter["lower"].as_f64().unwrap(),
                            upper: parameter["upper"].as_f64().unwrap(),
                        },
                    });
                } else if parameter["lower"].as_f64() > parameter["upper"].as_f64() {
                    println!("Warning: 'lower' and 'upper' fields for the '{}' parameter in the '{}' model have been swapped", parameter["name"], model_name);
                    parsed_parameters.push(Parameter {
                        id: id_count,
                        name: parameter["name"].as_str().unwrap().to_string(),
                        default: ParamType::FloatParam(parameter["default"].as_f64().unwrap()),
                        restriction: ParamRestriction::FloatRangeRestriction {
                            lower: parameter["upper"].as_f64().unwrap(),
                            upper: parameter["lower"].as_f64().unwrap(),
                        },
                    });
                } else {
                    Err(RestrictionError::InvalidRange(
                        parameter["name"].as_str().unwrap().to_string(),
                    ))?;
                }
            } else if parameter["allowed"].is_array()
                && parameter["lower"].is_null()
                && parameter["upper"].is_null()
                && parameter["maximum"].is_null()
            {
                // List restricted
                let mut allowed: Vec<f64> = Vec::new();
                for element in parameter["allowed"].as_array().unwrap() {
                    if element.is_f64() || element.is_i64() {
                        let float_element = element.as_f64().unwrap();
                        if !allowed.contains(&float_element) {
                            allowed.push(float_element)
                        } else {
                            println!("Warning: ignored duplicate value of '{}' in the 'allowed' field for the '{}' parameter in the '{}' model", float_element, parameter["name"], model_name);
                        }
                    } else {
                        Err(RestrictionError::InvalidList(
                            parameter["name"].as_str().unwrap().to_string(),
                        ))?;
                    }
                }
                parsed_parameters.push(Parameter {
                    id: id_count,
                    name: parameter["name"].as_str().unwrap().to_string(),
                    default: ParamType::FloatParam(parameter["default"].as_f64().unwrap()),
                    restriction: ParamRestriction::FloatListRestriction(allowed),
                });
            } else {
                Err(ParamError::InvalidFormatting(
                    parameter["name"].as_str().unwrap().to_string(),
                ))?;
            }
        } else if parameter["default"].is_string() {
            // String parameter
            if parameter["maximum"].is_i64()
                && parameter["upper"].is_null()
                && parameter["lower"].is_null()
                && parameter["allowed"].is_null()
            {
                // Range restricted
                if parameter["maximum"].as_i64().unwrap() > 0 {
                    parsed_parameters.push(Parameter {
                        id: id_count,
                        name: parameter["name"].as_str().unwrap().to_string(),
                        default: ParamType::StringParam(
                            parameter["default"].as_str().unwrap().to_string(),
                        ),
                        restriction: ParamRestriction::StringLengthRestriction(parameter["maximum"].as_i64().unwrap()),
                    });
                } else {
                    Err(RestrictionError::InvalidRange(
                        parameter["name"].as_str().unwrap().to_string(),
                    ))?;
                }
            } else if parameter["allowed"].is_array()
                && parameter["lower"].is_null()
                && parameter["upper"].is_null()
                && parameter["maximum"].is_null()
            {
                // List restricted
                let mut allowed: Vec<String> = Vec::new();
                for element in parameter["allowed"].as_array().unwrap() {
                    if element.is_string() {
                        let string_element = element.as_str().unwrap().to_string();
                        if !allowed.contains(&string_element) {
                            allowed.push(string_element)
                        } else {
                            println!("Warning: ignored duplicate value of '{}' in the 'allowed' field for the '{}' parameter in the '{}' model", string_element, parameter["name"], model_name);
                        }
                    } else {
                        Err(RestrictionError::InvalidList(
                            parameter["name"].as_str().unwrap().to_string(),
                        ))?;
                    }
                }
                parsed_parameters.push(Parameter {
                    id: id_count,
                    name: parameter["name"].as_str().unwrap().to_string(),
                    default: ParamType::StringParam(
                        parameter["default"].as_str().unwrap().to_string(),
                    ),
                    restriction: ParamRestriction::StringListRestriction(allowed),
                });
            } else {
                Err(ParamError::InvalidFormatting(
                    parameter["name"].as_str().unwrap().to_string(),
                ))?;
            }
        } else {
            Err(TypeError(parameter["name"].as_str().unwrap().to_string()))?;
        }
        id_count += 1;
    }

    Ok(parsed_parameters)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub id: i64,
    pub name: String,
    pub parameters: Vec<Parameter>
}

#[derive(Debug)]
enum ModuleError {
    ModuleNotPresent(String),
    ParameterNotPresent(String, String),
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModuleError::ModuleNotPresent(module) => {
                write!(f, "module '{}' not present in file", module)
            }
            ModuleError::ParameterNotPresent(module, parameter ) => {
                write!(f, "parameter '{}' not present in module '{}'", parameter, module)
            }
        }
    }
}

impl Error for ModuleError {}

// Parse the json modules and the parameters that they contain ensuring existence and restrictions
// FIXME: The representation of parts only as modules is flawed, change this
pub fn parse_modules(modules: &Vec<Value>, model_name: String, scad_path: &PathBuf) -> Result<Vec<Module>, Box<dyn Error>> {
    let mut parsed_modules: Vec<Module> = Vec::new();
    let mut id_count: i64 = 0;
    for module in modules {
        parsed_modules.push(Module {
            id: id_count,
            name: module["name"].as_str().unwrap().to_string(),
            parameters: parse_parameters(&module["parameters"].as_array().unwrap(), &model_name)?
        });
    }

    validate_scad(&parsed_modules, scad_path)?;

    Ok(parsed_modules)
}

// Checks that the provided parameters exist and follow the described type
fn validate_scad(
    modules: &Vec<Module>,
    scad_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let mut scad_file = fs::File::open(scad_path)?;
    let mut scad_string: String = String::new();
    scad_file.read_to_string(&mut scad_string)?;

    let scad_lines: Vec<&str> = scad_string.split("\n").collect();
    let mut scad_modules: Vec<(&str, Vec<&str>)> = Vec::new();
    for line in scad_lines {
        if line.len() > 6 {
            if &line[0..6] == "module" {
                let split_line: Vec<&str> = line[7..line.len()-1].split(&['(', ')'][..]).collect();
                let split_params: Vec<&str> = split_line[1].split(",").collect();

                let mut params: Vec<&str> = Vec::new();
                for param in split_params {
                    let parameter: Vec<&str> = param.split("=").collect();
                    // TODO: Implement type checking logic
                    params.push(parameter[0].trim());
                }
                scad_modules.push((split_line[0].trim(), params))
            }
        }
    }

    for module in modules {
        let mut present: bool = false;
        for scad_module in &scad_modules {
            if module.name == scad_module.0 {
                present = true;
                for parameter in &module.parameters {
                    if !scad_module.1.contains(&parameter.name.as_str()) {
                        Err(ModuleError::ParameterNotPresent(module.name.to_string(), parameter.name.to_string()))?;
                    }
                }
            }
        }

        if !present {
            Err(ModuleError::ModuleNotPresent(module.name.to_string()))?;
        }
    }

    Ok(())
}
