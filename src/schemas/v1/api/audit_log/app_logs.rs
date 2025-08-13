use std::sync::Arc;
use crate::DatabaseManager;
use super::super::super::db::queries as db;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;

/// List all audit log entries for a given app_id with pagination support.
#[get("/platform/<platform_id>/audit_logs/<app_id>?<page>&<per_page>")]
pub async fn list_audit_logs_for_app(
    platform_id: i64,
    app_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
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

    let page: i64 = page.unwrap_or(1);
    let per_page: i64 = per_page.unwrap_or(10);

    let audit_logs = match db::audit_log::get_audit_logs_by_app(&pool, app_id, page, per_page).await {
        Ok(logs) => logs,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to retrieve audit logs for app"
                }))
            ));
        }
    };

    let total_count = match db::audit_log::count_audit_logs_by_app(&pool, app_id).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count audit logs for app"
                }))
            ));
        }
    };

    let total_pages = if per_page > 0 {
        (total_count + per_page - 1) / per_page
    } else {
        1
    };

    let response = json!({
        "audit_logs": audit_logs,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    });

    Ok(Json(response))
}