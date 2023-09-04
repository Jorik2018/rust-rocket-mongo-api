mod api; 
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create,get_list,create_user, get_user, update_user, delete_user, get_all_users};
use repository::mongodb_repo::MongoRepo;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
    //.attach(CORS)
    .manage(db)
    .mount("/", routes![create])
    .mount("/", routes![get_list])
    .mount("/", routes![create_user])
    .mount("/", routes![get_user])
    .mount("/", routes![update_user])
    .mount("/", routes![delete_user])
    .mount("/", routes![get_all_users])
}



/*
use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}*/