use crate::db::tables::Deployment;
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deployment {
    id: String,
    name: String,
    owner: String,
    instances: i64,
    memory: i64, // in MB
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeploymentRequest {
    name: String,
    memory: i64,
    instances: i64,
    org_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleRequest {
    instances: i32,
    memory: i32,
}

#[get("/apps/deployments?<page>&<per_page>")]
pub async fn list_deployments(
    page: i64,
    per_page: i64,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Json<Vec<Deployment>>, Status> {
    db::deployment::list_deployments(pool, )
    {
        Ok(deployments) => deployments,
        Err(e) => {
            log::error!("Failed to fetch deployments: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    Ok(Json(deployments))
}