#[macro_use] extern crate rocket;

use rocket::fs::FileServer;

// #[post("/generate/<id>", data="<params>")]
// fn generate(id: &str, params: Json<Value>) -> status::Accepted<String> {
//     status::Accepted(Some(format!("{}:{:?}", id, params)))
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from("../build"))
}
