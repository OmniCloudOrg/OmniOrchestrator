use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use std::sync::Arc;
use crate::DatabaseManager;

/// Get alerts needing escalation
#[get("/platform/<platform_id>/alerts/needing-escalation?<org_id>&<hours_threshold>")]
pub async fn get_alerts_needing_escalation(
    platform_id: i64,
    org_id: Option<i64>,
    hours_threshold: Option<i64>,
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

    let hours_threshold = hours_threshold.unwrap_or(4); // Default to 4 hours
    
    let alerts = match db::alert::get_alerts_needing_escalation(
        &pool,
        org_id,
        hours_threshold,
    ).await {
        Ok(alerts) => alerts,
        Err(e) => {
            log::error!("Failed to fetch alerts needing escalation: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch alerts needing escalation"
                }))
            ));
        }
    };

    Ok(Json(json!({ "alerts": alerts })))
}