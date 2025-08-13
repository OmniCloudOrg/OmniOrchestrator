use super::super::super::db::queries as db;
use super::types::CreateAlertRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{post, State};
use std::sync::Arc;
use crate::DatabaseManager;

/// Create a new alert
#[post("/platform/<platform_id>/alerts", format = "json", data = "<alert_data>")]
pub async fn create_alert(
    platform_id: i64,
    alert_data: Json<CreateAlertRequest>,
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

    let data = alert_data.into_inner();
    
    let alert = match db::alert::create_alert(
        &pool,
        &data.alert_type,
        &data.severity,
        &data.service,
        &data.message,
        data.metadata,
        data.org_id,
        data.app_id,
        data.instance_id,
        data.region_id,
        data.node_id,
    ).await {
        Ok(alert) => alert,
        Err(e) => {
            log::error!("Failed to create alert: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to create alert"
                }))
            ));
        }
    };

    Ok(Json(json!({
        "message": "Alert created successfully",
        "alert": alert
    })))
}