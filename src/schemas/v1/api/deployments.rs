use std::sync::Arc;
use crate::DatabaseManager;
use super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put, State};
use serde::{Deserialize, Serialize};

use libomni::types::db::v1 as types;
use types::deployment::Deployment;

/// Request body for creating a deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeploymentRequest {
    pub app_id: i64,
    pub build_id: i64,
    pub version: String,
    pub deployment_strategy: String,
    pub previous_deployment_id: Option<i64>,
    pub canary_percentage: Option<i64>,
    pub environment_variables: Option<serde_json::Value>,
    pub annotations: Option<serde_json::Value>,
    pub labels: Option<serde_json::Value>,
}

/// Request body for updating a deployment's status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeploymentStatusRequest {
    pub status: String,
    pub error_message: Option<String>,
}

/// List all deployments with pagination support.
#[get("/platform/<platform_id>/deployments?<page>&<per_page>")]
pub async fn list_deployments(
    platform_id: i64,
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
            let deployments = match db::deployment::list_deployments(&pool, p, pp).await {
                Ok(deployments) => deployments,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve deployments"
                        }))
                    ));
                }
            };
            
            let total_count = match db::deployment::count_deployments(&pool).await {
                Ok(count) => count,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to count deployments"
                        }))
                    ));
                }
            };
            
            let total_pages = (total_count as f64 / pp as f64).ceil() as i64;

            let response = json!({
                "deployments": deployments,
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

/// Count the total number of deployments.
#[get("/platform/<platform_id>/count/deployments")]
pub async fn count_deployments(
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

    match db::deployment::count_deployments(&pool).await {
        Ok(count) => Ok(Json(count)),
        Err(_) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Database error",
                "message": "Failed to count deployments"
            }))
        )),
    }
}

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

/// List all deployments for a specific application with pagination.
#[get("/platform/<platform_id>/apps/<app_id>/deployments?<page>&<per_page>")]
pub async fn list_app_deployments(
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
            let deployments = match db::deployment::list_deployments_by_app(&pool, app_id, p, pp).await {
                Ok(deployments) => deployments,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve deployments"
                        }))
                    ));
                }
            };
            
            let total_count = match db::deployment::count_deployments_by_app(&pool, app_id).await {
                Ok(count) => count,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to count deployments"
                        }))
                    ));
                }
            };
            
            let total_pages = (total_count as f64 / pp as f64).ceil() as i64;

            let response = json!({
                "deployments": deployments,
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

/// Delete a specific deployment.
#[delete("/platform/<platform_id>/deployments/<deployment_id>")]
pub async fn delete_deployment(
    platform_id: i64,
    deployment_id: i64,
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

    match db::deployment::delete_deployment(&pool, deployment_id).await {
        Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
        Err(e) => Err((
            Status::InternalServerError, 
            Json(json!({
                "error": "Database error",
                "message": e.to_string()
            }))
        )),
    }
}