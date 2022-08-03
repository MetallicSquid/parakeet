mod manager;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::serde::{Serialize, Deserialize, json::Json};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[post("/generate/<id>", data = "<params>")]
// TODO: Support multiple modules
fn generate(id: &str, params: Json<Value>) -> Json<String> {
    let config: manager::PathConfig = confy::load("parakeet").expect("Could not load config file");

    let index_path: PathBuf = Path::join(&config.source_path, "index.json");
    let index_file: String = fs::read_to_string(index_path).expect("Could not open index file");
    let index_json: Value = serde_json::from_str(&index_file).expect("Could not read index file");

    let module_name = index_json[0]["modules"][0]["name"].as_str().unwrap();
    let scad_path = index_json[0]["scad_path"].as_str().unwrap();

    let mut model: manager::STLModel = manager::STLModel {
        id: index_json[0]["id"].as_str().unwrap().to_string(),
        config,
        parameters: vec![],
        command_string: String::new(),
        usages: 0
    };

    model.parse_parameters(&index_json[0]["modules"][0]["parameters"], &params);
    model.gen_command_string(module_name.to_string(), scad_path.to_string());
    model.create_stl().expect("Could not generate the .stl file");

    Json(model.get_identifier())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("../build"))
        .mount("/api", routes![generate])
}
