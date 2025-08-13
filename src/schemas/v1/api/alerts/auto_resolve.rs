use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{post, State};
use std::sync::Arc;
use crate::DatabaseManager;

/// Auto-resolve old alerts
#[post("/platform/<platform_id>/alerts/auto-resolve?<days_threshold>&<severity_level>")]
pub async fn auto_resolve_old_alerts(
    platform_id: i64,
    days_threshold: Option<i64>,
    severity_level: Option<Vec<String>>, // Can provide multiple severity levels
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

    let days_threshold = days_threshold.unwrap_or(7); // Default to 7 days
    
    // Convert Vec<String> to Vec<&str>
    let severity_refs: Option<Vec<&str>> = severity_level
        .as_ref()
        .map(|levels| levels.iter().map(AsRef::as_ref).collect());
    
    let count = match db::alert::auto_resolve_old_alerts(
        &pool,
        days_threshold,
        severity_refs,
    ).await {
        Ok(count) => count,
        Err(e) => {
            log::error!("Failed to auto-resolve old alerts: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to auto-resolve old alerts"
                }))
            ));
        }
    };

    Ok(Json(json!({
        "message": "Successfully auto-resolved old alerts",
        "count": count
    })))
}