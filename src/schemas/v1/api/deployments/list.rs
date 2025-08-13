use std::sync::Arc;
use crate::DatabaseManager;
use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};

/// List all deployments with pagination support.
#[get("/platform/<platform_id>/deployments?<page>&<per_page>")]
pub async fn list_deployments(
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
            let deployments = match db::deployment::list_deployments(&pool, p, pp).await {
                Ok(deployments) => deployments,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve deployments"
                        }))
                    ));
                }
            };
            
            let total_count = match db::deployment::count_deployments(&pool).await {
                Ok(count) => count,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to count deployments"
                        }))
                    ));
                }
            };
            
            let total_pages = (total_count as f64 / pp as f64).ceil() as i64;

            let response = json!({
                "deployments": deployments,
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

/// Count the total number of deployments.
#[get("/platform/<platform_id>/count/deployments")]
pub async fn count_deployments(
    platform_id: i64,
    db_manager: &State<Arc<DatabaseManager>>
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

    match db::deployment::count_deployments(&pool).await {
        Ok(count) => Ok(Json(count)),
        Err(_) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Database error",
                "message": "Failed to count deployments"
            }))
        )),
    }
}

/// List all deployments for a specific application with pagination.
#[get("/platform/<platform_id>/apps/<app_id>/deployments?<page>&<per_page>")]
pub async fn list_app_deployments(
    platform_id: i64,
    app_id: i64,
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
            let deployments = match db::deployment::list_deployments_by_app(&pool, app_id, p, pp).await {
                Ok(deployments) => deployments,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve deployments"
                        }))
                    ));
                }
            };
            
            let total_count = match db::deployment::count_deployments_by_app(&pool, app_id).await {
                Ok(count) => count,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to count deployments"
                        }))
                    ));
                }
            };
            
            let total_pages = (total_count as f64 / pp as f64).ceil() as i64;

            let response = json!({
                "deployments": deployments,
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