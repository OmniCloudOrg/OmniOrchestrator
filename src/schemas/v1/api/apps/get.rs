use super::super::super::db::queries as db;
use super::types::AppStats;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use std::sync::Arc;

use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::app::{App, AppWithInstances};

/// Get app with instances
#[get("/platform/<platform_id>/app_with_instances/<app_id>")]
pub async fn get_app_with_instances(
    db_manager: &State<Arc<DatabaseManager>>, 
    platform_id: i64,
    app_id: i64
) -> Result<Json<AppWithInstances>, (Status, Json<Value>)> {
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

    match db::app::get_app_with_instances(&pool, app_id).await {
        Ok(app_with_instances) => {
            Ok(Json(app_with_instances))
        }
        Err(_) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to fetch app with instances",
                "message": "An error occurred while retrieving the application data"
            })),
        )),
    }
}

/// Get a specific application by ID.
///
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_id` - The ID of the application to retrieve
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// The application if found, or None if not found
#[get("/platform/<platform_id>/apps/<app_id>")]
pub async fn get_app(
    platform_id: i64,
    app_id: i64, 
    db_manager: &State<Arc<DatabaseManager>>
) -> Result<Json<App>, (Status, Json<Value>)> {
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

    match db::app::get_app_by_id(&pool, app_id).await {
        Ok(app) => Ok(Json(app)),
        Err(_) => {
            Err((
                Status::NotFound,
                Json(json!({
                    "error": "App not found",
                    "message": format!("App with ID {} could not be found", app_id)
                }))
            ))
        }
    }
}

/// Get statistics for a specific application.
///
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_id` - The ID of the application to get statistics for
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// Statistics for the application
#[get("/platform/<platform_id>/apps/<app_id>/stats")]
pub async fn get_app_stats(
    platform_id: i64,
    app_id: String, 
    db_manager: &State<Arc<DatabaseManager>>
) -> Result<Json<AppStats>, (Status, Json<Value>)> {
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

    // Get platform-specific database pool (we'll need this for future implementations)
    let _pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
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

    // For now, return placeholder stats as in the original implementation
    let app_stats = AppStats {
        cpu_usage: 0.0,
        memory_usage: 0,
        disk_usage: 0,
        requests_per_second: 0.0,
        response_time_ms: 0,
    };
    Ok(Json(app_stats))
}