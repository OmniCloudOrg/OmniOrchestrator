use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use std::sync::Arc;
use crate::DatabaseManager;

/// Search for alerts
#[get("/platform/<platform_id>/alerts/search?<query>&<org_id>&<page>&<per_page>")]
pub async fn search_alerts(
    platform_id: i64,
    query: String,
    org_id: Option<i64>,
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

    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(20);
    
    let alerts = match db::alert::search_alerts(
        &pool,
        &query,
        org_id,
        page,
        per_page,
    ).await {
        Ok(alerts) => alerts,
        Err(e) => {
            log::error!("Failed to search alerts: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to search alerts"
                }))
            ));
        }
    };
    
    let total_count = match db::alert::count_search_alerts(
        &pool,
        &query,
        org_id,
    ).await {
        Ok(count) => count,
        Err(e) => {
            log::error!("Failed to count search results: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count search results"
                }))
            ));
        }
    };
    
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    
    Ok(Json(json!({
        "alerts": alerts,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    })))
}