mod manager;
mod database;

#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, NamedFile};
use rocket::serde::{Serialize, json::Json};
use serde_json::Value;
use std::fs;
use std::fs::canonicalize;
use std::path::PathBuf;
use rocket::State;
use rocket_db_pools::{Database, Connection};

#[get("/models")]
async fn get_models(db: &database::Db) -> Json<Vec<database::DisplayModel>> {
    Json(database::get_display_models(db).await.expect("Could not load models from database"))
}

#[get("/models/<id>")]
async fn get_model(db: &database::Db, id: i64) -> Json<database::Model> {
    Json(database::get_model(db, id).await.expect(&format!("Could not load model {} from database", id)))
}

#[derive(Serialize)]
struct GenerateInfo {
    filename: String,
    dimensions: (f64, f64, f64)
}

#[post("/generate/<model_id>/<part_id>", data = "<params>")]
async fn generate_part(db: &database::Db, model_id: i64, part_id: i64, params: Json<Value>, state: &State<manager::ParakeetConfig>) -> Json<GenerateInfo> {
    let model: database::Model = database::get_model(db, model_id).await.expect(&format!("Could not load model {} from database", model_id));
    for part in model.parts {
       if part.part_id == part_id {
           let parameters: Vec<(String, manager::ParamType)> = part.parameters.iter()
               .map(|parameter: &database::Parameter | {
                   let entry: (String, manager::ParamType);
                   match parameter {
                       database::Parameter::IntRange(p) => entry = (p.name.to_string(), manager::ParamType::IntParam(params.0[&p.parameter_id.to_string()].as_i64().unwrap())),
                       database::Parameter::IntList(p) => entry = (p.name.to_string(), manager::ParamType::IntParam(params.0[&p.parameter_id.to_string()].as_i64().unwrap())),
                       database::Parameter::FloatRange(p) => entry = (p.name.to_string(), manager::ParamType::FloatParam(params.0[&p.parameter_id.to_string()].as_f64().unwrap())),
                       database::Parameter::FloatList(p) => entry = (p.name.to_string(), manager::ParamType::FloatParam(params.0[&p.parameter_id.to_string()].as_f64().unwrap())),
                       database::Parameter::StringLength(p) => entry = (p.name.to_string(), manager::ParamType::StringParam(params.0[&p.parameter_id.to_string()].as_str().unwrap().to_string())),
                       database::Parameter::StringList(p) => entry = (p.name.to_string(), manager::ParamType::StringParam(params.0[&p.parameter_id.to_string()].as_str().unwrap().to_string())),
                       database::Parameter::Bool(p) => entry = (p.name.to_string(), manager::ParamType::BoolParam(params.0[&p.parameter_id.to_string()].as_bool().unwrap())),
                   }
                   entry
               }).collect();

           let mut stl_instance: manager::STLInstance = manager::STLInstance {
               model_id,
               part_id,
               parameters,
               command_string: String::new()
           };

           let command_string: String = stl_instance.gen_command_string(part.name, state.build_path.join(model.scad_path).to_str().unwrap().to_string());

           let path: String = stl_instance.get_identifier();
           let exists: bool = stl_instance.does_stl_exist(&state.build_path);
           let enough_space: bool = stl_instance.is_enough_space(&state.build_path, state.model_limit).expect(&format!("Could not read 'stls/' directory in {}", &state.build_path.to_str().unwrap()));

           if !exists && enough_space {
               stl_instance.create_stl(&state.build_path)
                   .expect("Could not create part instance locally");
               database::create_instance(db, database::Instance {
                   part_id,
                   path: path.to_string(),
                   command_string,
                   usage: None,
                   age: None
               })
                   .await
                   .expect(&format!("Could not create part instance with path {} in database", path.to_string()));
           } else if !exists && !enough_space {
               let least_valuable: database::Instance = database::find_least_valuable_instance(db).await
                   .expect("Could not find 'least valuable' instance in database");
               fs::remove_file(&least_valuable.path)
                   .expect(&format!("Could not delete file at {}", &least_valuable.path));
               database::remove_instance(db, &least_valuable.path).await
                   .expect(&format!("Could not remove instance with path {} from database", &least_valuable.path));
               database::create_instance(db, database::Instance {
                   part_id,
                   path: path.to_string(),
                   command_string,
                   usage: None,
                   age: None
               })
                   .await
                   .expect(&format!("Could not create part instance with path {} in database", path.to_string()));
           } else {
               database::increment_instance_usage(db, path.to_string()).await
                   .expect(&format!("Could not read part instance with path {} in database", path.to_string()))
           }

           return Json(GenerateInfo {
               filename: stl_instance.get_identifier(),
               dimensions: stl_instance.get_dimensions(&state.build_path).expect("Could not determine dimensions of the model")
           })
       }
    }

    Json(GenerateInfo {
        filename: String::from(""),
        dimensions: (0.0, 0.0, 0.0)
    })
}

// FIXME: Shouldn't really have to resort to this hack
#[get["/<_id>"]]
async fn pass(_id: i64, state: &State<manager::ParakeetConfig>) -> Option<NamedFile> {
    NamedFile::open(&state.build_path.join("index.html")).await.ok()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config: manager::ParakeetConfig = confy::load("parakeet", None).expect("Could not load config file");

    let _rocket = rocket::build()
        .mount("/", routes![pass])
        .mount("/", FileServer::from(&config.build_path))
        .mount("/api", routes![get_models, get_model, generate_part])
        .attach(database::Db::init())
        .manage(config)
        .launch()
        .await?;

    Ok(())
}
