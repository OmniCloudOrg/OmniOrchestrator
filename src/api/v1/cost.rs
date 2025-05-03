//! Cost management module for handling cost tracking and analysis operations.
//!
//! This module provides a REST API for managing cost-related entities, including:
//! - Resource types management
//! - Cost metrics tracking and analysis
//! - Cost projections and forecasting
//! - Budget management
//! - Resource pricing management
//! - Cost allocation tagging

use crate::api::auth::User;
use crate::db::tables::{ResourceType, CostMetric, CostMetricWithType, ResourcePricing, CostBudget, CostProjection, CostAllocationTag};
use crate::db::v1::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put, State};
use serde::{Deserialize, Serialize};
use sqlx::MySql;
use chrono::{DateTime, Utc};

// Types

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

// Resource Type Routes

/// List all resource types with pagination support.
#[get("/resource_types?<page>&<per_page>")]
pub async fn list_resource_types(
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let resource_types = db::cost::list_resource_types(pool, p, pp).await.unwrap();
            let total_count = db::cost::count_resource_types(pool).await.unwrap();
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
#[get("/resource_types/count")]
pub async fn count_resource_types(pool: &State<sqlx::Pool<MySql>>) -> Json<i64> {
    let count = db::cost::count_resource_types(pool).await.unwrap();
    Json(count)
}

/// Get a specific resource type by ID.
#[get("/resource_types/<id>")]
pub async fn get_resource_type(id: i32, pool: &State<sqlx::Pool<MySql>>) -> Option<Json<ResourceType>> {
    let result = db::cost::get_resource_type_by_id(pool, id).await;
    match result {
        Ok(resource_type) => Some(Json(resource_type)),
        Err(_) => {
            println!("Client requested resource type: {} but it could not be found", id);
            None
        }
    }
}

/// Create a new resource type.
#[post("/resource_types", format = "json", data = "<request>")]
pub async fn create_resource_type(
    request: Json<CreateResourceTypeRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<ResourceType>, (Status, Json<Value>)> {
    match db::cost::create_resource_type(
        pool,
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
#[put("/resource_types/<id>", format = "json", data = "<request>")]
pub async fn update_resource_type(
    id: i32,
    request: Json<UpdateResourceTypeRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<ResourceType>, (Status, Json<Value>)> {
    match db::cost::update_resource_type(
        pool,
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
#[delete("/resource_types/<id>")]
pub async fn delete_resource_type(
    id: i32,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match db::cost::delete_resource_type(pool, id).await {
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
#[get("/cost_metrics?<page>&<per_page>&<resource_type_id>&<provider_id>&<app_id>&<start_date>&<end_date>&<billing_period>")]
pub async fn list_cost_metrics(
    page: Option<i64>,
    per_page: Option<i64>,
    resource_type_id: Option<i32>,
    provider_id: Option<i64>,
    app_id: Option<i64>,
    start_date: Option<String>,
    end_date: Option<String>,
    billing_period: Option<String>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
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
            let cost_metrics = db::cost::list_cost_metrics(
                pool, p, pp, resource_type_id, provider_id, app_id, parsed_start_date, parsed_end_date, billing_period.as_deref()
            ).await.unwrap();
            
            let total_count = db::cost::count_cost_metrics(
                pool, resource_type_id, provider_id, app_id, parsed_start_date, parsed_end_date, billing_period.as_deref()
            ).await.unwrap();
            
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
#[get("/cost_metrics/<id>")]
pub async fn get_cost_metric(id: i64, pool: &State<sqlx::Pool<MySql>>) -> Option<Json<CostMetricWithType>> {
    let result = db::cost::get_cost_metric_by_id(pool, id).await;
    match result {
        Ok(cost_metric) => Some(Json(cost_metric)),
        Err(_) => {
            println!("Client requested cost metric: {} but it could not be found", id);
            None
        }
    }
}

/// Create a new cost metric.
#[post("/cost_metrics", format = "json", data = "<request>")]
pub async fn create_cost_metric(
    request: Json<CreateCostMetricRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<CostMetric>, (Status, Json<Value>)> {
    match db::cost::create_cost_metric(
        pool,
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
#[delete("/cost_metrics/<id>")]
pub async fn delete_cost_metric(
    id: i64,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match db::cost::delete_cost_metric(pool, id).await {
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
#[post("/cost_analysis/by_dimension", format = "json", data = "<request>")]
pub async fn analyze_costs_by_dimension(
    request: Json<CostAnalysisByDimensionRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Vec<(String, f64)>>, (Status, Json<Value>)> {
    match db::cost::get_cost_metrics_by_dimension(
        pool,
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
#[post("/cost_analysis/over_time", format = "json", data = "<request>")]
pub async fn analyze_cost_over_time(
    request: Json<CostOverTimeRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Vec<(DateTime<Utc>, f64)>>, (Status, Json<Value>)> {
    match db::cost::get_app_cost_over_time(
        pool,
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
#[get("/cost_budgets?<page>&<per_page>")]
pub async fn list_cost_budgets(
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let cost_budgets = db::cost::list_cost_budgets(pool, p, pp).await.unwrap();
            let total_count = db::cost::count_cost_budgets(pool).await.unwrap();
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
#[get("/cost_budgets/<id>")]
pub async fn get_cost_budget(id: i64, pool: &State<sqlx::Pool<MySql>>) -> Option<Json<CostBudget>> {
    let result = db::cost::get_cost_budget_by_id(pool, id).await;
    match result {
        Ok(budget) => Some(Json(budget)),
        Err(_) => {
            println!("Client requested cost budget: {} but it could not be found", id);
            None
        }
    }
}

/// Create a new cost budget.
#[post("/cost_budgets", format = "json", data = "<request>")]
pub async fn create_cost_budget(
    request: Json<CreateCostBudgetRequest>,
    pool: &State<sqlx::Pool<MySql>>,
    user: User,
) -> Result<Json<CostBudget>, (Status, Json<Value>)> {
    let user_id = user.id;

    //TODO: Validate user permissions here later

    match db::cost::create_cost_budget(
        pool,
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
#[put("/cost_budgets/<id>", format = "json", data = "<request>")]
pub async fn update_cost_budget(
    id: i64,
    request: Json<UpdateCostBudgetRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<CostBudget>, (Status, Json<Value>)> {
    match db::cost::update_cost_budget(
        pool,
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
#[delete("/cost_budgets/<id>")]
pub async fn delete_cost_budget(
    id: i64,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match db::cost::delete_cost_budget(pool, id).await {
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
#[get("/cost_projections?<page>&<per_page>")]
pub async fn list_cost_projections(
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let projections = db::cost::list_cost_projections(pool, p, pp).await.unwrap();
            
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
#[get("/cost_projections/<id>")]
pub async fn get_cost_projection(id: i64, pool: &State<sqlx::Pool<MySql>>) -> Option<Json<CostProjection>> {
    let result = db::cost::get_cost_projection_by_id(pool, id).await;
    match result {
        Ok(projection) => Some(Json(projection)),
        Err(_) => {
            println!("Client requested cost projection: {} but it could not be found", id);
            None
        }
    }
}

/// Create a new cost projection.
#[post("/cost_projections", format = "json", data = "<request>")]
pub async fn create_cost_projection(
    request: Json<CreateCostProjectionRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<CostProjection>, (Status, Json<Value>)> {
    match db::cost::create_cost_projection(
        pool,
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
#[delete("/cost_projections/<id>")]
pub async fn delete_cost_projection(
    id: i64,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match db::cost::delete_cost_projection(pool, id).await {
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
#[get("/resource_pricing?<page>&<per_page>&<resource_type_id>&<provider_id>&<region_id>&<pricing_model>&<tier_name>")]
pub async fn list_resource_pricing(
    page: Option<i64>,
    per_page: Option<i64>,
    resource_type_id: Option<i32>,
    provider_id: Option<i64>,
    region_id: Option<i64>,
    pricing_model: Option<String>,
    tier_name: Option<String>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let pricing = db::cost::list_resource_pricing(
                pool, p, pp, resource_type_id, provider_id, region_id, pricing_model.as_deref(), tier_name.as_deref()
            ).await.unwrap();
            
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
#[get("/resource_pricing/<id>")]
pub async fn get_resource_pricing(id: i64, pool: &State<sqlx::Pool<MySql>>) -> Option<Json<ResourcePricing>> {
    let result = db::cost::get_resource_pricing_by_id(pool, id).await;
    match result {
        Ok(pricing) => Some(Json(pricing)),
        Err(_) => {
            println!("Client requested resource pricing: {} but it could not be found", id);
            None
        }
    }
}

/// Create a new resource pricing entry.
#[post("/resource_pricing", format = "json", data = "<request>")]
pub async fn create_resource_pricing(
    request: Json<CreateResourcePricingRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<ResourcePricing>, (Status, Json<Value>)> {
    match db::cost::create_resource_pricing(
        pool,
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
#[put("/resource_pricing/<id>", format = "json", data = "<request>")]
pub async fn update_resource_pricing(
    id: i64,
    request: Json<UpdateResourcePricingRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<ResourcePricing>, (Status, Json<Value>)> {
    match db::cost::update_resource_pricing(
        pool,
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
#[delete("/resource_pricing/<id>")]
pub async fn delete_resource_pricing(
    id: i64,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match db::cost::delete_resource_pricing(pool, id).await {
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
#[get("/cost_allocation_tags/<resource_id>/<resource_type>")]
pub async fn get_cost_allocation_tags(
    resource_id: i64,
    resource_type: String,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Vec<CostAllocationTag>>, (Status, Json<Value>)> {
    match db::cost::get_cost_allocation_tags(pool, resource_id, &resource_type).await {
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
#[post("/cost_allocation_tags", format = "json", data = "<request>")]
pub async fn create_cost_allocation_tag(
    request: Json<CreateCostAllocationTagRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<CostAllocationTag>, (Status, Json<Value>)> {
    match db::cost::create_cost_allocation_tag(
        pool,
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
#[delete("/cost_allocation_tags/<id>")]
pub async fn delete_cost_allocation_tag(
    id: i64,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match db::cost::delete_cost_allocation_tag(pool, id).await {
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