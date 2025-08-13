use std::sync::Arc;
use crate::DatabaseManager;
use crate::schemas::v1::db::queries::{self as db};
use rocket::{get, http::Status, serde::json::{json, Json, Value}, State};

use libomni::types::db::v1 as types;
use types::metrics::Metric;

#[get("/platform/<platform_id>/metrics/<instance_id>")]
pub async fn get_metrics_by_app_id(
    platform_id: i64,
    instance_id: Option<i64>,
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

    let instance_id = instance_id.or(Some(0)); // Set to 0 (or null equivalent) if blank
    
    match db::metrics::get_metrics_by_app_id(&pool, instance_id).await {
        Ok(metrics) => Ok(Json(json!({ "metrics": metrics }))),
        Err(_) => {
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to retrieve metrics"
                }))
            ))
        }
    }
}

#[get("/platform/<platform_id>/metrics")]
pub async fn get_metrics(
    platform_id: i64,
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
    
    match db::metrics::get_metrics_by_app_id(&pool, None).await {
        Ok(metrics) => Ok(Json(json!({ "metrics": metrics }))),
        Err(_) => {
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to retrieve metrics"
                }))
            ))
        }
    }
}