//! Cost management module for handling cost tracking and analysis operations.
//!
//! This module provides a REST API for managing cost-related entities, including:
//! - Resource types management
//! - Cost metrics tracking and analysis
//! - Cost projections and forecasting
//! - Budget management
//! - Resource pricing management
//! - Cost allocation tagging

use std::sync::Arc;
use super::super::super::auth::User;
use crate::DatabaseManager;
use crate::models::{
    util_tables::ResourceType,
    cost::{
        CostBudget,
        CostMetric,
        CostProjection,
        ResourcePricing,
        CostAllocationTag,
        CostMetricWithType,
    }
};
use super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put, State};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Request data for creating a new resource type.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResourceTypeRequest {
    /// Name of the resource type
    name: String,
    /// Category of the resource
    category: String,
    /// Unit of measurement (e.g., 'vCPU-hour', 'GB-month')
    unit_of_measurement: String,
    /// Optional description of the resource type
    description: Option<String>,
}

/// Request data for updating a resource type.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResourceTypeRequest {
    /// New name for the resource type
    name: Option<String>,
    /// New category for the resource
    category: Option<String>,
    /// New unit of measurement
    unit_of_measurement: Option<String>,
    /// New description of the resource type
    description: Option<String>,
}

/// Request data for creating a new cost metric.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCostMetricRequest {
    /// Resource type ID
    resource_type_id: i32,
    /// Provider ID
    provider_id: Option<i64>,
    /// Region ID
    region_id: Option<i64>,
    /// Application ID
    app_id: Option<i64>,
    /// Worker ID
    worker_id: Option<i64>,
    /// Organization ID
    org_id: Option<i64>,
    /// Start time of the usage period
    start_time: DateTime<Utc>,
    /// End time of the usage period
    end_time: DateTime<Utc>,
    /// Amount of resource used
    usage_quantity: f64,
    /// Cost per unit
    unit_cost: f64,
    /// Currency code (e.g., 'USD')
    currency: String,
    /// Total cost for this usage
    total_cost: f64,
    /// Discount percentage applied
    discount_percentage: Option<f64>,
    /// Reason for the discount
    discount_reason: Option<String>,
    /// Billing period (e.g., '2025-05')
    billing_period: Option<String>,
}

/// Request data for filtering cost metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct CostMetricFilter {
    /// Filter by resource type ID
    resource_type_id: Option<i32>,
    /// Filter by provider ID
    provider_id: Option<i64>,
    /// Filter by application ID
    app_id: Option<i64>,
    /// Filter by start date
    start_date: Option<DateTime<Utc>>,
    /// Filter by end date
    end_date: Option<DateTime<Utc>>,
    /// Filter by billing period
    billing_period: Option<String>,
}

/// Request data for creating a new cost budget.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCostBudgetRequest {
    /// Organization ID
    org_id: i64,
    /// Application ID (optional)
    app_id: Option<i64>,
    /// Budget name
    budget_name: String,
    /// Budget amount
    budget_amount: f64,
    /// Currency code (e.g., 'USD')
    currency: String,
    /// Budget period type
    budget_period: String,
    /// Start date of the budget period
    period_start: DateTime<Utc>,
    /// End date of the budget period
    period_end: DateTime<Utc>,
    /// Alert threshold percentage
    alert_threshold_percentage: f64,
    /// Contacts to alert when threshold is reached (JSON)
    alert_contacts: String,
}

/// Request data for updating a cost budget.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCostBudgetRequest {
    /// New budget name
    budget_name: Option<String>,
    /// New budget amount
    budget_amount: Option<f64>,
    /// New alert threshold percentage
    alert_threshold_percentage: Option<f64>,
    /// New contacts to alert when threshold is reached (JSON)
    alert_contacts: Option<String>,
    /// Whether the budget is active
    is_active: Option<bool>,
}

