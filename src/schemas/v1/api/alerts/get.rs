use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use std::sync::Arc;
use crate::DatabaseManager;

/// Get details of a specific alert including related data
#[get("/platform/<platform_id>/alerts/<id>")]
pub async fn get_alert(
    platform_id: i64,
    id: i64,
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

    let alert_data = match db::alert::get_alert_with_related_data(&pool, id).await {
        Ok(data) => data,
        Err(e) => {
            log::error!("Failed to fetch alert {}: {}", id, e);
            return Err((
                if e.to_string().contains("no rows") {
                    Status::NotFound
                } else {
                    Status::InternalServerError
                },
                Json(json!({
                    "error": if e.to_string().contains("no rows") { "Alert not found" } else { "Database error" },
                    "message": if e.to_string().contains("no rows") { 
                        format!("Alert with ID {} does not exist", id) 
                    } else { 
                        "Failed to fetch alert details".to_string() 
                    }
                }))
            ));
        }
    };

    Ok(Json(json!(alert_data)))
}