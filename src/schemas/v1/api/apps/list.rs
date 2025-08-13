use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use std::sync::Arc;

use crate::DatabaseManager;

/// List all applications with pagination support.
///
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `page` - Required page number for pagination
/// * `per_page` - Required number of items per page
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// A JSON array of applications or an error if pagination parameters are missing
#[get("/platform/<platform_id>/apps?<page>&<per_page>")]
pub async fn list_apps(
    platform_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    // Get platform information from main database
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
            let apps = match db::app::list_apps(&pool, p, pp).await {
                Ok(apps) => apps,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve applications"
                        }))
                    ));
                }
            };
            
            let total_count = match db::app::count_apps(&pool).await {
                Ok(count) => count,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to count applications"
                        }))
                    ));
                }
            };
            
            let total_pages = (total_count / pp);

            let response = json!({
                "apps": apps,
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

/// Count the total number of applications.
#[get("/platform/<platform_id>/app-count")]
pub async fn count_apps(
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

    match db::app::count_apps(&pool).await {
        Ok(count) => Ok(Json(count)),
        Err(_) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Database error",
                "message": "Failed to count applications"
            }))
        )),
    }
}