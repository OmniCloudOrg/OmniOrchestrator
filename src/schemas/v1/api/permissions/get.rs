use crate::schemas::v1::db::queries::{self as db};
use rocket::{get, serde::json::Json, State};
use sqlx::MySql;

use libomni::types::db::v1 as types;
use types::permission::Permission;

#[get("/permissions/<id>")]
pub async fn get_permission_by_id(
    pool: &State<sqlx::Pool<MySql>>,
    id: i64,
) -> Json<Permission> {
    let permission = db::permission::get_permission_by_id(pool, id)
        .await
        .unwrap();

    Json(permission)
}