use super::super::super::db::queries as db;
use super::types::CreateCostProjectionRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, State};
use std::sync::Arc;
use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::cost::CostProjection;

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