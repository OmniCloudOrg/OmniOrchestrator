use rocket::{get, post, put, delete, State, routes, http::ContentType, Data};
use rocket::http::Status;
use rocket::serde::json::{Json, Value, json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;

mod helpers;

// Types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    id: String,
    name: String,
    owner: String,
    instances: i32,
    memory: i32,  // in MB
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStats {
    cpu_usage: f64,
    memory_usage: i32,
    disk_usage: i32,
    requests_per_second: f64,
    response_time_ms: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppRequest {
    name: String,
    memory: i32,
    instances: i32,
}

// State management
type AppStore = Arc<RwLock<HashMap<String, Application>>>;

// Routes

// List all applications
#[get("/apps")]
pub async fn list_apps(store: &State<AppStore>) -> Json<Vec<Application>> {
    let apps = store.read().await;
    let apps_vec: Vec<Application> = apps.values().cloned().collect();
    Json(apps_vec)
}

// Get specific application
#[get("/apps/<app_id>")]
pub async fn get_app(app_id: String, store: &State<AppStore>) -> Option<Json<Application>> {
    let apps = store.read().await;
    apps.get(&app_id).cloned().map(Json)
}

// Create new application
#[post("/apps", format = "json", data = "<app_request>")]
pub async fn create_app(
    app_request: Json<CreateAppRequest>,
    store: &State<AppStore>
) -> Json<Application> {
    let mut apps = store.write().await;
    let app = Application {
        id: uuid::Uuid::new_v4().to_string(),
        name: app_request.name.clone(),
        owner: "current_user".to_string(), // TODO: Add auth
        instances: app_request.instances,
        memory: app_request.memory,
        status: "STOPPED".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    apps.insert(app.id.clone(), app.clone());
    Json(app)
}

// Get application statistics
#[get("/apps/<app_id>/stats")]
pub async fn get_app_stats(app_id: String) -> Json<AppStats> {
    // TODO: Implement real metrics collection
    Json(AppStats {
        cpu_usage: 45.5,
        memory_usage: 512,
        disk_usage: 1024,
        requests_per_second: 100.0,
        response_time_ms: 250,
    })
}

// Start application
#[put("/apps/<app_id>/start")]
pub async fn start_app(app_id: String, store: &State<AppStore>) -> Option<Json<Application>> {
    let mut apps = store.write().await;
    if let Some(app) = apps.get_mut(&app_id) {
        app.status = "RUNNING".to_string();
        app.updated_at = chrono::Utc::now();
        Some(Json(app.clone()))
    } else {
        None
    }
}

// Stop application
#[put("/apps/<app_id>/stop")]
pub async fn stop_app(app_id: String, store: &State<AppStore>) -> Option<Json<Application>> {
    let mut apps = store.write().await;
    if let Some(app) = apps.get_mut(&app_id) {
        app.status = "STOPPED".to_string();
        app.updated_at = chrono::Utc::now();
        Some(Json(app.clone()))
    } else {
        None
    }
}

// Scale application
#[put("/apps/<app_id>/scale", format = "json", data = "<scale>")]
pub async fn scale_app(
    app_id: String,
    scale: Json<ScaleRequest>,
    store: &State<AppStore>
) -> Option<Json<Application>> {
    let mut apps = store.write().await;
    if let Some(app) = apps.get_mut(&app_id) {
        app.instances = scale.instances;
        app.memory = scale.memory;
        app.updated_at = chrono::Utc::now();
        Some(Json(app.clone()))
    } else {
        None
    }
}

// Delete application
#[delete("/apps/<app_id>")]
pub async fn delete_app(app_id: String, store: &State<AppStore>) -> Option<Json<Value>> {
    let mut apps = store.write().await;
    apps.remove(&app_id).map(|_| Json(json!({ "status": "deleted" })))
}

/// Releases a new version of the target application by uploading an artifact.
///
/// # Arguments
///
/// * `content_type` - The content type of the data being uploaded.
/// * `data` - The data stream of the artifact being uploaded.
///
/// # Returns
///
/// * `Status::Ok` - If the artifact is successfully uploaded and added to the build jobs list.
/// * `Status::BadRequest` - If there is an error in the upload process.
///
/// # Details
///
/// This route handles the release of a new version of an application by:
/// 1. Uploading the provided artifact to the build artifacts list.
/// 2. Adding the artifact to the list of build jobs for the Forge instances to pick up and process.
///
/// The actual implementation of the release process is delegated to the `helpers::release::release`
/// function, as it is quite extensive.
#[post("/apps/<app_id>/releases/<release_version>/upload", format = "multipart/form-data", data = "<data>")]
pub async fn release(app_id: String, release_version: String, content_type: &ContentType, data: Data<'_>) -> Result<Status, Status> {
    helpers::release::release(app_id, release_version, content_type, data).await
}

// Helper structs
#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleRequest {
    instances: i32,
    memory: i32,
}