use std::sync::Arc;
use crate::DatabaseManager;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use crate::schemas::v1::db::queries::{self as db};

use libomni::types::db::v1 as types;
use types::instance::Instance;

/// Get an instance by ID
#[get("/platform/<platform_id>/instances/<instance_id>")]
pub async fn get_instance(
    platform_id: i64,
    instance_id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Instance>, (Status, Json<Value>)> {
    // Get platform information
    let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(platform) => platform,
        Err(_) => {
            return Err((
                Status::NotFound,
                Json(json!({
                    "error": "Platform not found",
                    "message": format!("Platform with ID {} does not exist", platform_id)
                }))
            ));
        }
    };

    // Get platform-specific database pool
    let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
        Ok(pool) => pool,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to connect to platform database"
                }))
            ));
        }
    };

    match db::instance::get_instance_by_id(&pool, instance_id).await {
        Ok(instance) => Ok(Json(instance)),
        Err(_) => {
            Err((
                Status::NotFound,
                Json(json!({
                    "error": "Instance not found",
                    "message": format!("Instance with ID {} does not exist", instance_id)
                }))
            ))
        }
    }
}