/// Request data for creating a new cost projection.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCostProjectionRequest {
    /// Organization ID
    org_id: i64,
    /// Application ID (optional)
    app_id: Option<i64>,
    /// Projection period type (e.g., 'monthly', 'quarterly')
    projection_period: String,
    /// Start date of the projection period
    start_date: DateTime<Utc>,
    /// End date of the projection period
    end_date: DateTime<Utc>,
    /// Projected cost amount
    projected_cost: f64,
    /// Currency code (e.g., 'USD')
    currency: String,
    /// Projection model used (e.g., 'linear', 'average_30d')
    projection_model: String,
    /// Confidence level of the projection
    confidence_level: Option<f64>,
    /// Additional metadata about the projection (JSON)
    metadata: Option<String>,
}

/// Request data for creating a new resource pricing entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResourcePricingRequest {
    /// Resource type ID
    resource_type_id: i32,
    /// Provider ID
    provider_id: i64,
    /// Region ID (optional)
    region_id: Option<i64>,
    /// Tier name (e.g., 'standard', 'premium')
    tier_name: String,
    /// Price per unit
    unit_price: f64,
    /// Currency code (e.g., 'USD')
    currency: String,
    /// When this pricing becomes effective
    effective_from: DateTime<Utc>,
    /// When this pricing expires (optional)
    effective_to: Option<DateTime<Utc>>,
    /// Pricing model (e.g., 'on-demand', 'reserved')
    pricing_model: String,
    /// Commitment period (e.g., '1-year', '3-year')
    commitment_period: Option<String>,
    /// Volume discount tiers (JSON)
    volume_discount_tiers: Option<String>,
}

/// Request data for updating a resource pricing entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResourcePricingRequest {
    /// New price per unit
    unit_price: Option<f64>,
    /// New expiration date
    effective_to: Option<DateTime<Utc>>,
    /// New volume discount tiers (JSON)
    volume_discount_tiers: Option<String>,
}

/// Request data for creating a new cost allocation tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCostAllocationTagRequest {
    /// Tag key
    tag_key: String,
    /// Tag value
    tag_value: String,
    /// Resource ID
    resource_id: i64,
    /// Resource type
    resource_type: String,
}

/// Request data for aggregate cost analysis by dimension.
#[derive(Debug, Serialize, Deserialize)]
pub struct CostAnalysisByDimensionRequest {
    /// Dimension to group by
    dimension: String,
    /// Start date for analysis
    start_date: DateTime<Utc>,
    /// End date for analysis
    end_date: DateTime<Utc>,
    /// Maximum number of results to return
    limit: i64,
}

/// Request data for cost analysis over time.
#[derive(Debug, Serialize, Deserialize)]
pub struct CostOverTimeRequest {
    /// Application ID to analyze
    app_id: i64,
    /// Time interval ('day', 'week', 'month')
    interval: String,
    /// Start date for analysis
    start_date: DateTime<Utc>,
    /// End date for analysis
    end_date: DateTime<Utc>,
}

/// List all resource types with pagination support.
#[get("/platform/<platform_id>/resource_types?<page>&<per_page>")]
pub async fn list_resource_types(
    platform_id: i64,
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

    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let resource_types = match db::cost::list_resource_types(&pool, p, pp).await {
                Ok(types) => types,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve resource types"
                        }))
                    ));
                }
            };
            
            let total_count = match db::cost::count_resource_types(&pool).await {
                Ok(count) => count,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to count resource types"
                        }))
                    ));
                }
            };
            
            let total_pages = (total_count as f64 / pp as f64).ceil() as i64;

            let response = json!({
                "resource_types": resource_types,
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

/// Count the total number of resource types.
#[get("/platform/<platform_id>/count/resource_types")]
pub async fn count_resource_types(
    platform_id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<i64>, (Status, Json<Value>)> {
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

    match db::cost::count_resource_types(&pool).await {
        Ok(count) => Ok(Json(count)),
        Err(_) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Database error",
                "message": "Failed to count resource types"
            }))
        )),
    }
}

