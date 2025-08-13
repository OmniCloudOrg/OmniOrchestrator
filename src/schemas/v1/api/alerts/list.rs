use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use std::sync::Arc;
use crate::DatabaseManager;

/// Get a paginated list of alerts with filtering options
#[get("/platform/<platform_id>/alerts?<page>&<per_page>&<status>&<severity>&<org_id>&<app_id>&<service>&<from_date>&<to_date>")]
pub async fn list_alerts(
    platform_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    status: Option<String>,
    severity: Option<String>,
    org_id: Option<i64>,
    app_id: Option<i64>,
    service: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
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

    // Set default pagination if not provided
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(20);
    
    // Convert Optional String to Optional &str
    let status_ref = status.as_deref();
    let severity_ref = severity.as_deref();
    let service_ref = service.as_deref();

    // Fetch alerts with filters
    let alerts = match db::alert::list_alerts(
        &pool, 
        page, 
        per_page,
        status_ref,
        severity_ref,
        org_id,
        app_id,
        service_ref,
        from_date.and_then(|date_str| chrono::DateTime::parse_from_rfc3339(&date_str).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
        to_date.and_then(|date_str| chrono::DateTime::parse_from_rfc3339(&date_str).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
    ).await {
        Ok(alerts) => alerts,
        Err(e) => {
            log::error!("Failed to fetch alerts: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch alerts"
                }))
            ));
        }
    };

    // Count total alerts with same filters for pagination
    let total_count = match db::alert::count_alerts(
        &pool,
        status_ref,
        severity_ref,
        org_id,
        app_id,
    ).await {
        Ok(count) => count,
        Err(e) => {
            log::error!("Failed to fetch alert count: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count alerts"
                }))
            ));
        }
    };

    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    let response = json!({
        "alerts": alerts,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    });

    Ok(Json(response))
}