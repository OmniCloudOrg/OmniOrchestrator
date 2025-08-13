use std::sync::Arc;
use crate::DatabaseManager;
use super::super::super::db::queries as db;
use super::types::UpdateDeploymentStatusRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{put, State};

use libomni::types::db::v1 as types;
use types::deployment::Deployment;

/// Update a deployment's status.
#[put("/platform/<platform_id>/deployments/<deployment_id>/status", format = "json", data = "<status_request>")]
pub async fn update_deployment_status(
    platform_id: i64,
    deployment_id: i64,
    status_request: Json<UpdateDeploymentStatusRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Deployment>, (Status, Json<Value>)> {
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

    match db::deployment::update_deployment_status(
        &pool,
        deployment_id,
        &status_request.status,
        status_request.error_message.as_deref(),
    ).await {
        Ok(deployment) => Ok(Json(deployment)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to update deployment status",
                "message": e.to_string()
            }))
        )),
    }
}