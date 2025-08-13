use crate::schemas::v1::db::queries::{self as db};
use rocket::{post, serde::json::Json, State};
use sqlx::MySql;

use libomni::types::db::v1 as types;
use types::permission::Permission;

#[post("/permissions", format = "json", data = "<permission>")]
pub async fn create_permission(
    pool: &State<sqlx::Pool<MySql>>,
    permission: Json<Permission>,
) -> Json<Permission> {
    let permission = db::permission::create_permission(
        pool,
        &permission.name,
        permission.description.clone(),
        permission.resource_type.clone().unwrap(),
    )
    .await
    .unwrap();
    Json(permission)
}