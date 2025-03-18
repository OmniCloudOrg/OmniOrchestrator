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

use crate::db::tables::App;
use crate::db::v1::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, http::ContentType, post, put, Data, State};
use serde::{Deserialize, Serialize};
use sqlx::MySql;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Types

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

// State management

/// Type alias for application state storage.
type AppStore = Arc<RwLock<HashMap<String, Application>>>;

/// List all applications with pagination support.
///
/// # Arguments
///
/// * `page` - Page number for pagination
/// * `per_page` - Number of items per page
/// * `pool` - Database connection pool
///
/// # Returns
///
/// A JSON array of applications
#[get("/apps?<page>&<per_page>")]
pub async fn list_apps(
    page: i64,
    per_page: i64,
    pool: &State<sqlx::Pool<MySql>>,
) -> Json<Vec<App>> {
    let apps = db::app::list_apps(pool, page, per_page).await.unwrap();
    println!("Found {} apps", apps.len());
    let apps_vec: Vec<App> = apps.into_iter().collect();
    println!("Returning {} apps", apps_vec.len());
    Json(apps_vec)
}

/// Get a specific application by ID.
///
/// # Arguments
///
/// * `app_id` - The ID of the application to retrieve
/// * `pool` - Database connection pool
///
/// # Returns
///
/// The application if found, or None if not found
#[get("/apps/<app_id>")]
pub async fn get_app(app_id: i64, pool: &State<sqlx::Pool<MySql>>) -> Option<Json<App>> {
    let app_result = db::app::get_app_by_id(pool, app_id).await;
    let app: Option<App> = match app_result {
        Ok(app) => Some(app),
        Err(_) => {
            println!(
                "Client requested app: {} but the app could not be found by the DB query",
                app_id
            );
            None
        }
    };
    app.map(Json)
}

/// Create a new application.
///
/// # Arguments
///
/// * `app_request` - JSON data containing application details
/// * `pool` - Database connection pool
///
/// # Returns
///
/// The newly created application
#[post("/apps", format = "json", data = "<app_request>")]
pub async fn create_app(
    app_request: Json<CreateAppRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Json<App> {
    // let mut apps = store.write().await;

    let app = db::app::create_app(
        pool,
        &app_request.name,
        app_request.org_id,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    Json(app)
}

/// Update an existing application.
///
/// # Arguments
///
/// * `app_request` - JSON data containing updated application details
/// * `pool` - Database connection pool
/// * `app_id` - The ID of the application to update
///
/// # Returns
///
/// The updated application
#[post("/apps/<app_id>", format = "json", data = "<app_request>")]
pub async fn update_app(
    app_request: Json<UpdateAppRequest>,
    pool: &State<sqlx::Pool<MySql>>,
    app_id: i64,
) -> Json<App> {
    let app = db::app::update_app(
        pool,
        app_id,
        Some(&app_request.name),
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    Json(app)
}

/// Get statistics for a specific application.
///
/// # Arguments
///
/// * `app_id` - The ID of the application to get statistics for
/// * `pool` - Database connection pool
///
/// # Returns
///
/// Statistics for the application
#[get("/apps/<app_id>/stats")]
pub async fn get_app_stats(app_id: String, pool: &State<sqlx::Pool<MySql>>) -> Json<AppStats> {
    let app_stats = AppStats {
        cpu_usage: 0.0,
        memory_usage: 0,
        disk_usage: 0,
        requests_per_second: 0.0,
        response_time_ms: 0,
    };
    Json(app_stats)
}

/// Start a specific application.
///
/// # Arguments
///
/// * `app_id` - The ID of the application to start
///
/// # Returns
///
/// The updated application if found, or None if not found
#[put("/apps/<app_id>/start")]
pub async fn start_app(app_id: String) -> Option<Json<Application>> {
    todo!()
}

/// Stop a specific application.
///
/// # Arguments
///
/// * `app_id` - The ID of the application to stop
///
/// # Returns
///
/// The updated application if found, or None if not found
#[put("/apps/<app_id>/stop")]
pub async fn stop_app(app_id: String) -> Option<Json<Application>> {
    todo!()
}

/// Scale a specific application.
///
/// # Arguments
///
/// * `app_id` - The ID of the application to scale
/// * `scale` - JSON data containing scaling parameters
///
/// # Returns
///
/// The updated application if found, or None if not found
#[put("/apps/<app_id>/scale", format = "json", data = "<scale>")]
pub async fn scale_app(app_id: String, scale: Json<ScaleRequest>) -> Option<Json<Application>> {
    todo!()
}

/// Delete a specific application.
///
/// # Arguments
///
/// * `app_id` - The ID of the application to delete
/// * `pool` - Database connection pool
///
/// # Returns
///
/// A JSON response indicating success or an error message
#[delete("/apps/<app_id>")]
pub async fn delete_app(
    app_id: String,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (rocket::http::Status, String)> {
    match app_id.parse::<i64>() {
        Ok(id) => {
            db::app::delete_app(pool, id).await.unwrap();
            Ok(Json(json!({ "status": "deleted" })))
        }
        Err(e) => {
            let code = rocket::http::Status::Ok;
            Err((code, format!("{e}")))
        }
    }
}

/// Releases a new version of the target application by uploading an artifact.
///
/// # Arguments
///
/// * `app_id` - The ID of the application to release a new version for
/// * `release_version` - The version tag for this release
/// * `content_type` - The content type of the data being uploaded
/// * `data` - The data stream of the artifact being uploaded
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
    "/apps/<app_id>/releases/<release_version>/upload",
    format = "multipart/form-data",
    data = "<data>"
)]
pub async fn release(
    app_id: String,
    release_version: String,
    content_type: &ContentType,
    data: Data<'_>,
) -> Result<Status, Status> {
    // See if the app exists in DB
    // If not create new app and return app ID
    // If so we need to fetch the existing app ID
    //Create the build recrd in builds table using the app ID

    // Accept the release tarball and save it to the filesystem

    super::helpers::release::release(app_id, release_version, content_type, data).await
}