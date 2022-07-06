#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket::serde::{Deserialize, json::Json};
use rocket::response::status;
use serde_json::Value;
use std::fs;

#[derive(Debug)]
enum ParamType {
    BoolParam(bool),
    IntParam(i64),
    FloatParam(f64),
    StringParam(String)
}

#[derive(Debug)]
struct Parameter {
    name: String,
    value: ParamType
}


#[post("/generate/<id>", data="<params>")]
// TODO: Support multiple modules
fn generate(id: &str, params: Json<Value>) -> status::Accepted<String> {
    let index_file: String = fs::read_to_string("../src/index.json").expect("Could not open index file");
    let index_json: Value = serde_json::from_str(&index_file).expect("Could not read index file");

    let mut parameters: Vec<Parameter> = Vec::new();
    for parameter in index_json[0]["modules"][0]["parameters"].as_array().unwrap() {
        let mut value: ParamType;
        let mut raw_value = &params.0[parameter["id"].to_string()];
        if raw_value.is_i64() {
            value = ParamType::IntParam(raw_value.as_i64().unwrap());
        } else if raw_value.is_f64() {
            value = ParamType::FloatParam(raw_value.as_f64().unwrap());
        } else if raw_value.is_boolean() {
            value = ParamType::BoolParam(raw_value.as_bool().unwrap());
        } else {
            value = ParamType::StringParam(raw_value.as_str().unwrap().to_string());
        }

        parameters.push(Parameter {
            name: parameter["name"].as_str().unwrap().to_string(),
            value
        })
    }

    // TODO: Generate the required .scad code and execute it
    // Potential command: echo "scad code" | openscad -o output.stl /dev/stdin
    // This essentially treats the echoed string as a file for openscad
    // removing the need for input file creation + deletion

    println!("{:?}", parameters);

    status::Accepted(Some(format!("{}:{:?}", id, params)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("../build"))
        .mount("/api", routes![generate])
}
