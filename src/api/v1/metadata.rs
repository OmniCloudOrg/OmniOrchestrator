use rocket::{get, post, State};
use sqlx::MySql;
use crate::db::v1::queries as db;

#[get("/meta/<key>")]
pub async fn get_meta_value(
    pool: &State<sqlx::Pool<MySql>>,
    key: String
) -> (rocket::http::Status,String) {
    let result = db::metadata::get_meta_value(pool,&key).await;
    match result {
        Ok(value) => (rocket::http::Status::Ok,value),
        Err(e) => (rocket::http::Status::InternalServerError,format!("{e:#}")),
    }
}

#[post("/meta/<key>", format = "json", data = "<value>")]
pub async fn set_meta_value(
    pool: &State<sqlx::Pool<MySql>>,
    key: String,
    value: String
) -> (rocket::http::Status,String) {
    let result = db::metadata::set_meta_value(&**pool,&key,&value).await;
    match result {
        Ok(_) => (rocket::http::Status::Ok,"Meta value has been successfully set".to_string()),
        Err(e) => (rocket::http::Status::InternalServerError,format!("{e:#}")),
    }
}