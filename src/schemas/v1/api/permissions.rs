use crate::schemas::v1::db::queries::{self as db};
use rocket::{delete, get, post, serde::json::Json, State};
use sqlx::MySql;

use libomni::types::db::v1 as types;
use types::permission::Permission;

// #[post("/app")]

// pub async fn update_permission() {

// }
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

#[get("/permissions")]
pub async fn list_permission(pool: &State<sqlx::Pool<MySql>>) -> Json<Vec<Permission>> {
    let permissions = db::permission::list_permissions(pool).await.unwrap();

    Json(permissions)
}
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
