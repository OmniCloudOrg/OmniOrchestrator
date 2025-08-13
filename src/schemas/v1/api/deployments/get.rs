use std::sync::Arc;
use crate::DatabaseManager;
use super::super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};

use libomni::types::db::v1 as types;
use types::deployment::Deployment;

/// Get a specific deployment by ID.
#[get("/platform/<platform_id>/deployments/<deployment_id>")]
pub async fn get_deployment(
    platform_id: i64,
    deployment_id: i64,
    db_manager: &State<Arc<DatabaseManager>>
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

    match db::deployment::get_deployment_by_id(&pool, deployment_id).await {
        Ok(deployment) => Ok(Json(deployment)),
        Err(_) => Err((
            Status::NotFound,
            Json(json!({
                "error": "Deployment not found",
                "message": format!("Deployment with ID {} could not be found", deployment_id)
            }))
        )),
    }
}