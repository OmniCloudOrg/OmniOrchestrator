use std::sync::Arc;
use crate::DatabaseManager;
use super::super::super::db::queries as db;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;

/// List audit log entries with pagination support.
#[get("/platform/<platform_id>/audit_logs?<page>&<per_page>")]
pub async fn list_audit_logs(
    platform_id: i64,
    page: Option<u32>,
    per_page: Option<u32>,
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

    let p: i64 = page.unwrap_or(1).into();
    let pp: i64 = per_page.unwrap_or(10).into();

    let audit_logs = match db::audit_log::list_audit_logs_paginated(&pool, pp, p).await {
        Ok(logs) => logs,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to list audit logs"
                }))
            ));
        }
    };

    let total_count = match db::audit_log::count_audit_logs(&pool).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count audit logs"
                }))
            ));
        }
    };

    let total_pages = if pp > 0 {
        (total_count + pp - 1) / pp
    } else {
        1
    };

    let response = json!({
        "audit_logs": audit_logs,
        "pagination": {
            "page": p,
            "per_page": pp,
            "total_count": total_count,
            "total_pages": total_pages
        }
    });

    Ok(Json(response))
}