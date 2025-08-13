use crate::schemas::v1::db::queries::{self as db};
use rocket::{get, serde::json::Json, State};
use sqlx::MySql;

use libomni::types::db::v1 as types;
use types::permission::Permission;

#[get("/permissions")]
pub async fn list_permission(pool: &State<sqlx::Pool<MySql>>) -> Json<Vec<Permission>> {
    let permissions = db::permission::list_permissions(pool).await.unwrap();

    Json(permissions)
}