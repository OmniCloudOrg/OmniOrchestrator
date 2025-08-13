use super::super::super::db::queries as db;
use super::types::CreateCostAllocationTagRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, State};
use std::sync::Arc;
use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::cost::CostAllocationTag;

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