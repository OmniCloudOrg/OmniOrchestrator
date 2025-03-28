/// All routes related to the configuration and setup of an omni platform.
use std::fs;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::http::Status;
use rocket::post;
use rocket::response::status::Custom;

#[post("/platforms/init", data = "<data>")]
pub fn recieve_cloud_config(data: Json<String>) {
    println!("Received data: {:#?}", data);
}
