//! Application management module for handling CRUD operations on applications.
//!
//! This module provides a REST API for managing applications, including:
//! - Listing applications
//! - Creating new applications
//! - Updating existing applications
//! - Getting application details and statistics
//! - Starting and stopping applications
//! - Scaling applications
//! - Deleting applications
//! - Releasing new versions of applications

use super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, http::ContentType, post, put, Data, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Import DatabaseManager
use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::app::{App, AppWithInstances};

// TODO: @tristanpoland Review if we actually need this or should drop in favor of using a central struct. Regardless we will need to move these to the modals module and eventually to LibOmni.

/// Represents an application in the system.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    /// Unique identifier for the application
    id: String,
    /// Name of the application
    name: String,
    /// Owner of the application
    owner: String,
    /// Number of running instances
    instances: i64,
    /// Memory allocation in MB
    memory: i64,
    /// Current status of the application
    status: String,
    /// Creation timestamp
    created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    updated_at: chrono::DateTime<chrono::Utc>,
}

/// Request data for scaling an application.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleRequest {
    /// Number of instances to scale to
    instances: i32,
    /// Memory allocation in MB to scale to
    memory: i32,
}

/// Statistics for an application's resource usage and performance.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppStats {
    /// CPU usage as a percentage
    cpu_usage: f64,
    /// Memory usage in bytes
    memory_usage: i64,
    /// Disk usage in bytes
    disk_usage: i64,
    /// Average number of requests per second
    requests_per_second: f64,
    /// Average response time in milliseconds
    response_time_ms: i64,
}

/// Request data for creating a new application.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppRequest {
    /// Name of the application
    name: String,
    /// Memory allocation in MB
    memory: i64,
    /// Number of instances
    instances: i64,
    /// Organization ID that owns the application
    org_id: i64,
}

/// Request data for updating an existing application.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppRequest {
    /// New name for the application
    name: String,
    /// New memory allocation in MB
    memory: i64,
    /// New number of instances
    instances: i64,
    /// Organization ID that owns the application
    org_id: i64,
}

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

/// Create a new application.
///
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_request` - JSON data containing application details
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// The newly created application
#[post("/platform/<platform_id>/apps", format = "json", data = "<app_request>")]
pub async fn create_app(
    platform_id: i64,
    app_request: Json<CreateAppRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
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

    match db::app::create_app(
        &pool,
        &app_request.name,
        app_request.org_id,
        None,
        None,
        None,
        None,
    ).await {
        Ok(app) => Ok(Json(app)),
        Err(_) => {
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to create application"
                }))
            ))
        }
    }
}

/// Update an existing application.
///
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_request` - JSON data containing updated application details
/// * `db_manager` - Database manager for accessing platform-specific pools
/// * `app_id` - The ID of the application to update
///
/// # Returns
///
/// The updated application
#[post("/platform/<platform_id>/apps/<app_id>", format = "json", data = "<app_request>")]
pub async fn update_app(
    platform_id: i64,
    app_request: Json<UpdateAppRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
    app_id: i64,
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

    match db::app::update_app(
        &pool,
        app_id,
        Some(&app_request.name),
        None,
        None,
        None,
        None,
        None,
    ).await {
        Ok(app) => Ok(Json(app)),
        Err(_) => {
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to update application"
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

/// Start a specific application.
///
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_id` - The ID of the application to start
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// The updated application if found, or None if not found
#[put("/platform/<platform_id>/apps/<app_id>/start")]
pub async fn start_app(
    platform_id: i64,
    app_id: String,
    db_manager: &State<Arc<DatabaseManager>>
) -> Result<Json<Application>, (Status, Json<Value>)> {
    // This function is already marked as todo!, but we need to add platform-specific
    // handling for future implementation
    todo!()
}

/// Stop a specific application.
///
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_id` - The ID of the application to stop
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// The updated application if found, or None if not found
#[put("/platform/<platform_id>/apps/<app_id>/stop")]
pub async fn stop_app(
    platform_id: i64,
    app_id: String,
    db_manager: &State<Arc<DatabaseManager>>
) -> Result<Json<Application>, (Status, Json<Value>)> {
    // This function is already marked as todo!, but we need to add platform-specific
    // handling for future implementation
    todo!()
}

/// Scale a specific application.
///
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_id` - The ID of the application to scale
/// * `scale` - JSON data containing scaling parameters
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// The updated application if found, or None if not found
#[put("/platform/<platform_id>/apps/<app_id>/scale", format = "json", data = "<scale>")]
pub async fn scale_app(
    platform_id: i64,
    app_id: String, 
    scale: Json<ScaleRequest>,
    db_manager: &State<Arc<DatabaseManager>>
) -> Result<Json<Application>, (Status, Json<Value>)> {
    // This function is already marked as todo!, but we need to add platform-specific
    // handling for future implementation
    todo!()
}

/// Delete a specific application.
///
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_id` - The ID of the application to delete
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// A JSON response indicating success or an error message
#[delete("/platform/<platform_id>/apps/<app_id>")]
pub async fn delete_app(
    platform_id: i64,
    app_id: String,
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

    match app_id.parse::<i64>() {
        Ok(id) => {
            match db::app::delete_app(&pool, id).await {
                Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
                Err(_) => {
                    Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to delete application"
                        }))
                    ))
                }
            }
        }
        Err(e) => {
            Err((
                Status::BadRequest,
                Json(json!({
                    "error": "Invalid ID format",
                    "message": format!("The application ID must be a valid integer: {}", e)
                }))
            ))
        }
    }
}

/// Releases a new version of the target application by uploading an artifact.
/// TODO: @tristanpoland Review if we actually need this or should drop in favor
///       of using the deploy route.
/// 
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_id` - The ID of the application to release a new version for
/// * `release_version` - The version tag for this release
/// * `content_type` - The content type of the data being uploaded
/// * `data` - The data stream of the artifact being uploaded
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// * `Status::Ok` - If the artifact is successfully uploaded and added to the build jobs list
/// * `Status::BadRequest` - If there is an error in the upload process
///
/// # Details
///
/// This route handles the release of a new version of an application by:
/// 1. Uploading the provided artifact to the build artifacts list.
/// 2. Adding the artifact to the list of build jobs for the Forge instances to pick up and process.
///
/// The actual implementation of the release process is delegated to the `helpers::release::release`
/// function, as it is quite extensive.
#[post(
    "/platform/<platform_id>/apps/<app_id>/releases/<release_version>/upload",
    format = "multipart/form-data",
    data = "<data>"
)]
pub async fn release(
    platform_id: i64,
    app_id: String,
    release_version: String,
    content_type: &ContentType,
    data: Data<'_>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Status, Status> {
    // We need to modify the helper function to work with platform-specific DBs
    // For now, we'll just pass the helper what it needs, but ideally the helper should be updated to use platform pools

    // Get platform info and pass to helper
    match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(_) => {
            // We found the platform, proceed with release
            super::helpers::release::release(app_id, release_version, content_type, data).await
        },
        Err(_) => {
            // Platform not found
            Err(Status::NotFound)
        }
    }
}

// List all instances for an application with pagination
#[get("/platform/<platform_id>/apps/<app_id>/instances?<page>&<per_page>")]
pub async fn list_instances(
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
            let instances = match db::app::list_instances(&pool, app_id, p, pp).await {
                Ok(instances) => instances,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve instances"
                        }))
                    ));
                }
            };
            
            let total_count = match db::app::count_instances_by_app(&pool, app_id).await {
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
            
            let total_pages = (total_count as f64 / pp as f64).ceil() as i64;

            let response = json!({
                "instances": instances,
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