use std::sync::Arc;
use crate::DatabaseManager;
use crate::models::instance::Instance;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use crate::schemas::v1::db::queries::{self as db};

/// List all instances by `region_id` and `app_id`
#[get("/platform/<platform_id>/apps/<app_id>/instances/region/<region_id>?<page>&<per_page>")]
pub async fn list_instances_by_region(
    platform_id: i64,
    app_id: i64,
    region_id: i64,
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

    // Default to page 1 and 10 items per page
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    
    let instances = match db::instance::list_instances_by_region(&pool, region_id, app_id, page, per_page).await {
        Ok(instances) => instances,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to list instances by region"
                }))
            ));
        }
    };
    
    let instances_vec: Vec<Instance> = instances.into_iter().collect();
    
    Ok(Json(json!({
        "instances": instances_vec,
        "pagination": {
            "page": page,
            "per_page": per_page
        }
    })))
}

/// Count all instances across all applications
#[get("/platform/<platform_id>/instance-count")]
pub async fn count_instances(
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

    let count = match db::instance::count_instances(&pool).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count instances"
                }))
            ));
        }
    };
    
    Ok(Json(json!({ "count": count })))
}

/// Get an instance by ID
#[get("/platform/<platform_id>/instances/<instance_id>")]
pub async fn get_instance(
    platform_id: i64,
    instance_id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Instance>, (Status, Json<Value>)> {
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

    match db::instance::get_instance_by_id(&pool, instance_id).await {
        Ok(instance) => Ok(Json(instance)),
        Err(_) => {
            Err((
                Status::NotFound,
                Json(json!({
                    "error": "Instance not found",
                    "message": format!("Instance with ID {} does not exist", instance_id)
                }))
            ))
        }
    }
}

// Note: The commented out routes would also need similar updates if enabled