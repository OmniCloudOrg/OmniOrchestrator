use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, State};
use std::sync::Arc;

use crate::DatabaseManager;

/// Delete a specific application.
///
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_id` - The ID of the application to delete
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// A JSON response indicating success or an error message
#[delete("/platform/<platform_id>/apps/<app_id>")]
pub async fn delete_app(
    platform_id: i64,
    app_id: String,
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

    match app_id.parse::<i64>() {
        Ok(id) => {
            match db::app::delete_app(&pool, id).await {
                Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
                Err(_) => {
                    Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to delete application"
                        }))
                    ))
                }
            }
        }
        Err(e) => {
            Err((
                Status::BadRequest,
                Json(json!({
                    "error": "Invalid ID format",
                    "message": format!("The application ID must be a valid integer: {}", e)
                }))
            ))
        }
    }
}