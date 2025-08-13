use super::super::super::db::queries as db;
use super::types::BulkUpdateStatusRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{put, State};
use std::sync::Arc;
use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::user::User;

/// Bulk update alert status
#[put("/platform/<platform_id>/alerts/bulk-status", format = "json", data = "<update_data>")]
pub async fn bulk_update_alert_status(
    platform_id: i64,
    update_data: Json<BulkUpdateStatusRequest>,
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

    let data = update_data.into_inner();
    
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
    
    // Validate that at least one filter is provided
    if data.ids.is_none() && data.service.is_none() && data.app_id.is_none() {
        return Err((
            Status::BadRequest,
            Json(json!({
                "error": "Missing filters",
                "message": "At least one filter (ids, service, or app_id) must be provided"
            }))
        ));
    }

    let count = match db::alert::bulk_update_alert_status(
        &pool,
        data.ids,
        data.service.as_deref(),
        data.app_id,
        &data.status,
        user.id, // Use user.id instead of user_id
        data.notes.as_deref(),
    ).await {
        Ok(count) => count,
        Err(e) => {
            log::error!("Failed to bulk update alert status: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to update alert statuses"
                }))
            ));
        }
    };

    Ok(Json(json!({
        "message": "Successfully updated alert status",
        "count": count
    })))
}