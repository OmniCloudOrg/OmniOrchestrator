use super::super::super::db::queries as db;
use super::types::CreateCostMetricRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, State};
use std::sync::Arc;
use crate::DatabaseManager;
use chrono::{DateTime, Utc};

use libomni::types::db::v1 as types;
use types::cost::{CostMetric, CostMetricWithType};

/// List cost metrics with pagination and filtering support.
#[get("/platform/<platform_id>/cost_metrics?<page>&<per_page>&<resource_type_id>&<provider_id>&<app_id>&<start_date>&<end_date>&<billing_period>")]
pub async fn list_cost_metrics(
    platform_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    resource_type_id: Option<i32>,
    provider_id: Option<i64>,
    app_id: Option<i64>,
    start_date: Option<String>,
    end_date: Option<String>,
    billing_period: Option<String>,
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

    use chrono::TimeZone;

    // Parse start_date and end_date from Option<String> to Option<DateTime<Utc>>
    let parsed_start_date = match start_date {
        Some(ref s) => match DateTime::parse_from_rfc3339(s) {
            Ok(dt) => Some(dt.with_timezone(&Utc)),
            Err(_) => None,
        },
        None => None,
    };
    let parsed_end_date = match end_date {
        Some(ref s) => match DateTime::parse_from_rfc3339(s) {
            Ok(dt) => Some(dt.with_timezone(&Utc)),
            Err(_) => None,
        },
        None => None,
    };

    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let cost_metrics = match db::cost::list_cost_metrics(
                &pool, p, pp, resource_type_id, provider_id, app_id, parsed_start_date, parsed_end_date, billing_period.as_deref()
            ).await {
                Ok(metrics) => metrics,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve cost metrics"
                        }))
                    ));
                }
            };
            
            let total_count = match db::cost::count_cost_metrics(
                &pool, resource_type_id, provider_id, app_id, parsed_start_date, parsed_end_date, billing_period.as_deref()
            ).await {
                Ok(count) => count,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to count cost metrics"
                        }))
                    ));
                }
            };
            
            let total_pages = (total_count as f64 / pp as f64).ceil() as i64;

            let response = json!({
                "cost_metrics": cost_metrics,
                "pagination": {
                    "page": p,
                    "per_page": pp,
                    "total_count": total_count,
                    "total_pages": total_pages
                }
            });

            Ok(Json(response))
        }
        _ => Err((
            Status::BadRequest,
            Json(json!({
                "error": "Missing pagination parameters",
                "message": "Please provide both 'page' and 'per_page' parameters"
            }))
        ))
    }
}

/// Get a specific cost metric by ID.
#[get("/platform/<platform_id>/cost_metrics/<id>")]
pub async fn get_cost_metric(
    platform_id: i64,
    id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<CostMetricWithType>, (Status, Json<Value>)> {
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

    match db::cost::get_cost_metric_by_id(&pool, id).await {
        Ok(cost_metric) => Ok(Json(cost_metric)),
        Err(e) => Err((
            Status::NotFound,
            Json(json!({
                "error": "Cost metric not found",
                "message": format!("Cost metric with ID {} could not be found: {}", id, e)
            }))
        )),
    }
}

/// Create a new cost metric.
#[post("/platform/<platform_id>/cost_metrics", format = "json", data = "<request>")]
pub async fn create_cost_metric(
    platform_id: i64,
    request: Json<CreateCostMetricRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<CostMetric>, (Status, Json<Value>)> {
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

    match db::cost::create_cost_metric(
        &pool,
        request.resource_type_id,
        request.provider_id,
        request.region_id,
        request.app_id,
        request.worker_id,
        request.org_id,
        request.start_time,
        request.end_time,
        request.usage_quantity,
        request.unit_cost,
        &request.currency,
        request.total_cost,
        request.discount_percentage,
        request.discount_reason.as_deref(),
        request.billing_period.as_deref(),
    ).await {
        Ok(cost_metric) => Ok(Json(cost_metric)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to create cost metric",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Delete a cost metric.
#[delete("/platform/<platform_id>/cost_metrics/<id>")]
pub async fn delete_cost_metric(
    platform_id: i64,
    id: i64,
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

    match db::cost::delete_cost_metric(&pool, id).await {
        Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to delete cost metric",
                "message": format!("{}", e)
            }))
        )),
    }
}