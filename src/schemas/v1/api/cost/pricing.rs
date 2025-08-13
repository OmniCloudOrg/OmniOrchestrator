use super::super::super::db::queries as db;
use super::types::{CreateResourcePricingRequest, UpdateResourcePricingRequest};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put, State};
use std::sync::Arc;
use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::cost::ResourcePricing;

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