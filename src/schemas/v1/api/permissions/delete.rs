use crate::schemas::v1::db::queries::{self as db};
use rocket::{delete, State};
use sqlx::MySql;

#[delete("/permissions/<id>")]
pub async fn delete_permission(
    pool: &State<sqlx::Pool<MySql>>,
    id: i64,
) -> (rocket::http::Status, String) {
    let result = db::permission::delete_permission(pool, id);
    match result.await {
        Ok(_) => (
            rocket::http::Status::Ok,
            "Permission has been successfully deleted".to_string(),
        ),
        Err(e) => (rocket::http::Status::InternalServerError, format!("{e:#}")),
    }
}