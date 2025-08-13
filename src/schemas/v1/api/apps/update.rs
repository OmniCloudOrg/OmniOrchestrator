use super::super::super::db::queries as db;
use super::types::UpdateAppRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{post, State};
use std::sync::Arc;

use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::app::App;

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