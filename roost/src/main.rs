#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket::serde::{Deserialize, json::Json};
use rocket::response::status;
use serde_json::Value;


#[post("/generate/<id>", data="<params>")]
fn generate(id: &str, params: Json<Value>) -> status::Accepted<String> {
    // TODO: Go through the process of converting the .scad file to .stl
    status::Accepted(Some(format!("{}:{:?}", id, params)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("../build"))
        .mount("/api", routes![generate])
}
