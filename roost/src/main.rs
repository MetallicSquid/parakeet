mod manager;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::serde::{Serialize, Deserialize, json::Json};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use rocket::Request;

#[get("/models")]
fn get_index() -> Json<Value> {
    let config: manager::ParakeetConfig = confy::load("parakeet").expect("Could not load config file");

    let index_path: PathBuf = Path::join(&config.build_path, "index.json");
    let index_file: String = fs::read_to_string(index_path).expect("Could not open index file");
    let index_json: Value = serde_json::from_str(&index_file).expect("Could not read index file");

    return Json::from(index_json)
}

#[derive(Serialize)]
struct GenerateInfo {
    filename: String,
    dimensions: (f64, f64, f64)
}

// TODO: Support multiple modules
#[post("/generate/<id>", data = "<params>")]
fn generate_model(id: &str, params: Json<Value>) -> Json<GenerateInfo> {
    let config: manager::ParakeetConfig = confy::load("parakeet").expect("Could not load config file");

    let index_path: PathBuf = Path::join(&config.build_path, "index.json");
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

    Json(GenerateInfo {
        filename: model.get_identifier(),
        dimensions: model.get_dimensions().expect("Could not determine dimensions of model")
    })
}

#[get["/<id>"]]
fn pass(id: &str) {}

#[launch]
fn rocket() -> _ {
    let config: manager::ParakeetConfig = confy::load("parakeet").expect("Could not load config file");

    rocket::build()
        .mount("/", routes![pass])
        .mount("/", FileServer::from(config.build_path))
        .mount("/api", routes![generate_model, get_index])
}
