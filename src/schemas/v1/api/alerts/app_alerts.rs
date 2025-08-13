use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use std::sync::Arc;
use crate::DatabaseManager;

/// Get alerts for a specific application
#[get("/platform/<platform_id>/apps/<app_id>/alerts?<limit>&<include_resolved>")]
pub async fn get_app_alerts(
    platform_id: i64,
    app_id: i64,
    limit: Option<i64>,
    include_resolved: Option<bool>,
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

    let limit = limit.unwrap_or(20);
    let include_resolved = include_resolved.unwrap_or(false);
    
    let alerts = match db::alert::get_recent_app_alerts(
        &pool,
        app_id,
        limit,
        include_resolved,
    ).await {
        Ok(alerts) => alerts,
        Err(e) => {
            log::error!("Failed to fetch app alerts: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch application alerts"
                }))
            ));
        }
    };

    Ok(Json(json!({ "alerts": alerts })))
}