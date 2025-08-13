use super::super::super::db::queries as db;
use super::types::{CostAnalysisByDimensionRequest, CostOverTimeRequest};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{post, State};
use std::sync::Arc;
use crate::DatabaseManager;
use chrono::{DateTime, Utc};

/// Get cost analysis by dimension (app, provider, resource_type, etc.)
#[post("/platform/<platform_id>/cost_analysis/by_dimension", format = "json", data = "<request>")]
pub async fn analyze_costs_by_dimension(
    platform_id: i64,
    request: Json<CostAnalysisByDimensionRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Vec<(String, f64)>>, (Status, Json<Value>)> {
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

    match db::cost::get_cost_metrics_by_dimension(
        &pool,
        &request.dimension,
        request.start_date,
        request.end_date,
        request.limit,
    ).await {
        Ok(results) => Ok(Json(results)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to analyze costs by dimension",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Get application cost over time
#[post("/platform/<platform_id>/cost_analysis/over_time", format = "json", data = "<request>")]
pub async fn analyze_cost_over_time(
    platform_id: i64,
    request: Json<CostOverTimeRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Vec<(DateTime<Utc>, f64)>>, (Status, Json<Value>)> {
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

    match db::cost::get_app_cost_over_time(
        &pool,
        request.app_id,
        &request.interval,
        request.start_date,
        request.end_date,
    ).await {
        Ok(results) => Ok(Json(results)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to analyze cost over time",
                "message": format!("{}", e)
            }))
        )),
    }
}