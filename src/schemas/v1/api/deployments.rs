// routes/deployment.rs
use crate::models::deployment::Deployment;
use super::super::db::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put, State};
use serde::{Deserialize, Serialize};
use sqlx::MySql;

/// Request data for creating a new deployment.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeploymentRequest {
    /// ID of the application being deployed
    app_id: i64,
    /// ID of the build being deployed
    build_id: i64,
    /// Version tag for this deployment
    version: String,
    /// Strategy for deployment (e.g., "rolling", "blue-green")
    deployment_strategy: String,
    /// Optional ID of the previous deployment
    previous_deployment_id: Option<i64>,
    /// Percentage of traffic for canary deployments
    canary_percentage: Option<i64>,
    /// Environment variables for the deployment
    environment_variables: Option<serde_json::Value>,
    /// Annotations for the deployment
    annotations: Option<serde_json::Value>,
    /// Labels for the deployment
    labels: Option<serde_json::Value>,
}

/// Request data for updating a deployment's status.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeploymentStatusRequest {
    /// New status for the deployment
    status: String,
    /// Optional error message if the deployment failed
    error_message: Option<String>,
}

/// List all deployments with pagination support.
#[get("/deployments?<page>&<per_page>")]
pub async fn list_deployments(
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let deployments = db::deployment::list_deployments(pool, p, pp).await.unwrap();
            let total_count = db::deployment::count_deployments(pool).await.unwrap();
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
#[get("/deployments/count")]
pub async fn count_deployments(pool: &State<sqlx::Pool<MySql>>) -> Json<i64> {
    let count = db::deployment::count_deployments(pool).await.unwrap();
    Json(count)
}

/// Get a specific deployment by ID.
#[get("/deployments/<deployment_id>")]
pub async fn get_deployment(deployment_id: i64, pool: &State<sqlx::Pool<MySql>>) -> Option<Json<Deployment>> {
    let deployment_result = db::deployment::get_deployment_by_id(pool, deployment_id).await;
    let deployment: Option<Deployment> = match deployment_result {
        Ok(deployment) => Some(deployment),
        Err(_) => {
            println!(
                "Client requested deployment: {} but the deployment could not be found by the DB query",
                deployment_id
            );
            None
        }
    };
    deployment.map(Json)
}

/// List all deployments for a specific application with pagination.
#[get("/apps/<app_id>/deployments?<page>&<per_page>")]
pub async fn list_app_deployments(
    app_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let deployments = db::deployment::list_deployments_by_app(pool, app_id, p, pp).await.unwrap();
            let total_count = db::deployment::count_deployments_by_app(pool, app_id).await.unwrap();
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
#[post("/deployments", format = "json", data = "<deployment_request>")]
pub async fn create_deployment(
    deployment_request: Json<CreateDeploymentRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Json<Deployment> {
    let deployment = db::deployment::create_deployment(
        pool,
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
    )
    .await
    .unwrap();
    
    Json(deployment)
}

/// Update a deployment's status.
#[put("/deployments/<deployment_id>/status", format = "json", data = "<status_request>")]
pub async fn update_deployment_status(
    deployment_id: i64,
    status_request: Json<UpdateDeploymentStatusRequest>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Deployment>, (Status, Json<Value>)> {
    match db::deployment::update_deployment_status(
        pool,
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
#[delete("/deployments/<deployment_id>")]
pub async fn delete_deployment(
    deployment_id: i64,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Value>, (Status, String)> {
    match db::deployment::delete_deployment(pool, deployment_id).await {
        Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}