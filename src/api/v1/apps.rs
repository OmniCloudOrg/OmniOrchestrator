use rocket::{get, post, put, delete, State, http::ContentType, Data};
use rocket::http::Status;
use rocket::serde::json::{Json, Value, json};
use serde::{Deserialize, Serialize};
use sqlx::MySql;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use crate::db::tables::App;
use crate::db::v1::queries as db;

// Types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    id: String,
    name: String,
    owner: String,
    instances: i64,
    memory: i64,  // in MB
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleRequest {
    instances: i32,
    memory: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStats {
    cpu_usage: f64,
    memory_usage: i64,
    disk_usage: i64,
    requests_per_second: f64,
    response_time_ms: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppRequest {
    name: String,
    memory: i64,
    instances: i64,
    org_id: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppRequest {
    name: String,
    memory: i64,
    instances: i64,
    org_id: i64
}

// State management
type AppStore = Arc<RwLock<HashMap<String, Application>>>;


// List all applications
#[get("/apps?<page>&<per_page>")]
pub async fn list_apps(page: i64, per_page: i64, pool: &State<sqlx::Pool<MySql>>) -> Json<Vec<App>> {
    let apps = db::app::list_apps(pool, page, per_page).await.unwrap();
    println!("Found {} apps", apps.len());
    let apps_vec: Vec<App> = apps.into_iter().collect();
    println!("Returning {} apps", apps_vec.len());
    Json(apps_vec)
}

// Get specific application
#[get("/apps/<app_id>")]
pub async fn get_app(app_id: i64, pool: &State<sqlx::Pool<MySql>>) -> Option<Json<App>> {
    let app_result = db::app::get_app_by_id(pool, app_id).await;
    let app: Option<App> = match app_result {
        Ok(app) => Some(app),
        Err(_) => {
            println!("Client requested app: {} but the app could not be found by the DB query", app_id);
            None
        },
    };
    app.map(Json)
}

// Create new application
#[post("/apps", format = "json", data = "<app_request>")]
pub async fn create_app(
    app_request: Json<CreateAppRequest>,
    pool: &State<sqlx::Pool<MySql>>
) -> Json<App> {
    // let mut apps = store.write().await;

    let app = db::app::create_app(
        pool,
        &app_request.name,
        app_request.org_id,
        None,
        None,   
        None,
        None
    ).await.unwrap();
    Json(app)
}

#[post("/apps/<app_id>",format = "json", data = "<app_request>")]
pub async fn update_app(
    app_request: Json<UpdateAppRequest>,
    pool: &State<sqlx::Pool<MySql>>,
    app_id: i64) -> Json<App> {
    let app = db::app::update_app(pool,app_id,Some(&app_request.name),None,None,None,None,None).await.unwrap();


    Json(app)
}

// Get application statistics
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
        // app.instances = scale.instances;
        // app.memory = scale.memory;
        app.updated_at = chrono::Utc::now();
        Some(Json(app.clone()))
    } else {
        None
    }
}

// Delete application
#[delete("/apps/<app_id>")]
pub async fn delete_app(app_id: String,pool: &State<sqlx::Pool<MySql>>) -> Result<Json<Value>,(rocket::http::Status,String)> {
    match app_id.parse::<i64>() {
        Ok(id) => {
            db::app::delete_app(pool,id).await.unwrap();
            Ok(Json(json!({ "status": "deleted" })))
        },
        Err(e) => {
            let code = rocket::http::Status::Ok;
            Err((code,format!("{e}")))
        },
    }
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
    // See if the app exists in DB
        // If not create new app and return app ID
        // If so we need to fetch the existing app ID
    //Create the build recrd in builds table using the app ID

    // Accept the release tarball and save it to the filesystem
    

    super::helpers::release::release(app_id, release_version, content_type, data).await
}
