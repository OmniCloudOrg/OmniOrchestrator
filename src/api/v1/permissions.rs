use rocket::{post,get,State,serde::json::Json};
use sqlx::MySql;
use crate::db::v1::queries as db;
use crate::db::v1::tables as tables;
// #[post("/app")]

// pub async fn update_permission() {

// }
#[get("/permissions/<id>")]
pub async fn get_permission_by_id(
    pool: &State<sqlx::Pool<MySql>>,
    id: i64
) -> Json<tables::Permission> {
    let permission = db::permission::get_permission_by_id(pool, id).await.unwrap();

    Json(permission)
}


#[get("/permissions")]
pub async fn list_permission(
    pool: &State<sqlx::Pool<MySql>>,
) -> Json<Vec<tables::Permission>> {
    let permissions = db::permission::list_permissions(pool).await.unwrap();

    Json(permissions)
}
#[post("/permissions/create", format = "json", data = "<permission>")]
pub async fn create_permission(
    pool: &State<sqlx::Pool<MySql>>,
    permission: Json<tables::Permission>,
    
) -> Json<tables::Permission> {
    let permission = db::permission::create_permission(pool, &permission.name, permission.description.clone(), permission.resource_type.clone().unwrap()).await.unwrap();
    Json(permission)

}