/// Get a specific resource type by ID.
#[get("/platform/<platform_id>/resource_types/<id>")]
pub async fn get_resource_type(
    platform_id: i64,
    id: i32,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<ResourceType>, (Status, Json<Value>)> {
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

    match db::cost::get_resource_type_by_id(&pool, id).await {
        Ok(resource_type) => Ok(Json(resource_type)),
        Err(_) => Err((
            Status::NotFound,
            Json(json!({
                "error": "Resource type not found",
                "message": format!("Resource type with ID {} could not be found", id)
            }))
        )),
    }
}

/// Create a new resource type.
#[post("/platform/<platform_id>/resource_types", format = "json", data = "<request>")]
pub async fn create_resource_type(
    platform_id: i64,
    request: Json<CreateResourceTypeRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<ResourceType>, (Status, Json<Value>)> {
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

    match db::cost::create_resource_type(
        &pool,
        &request.name,
        &request.category,
        &request.unit_of_measurement,
        request.description.as_deref(),
    ).await {
        Ok(resource_type) => Ok(Json(resource_type)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to create resource type",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Update an existing resource type.
#[put("/platform/<platform_id>/resource_types/<id>", format = "json", data = "<request>")]
pub async fn update_resource_type(
    platform_id: i64,
    id: i32,
    request: Json<UpdateResourceTypeRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<ResourceType>, (Status, Json<Value>)> {
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

    match db::cost::update_resource_type(
        &pool,
        id,
        request.name.as_deref(),
        request.category.as_deref(),
        request.unit_of_measurement.as_deref(),
        request.description.as_deref(),
    ).await {
        Ok(resource_type) => Ok(Json(resource_type)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to update resource type",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Delete a resource type.
#[delete("/platform/<platform_id>/resource_types/<id>")]
pub async fn delete_resource_type(
    platform_id: i64,
    id: i32,
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

    match db::cost::delete_resource_type(&pool, id).await {
        Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to delete resource type",
                "message": format!("{}", e)
            }))
        )),
    }
}

// Cost Metrics Routes

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

// Cost Budget Routes

/// List all cost budgets with pagination support.
#[get("/platform/<platform_id>/cost_budgets?<page>&<per_page>")]
pub async fn list_cost_budgets(
    platform_id: i64,
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

    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let cost_budgets = match db::cost::list_cost_budgets(&pool, p, pp).await {
                Ok(budgets) => budgets,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve cost budgets"
                        }))
                    ));
                }
            };
            
            let total_count = match db::cost::count_cost_budgets(&pool).await {
                Ok(count) => count,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to count cost budgets"
                        }))
                    ));
                }
            };
            
            let total_pages = (total_count as f64 / pp as f64).ceil() as i64;

            let response = json!({
                "cost_budgets": cost_budgets,
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

/// Get a specific cost budget by ID.
#[get("/platform/<platform_id>/cost_budgets/<id>")]
pub async fn get_cost_budget(
    platform_id: i64,
    id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<CostBudget>, (Status, Json<Value>)> {
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

    match db::cost::get_cost_budget_by_id(&pool, id).await {
        Ok(budget) => Ok(Json(budget)),
        Err(_) => Err((
            Status::NotFound,
            Json(json!({
                "error": "Cost budget not found",
                "message": format!("Cost budget with ID {} could not be found", id)
            }))
        )),
    }
}

/// Create a new cost budget.
#[post("/platform/<platform_id>/cost_budgets", format = "json", data = "<request>")]
pub async fn create_cost_budget(
    platform_id: i64,
    request: Json<CreateCostBudgetRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
    user: User,
) -> Result<Json<CostBudget>, (Status, Json<Value>)> {
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

    let user_id = user.id;

    //TODO: Validate user permissions here later

    match db::cost::create_cost_budget(
        &pool,
        request.org_id,
        request.app_id,
        &request.budget_name,
        request.budget_amount,
        &request.currency,
        &request.budget_period,
        request.period_start,
        request.period_end,
        request.alert_threshold_percentage,
        &request.alert_contacts,
        user_id,
    ).await {
        Ok(budget) => Ok(Json(budget)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to create cost budget",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Update an existing cost budget.
#[put("/platform/<platform_id>/cost_budgets/<id>", format = "json", data = "<request>")]
pub async fn update_cost_budget(
    platform_id: i64,
    id: i64,
    request: Json<UpdateCostBudgetRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<CostBudget>, (Status, Json<Value>)> {
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

    match db::cost::update_cost_budget(
        &pool,
        id,
        request.budget_name.as_deref(),
        request.budget_amount,
        request.alert_threshold_percentage,
        request.alert_contacts.as_deref(),
        request.is_active,
    ).await {
        Ok(budget) => Ok(Json(budget)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to update cost budget",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Delete a cost budget.
#[delete("/platform/<platform_id>/cost_budgets/<id>")]
pub async fn delete_cost_budget(
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

    match db::cost::delete_cost_budget(&pool, id).await {
        Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to delete cost budget",
                "message": format!("{}", e)
            }))
        )),
    }
}

// Cost Projection Routes

/// List all cost projections with pagination support.
#[get("/platform/<platform_id>/cost_projections?<page>&<per_page>")]
pub async fn list_cost_projections(
    platform_id: i64,
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

    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let projections = match db::cost::list_cost_projections(&pool, p, pp).await {
                Ok(projections) => projections,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve cost projections"
                        }))
                    ));
                }
            };
            
            let response = json!({
                "cost_projections": projections,
                "pagination": {
                    "page": p,
                    "per_page": pp
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

/// Get a specific cost projection by ID.
#[get("/platform/<platform_id>/cost_projections/<id>")]
pub async fn get_cost_projection(
    platform_id: i64,
    id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<CostProjection>, (Status, Json<Value>)> {
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

    match db::cost::get_cost_projection_by_id(&pool, id).await {
        Ok(projection) => Ok(Json(projection)),
        Err(_) => Err((
            Status::NotFound,
            Json(json!({
                "error": "Cost projection not found",
                "message": format!("Cost projection with ID {} could not be found", id)
            }))
        )),
    }
}

/// Create a new cost projection.
#[post("/platform/<platform_id>/cost_projections", format = "json", data = "<request>")]
pub async fn create_cost_projection(
    platform_id: i64,
    request: Json<CreateCostProjectionRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<CostProjection>, (Status, Json<Value>)> {
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

    match db::cost::create_cost_projection(
        &pool,
        request.org_id,
        request.app_id,
        &request.projection_period,
        request.start_date,
        request.end_date,
        request.projected_cost,
        &request.currency,
        &request.projection_model,
        request.confidence_level,
        request.metadata.as_deref(),
    ).await {
        Ok(projection) => Ok(Json(projection)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to create cost projection",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Delete a cost projection.
#[delete("/platform/<platform_id>/cost_projections/<id>")]
pub async fn delete_cost_projection(
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

    match db::cost::delete_cost_projection(&pool, id).await {
        Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to delete cost projection",
                "message": format!("{}", e)
            }))
        )),
    }
}

// Resource Pricing Routes

/// List resource pricing with pagination and filtering support.
#[get("/platform/<platform_id>/resource_pricing?<page>&<per_page>&<resource_type_id>&<provider_id>&<region_id>&<pricing_model>&<tier_name>")]
pub async fn list_resource_pricing(
    platform_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    resource_type_id: Option<i32>,
    provider_id: Option<i64>,
    region_id: Option<i64>,
    pricing_model: Option<String>,
    tier_name: Option<String>,
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

    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let pricing = match db::cost::list_resource_pricing(
                &pool, p, pp, resource_type_id, provider_id, region_id, pricing_model.as_deref(), tier_name.as_deref()
            ).await {
                Ok(pricing) => pricing,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve resource pricing"
                        }))
                    ));
                }
            };
            
            let response = json!({
                "resource_pricing": pricing,
                "pagination": {
                    "page": p,
                    "per_page": pp
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

/// Get a specific resource pricing entry by ID.
#[get("/platform/<platform_id>/resource_pricing/<id>")]
pub async fn get_resource_pricing(
    platform_id: i64,
    id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<ResourcePricing>, (Status, Json<Value>)> {
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

    match db::cost::get_resource_pricing_by_id(&pool, id).await {
        Ok(pricing) => Ok(Json(pricing)),
        Err(_) => Err((
            Status::NotFound,
            Json(json!({
                "error": "Resource pricing not found",
                "message": format!("Resource pricing with ID {} could not be found", id)
            }))
        )),
    }
}

/// Create a new resource pricing entry.
#[post("/platform/<platform_id>/resource_pricing", format = "json", data = "<request>")]
pub async fn create_resource_pricing(
    platform_id: i64,
    request: Json<CreateResourcePricingRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<ResourcePricing>, (Status, Json<Value>)> {
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

    match db::cost::create_resource_pricing(
        &pool,
        request.resource_type_id,
        request.provider_id,
        request.region_id,
        &request.tier_name,
        request.unit_price,
        &request.currency,
        request.effective_from,
        request.effective_to,
        &request.pricing_model,
        request.commitment_period.as_deref(),
        request.volume_discount_tiers.as_deref(),
    ).await {
        Ok(pricing) => Ok(Json(pricing)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to create resource pricing",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Update an existing resource pricing entry.
#[put("/platform/<platform_id>/resource_pricing/<id>", format = "json", data = "<request>")]
pub async fn update_resource_pricing(
    platform_id: i64,
    id: i64,
    request: Json<UpdateResourcePricingRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<ResourcePricing>, (Status, Json<Value>)> {
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

    match db::cost::update_resource_pricing(
        &pool,
        id,
        request.unit_price,
        request.effective_to,
        request.volume_discount_tiers.as_deref(),
    ).await {
        Ok(pricing) => Ok(Json(pricing)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to update resource pricing",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Delete a resource pricing entry.
#[delete("/platform/<platform_id>/resource_pricing/<id>")]
pub async fn delete_resource_pricing(
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

    match db::cost::delete_resource_pricing(&pool, id).await {
        Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to delete resource pricing",
                "message": format!("{}", e)
            }))
        )),
    }
}

// Cost Allocation Tag Routes

/// Get cost allocation tags for a specific resource.
#[get("/platform/<platform_id>/cost_allocation_tags/<resource_id>/<resource_type>")]
pub async fn get_cost_allocation_tags(
    platform_id: i64,
    resource_id: i64,
    resource_type: String,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Vec<CostAllocationTag>>, (Status, Json<Value>)> {
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

    match db::cost::get_cost_allocation_tags(&pool, resource_id, &resource_type).await {
        Ok(tags) => Ok(Json(tags)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to retrieve cost allocation tags",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Create a new cost allocation tag.
#[post("/platform/<platform_id>/cost_allocation_tags", format = "json", data = "<request>")]
pub async fn create_cost_allocation_tag(
    platform_id: i64,
    request: Json<CreateCostAllocationTagRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<CostAllocationTag>, (Status, Json<Value>)> {
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

    match db::cost::create_cost_allocation_tag(
        &pool,
        &request.tag_key,
        &request.tag_value,
        request.resource_id,
        &request.resource_type,
    ).await {
        Ok(tag) => Ok(Json(tag)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to create cost allocation tag",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Delete a cost allocation tag.
#[delete("/platform/<platform_id>/cost_allocation_tags/<id>")]
pub async fn delete_cost_allocation_tag(
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

    match db::cost::delete_cost_allocation_tag(&pool, id).await {
        Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to delete cost allocation tag",
                "message": format!("{}", e)
            }))
        )),
    }
}