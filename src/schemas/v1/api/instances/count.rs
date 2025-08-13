use std::sync::Arc;
use crate::DatabaseManager;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use crate::schemas::v1::db::queries::{self as db};

/// Count all instances across all applications
#[get("/platform/<platform_id>/instance-count")]
pub async fn count_instances(
    platform_id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
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

    let count = match db::instance::count_instances(&pool).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count instances"
                }))
            ));
        }
    };
    
    Ok(Json(json!({ "count": count })))
}