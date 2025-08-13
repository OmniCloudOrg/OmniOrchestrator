use std::sync::Arc;
use crate::DatabaseManager;
use super::super::super::db::queries as db;
use rocket::post;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;

use libomni::types::db::v1 as types;
use types::audit_log::AuditLog;

/// Creates a new audit log entry in the system.
#[post("/platform/<platform_id>/audit_log", format = "json", data = "<audit_log>")]
pub async fn create_audit_log(
    platform_id: i64,
    audit_log: Json<AuditLog>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<AuditLog>, (Status, Json<Value>)> {
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

    match db::audit_log::create_audit_log(
        &pool,
        audit_log.user_id,
        audit_log.org_id,
        &audit_log.action,
        &audit_log.resource_type,
        audit_log.resource_id.clone(),
    ).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Database error",
                "message": "Failed to create audit log entry"
            }))
        )),
    }
}