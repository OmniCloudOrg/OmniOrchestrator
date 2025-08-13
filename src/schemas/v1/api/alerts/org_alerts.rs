use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use std::sync::Arc;
use crate::DatabaseManager;

/// Get active alerts for an organization
#[get("/platform/<platform_id>/orgs/<org_id>/active-alerts?<limit>")]
pub async fn get_org_active_alerts(
    platform_id: i64,
    org_id: i64,
    limit: Option<i64>,
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
    
    let alerts = match db::alert::get_org_active_alerts(
        &pool,
        org_id,
        limit,
    ).await {
        Ok(alerts) => alerts,
        Err(e) => {
            log::error!("Failed to fetch org active alerts: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch organization alerts"
                }))
            ));
        }
    };

    Ok(Json(json!({ "alerts": alerts })))
}

/// Get alert statistics for an organization
#[get("/platform/<platform_id>/orgs/<org_id>/alert-stats?<days>")]
pub async fn get_org_alert_stats(
    platform_id: i64,
    org_id: i64,
    days: Option<i64>,
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

    let days = days.unwrap_or(30); // Default to last 30 days
    
    let stats = match db::alert::get_alert_stats(
        &pool,
        org_id,
        days,
    ).await {
        Ok(stats) => stats,
        Err(e) => {
            log::error!("Failed to fetch alert stats: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch alert statistics"
                }))
            ));
        }
    };

    Ok(Json(stats))
}