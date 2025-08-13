use super::super::super::db::queries as db;
use super::types::UpdateAlertStatusRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{put, State};
use std::sync::Arc;
use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::user::User;

/// Update an alert's status
#[put("/platform/<platform_id>/alerts/<id>/status", format = "json", data = "<status_data>")]
pub async fn update_alert_status(
    platform_id: i64,
    id: i64,
    status_data: Json<UpdateAlertStatusRequest>,
    user: User, // Extract user from request guard
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

    let data = status_data.into_inner();
    
    // Validate the status is a valid value
    match data.status.as_str() {
        "active" | "acknowledged" | "resolved" | "auto_resolved" => {},
        _ => return Err((
            Status::BadRequest,
            Json(json!({
                "error": "Invalid status",
                "message": "Status must be one of: active, acknowledged, resolved, auto_resolved"
            }))
        ))
    }
    
    let user_id = user.id;

    let updated_alert = match db::alert::update_alert_status(
        &pool,
        id,
        &data.status,
        Some(user_id),
        data.notes.as_deref(),
    ).await {
        Ok(alert) => alert,
        Err(e) => {
            log::error!("Failed to update alert status: {}", e);
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
                        "Failed to update alert status".to_string() 
                    }
                }))
            ));
        }
    };

    Ok(Json(json!({
        "message": "Alert status updated successfully",
        "alert": updated_alert
    })))
}