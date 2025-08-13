use std::sync::Arc;
use crate::DatabaseManager;
use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, State};

/// Delete a specific deployment.
#[delete("/platform/<platform_id>/deployments/<deployment_id>")]
pub async fn delete_deployment(
    platform_id: i64,
    deployment_id: i64,
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

    match db::deployment::delete_deployment(&pool, deployment_id).await {
        Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
        Err(e) => Err((
            Status::InternalServerError, 
            Json(json!({
                "error": "Database error",
                "message": e.to_string()
            }))
        )),
    }
}