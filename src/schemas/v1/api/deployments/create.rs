use std::sync::Arc;
use crate::DatabaseManager;
use super::super::super::db::queries as db;
use super::types::CreateDeploymentRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{post, State};

use libomni::types::db::v1 as types;
use types::deployment::Deployment;

/// Create a new deployment.
#[post("/platform/<platform_id>/deployments", format = "json", data = "<deployment_request>")]
pub async fn create_deployment(
    platform_id: i64,
    deployment_request: Json<CreateDeploymentRequest>,
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

    match db::deployment::create_deployment(
        &pool,
        deployment_request.app_id,
        deployment_request.build_id,
        &deployment_request.version,
        &deployment_request.deployment_strategy,
        deployment_request.previous_deployment_id,
        deployment_request.canary_percentage,
        deployment_request.environment_variables.clone(),
        deployment_request.annotations.clone(),
        deployment_request.labels.clone(),
        None, // created_by would typically come from auth middleware
    ).await {
        Ok(deployment) => Ok(Json(deployment)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to create deployment",
                "message": e.to_string()
            }))
        )),
    }
}