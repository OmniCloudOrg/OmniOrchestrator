use super::super::super::db::queries as db;
use super::types::{CreateResourceTypeRequest, UpdateResourceTypeRequest};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put, State};
use std::sync::Arc;
use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::util_tables::ResourceType;

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