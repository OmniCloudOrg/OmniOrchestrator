//! Worker management module for the OmniOrchestrator API
//! 
//! This module provides a REST API for managing workers, including:
//! - Listing workers
//! - Creating new workers
//! - Updating existing workers
//! - Getting worker details and statistics
//! - Starting and stopping workers
//! - Scaling workers
//! - Deleting workers

use crate::db::tables::Worker;
use crate::db::v1::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, http::ContentType, post, put, Data, State};
use serde::{Deserialize, Serialize};
use sqlx::MySql;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// List all workers with pagination support.
#[get("/workers?<page>&<per_page>")]
pub async fn list_workers(
    page: Option<u64>,
    per_page: Option<u64>,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Vec<Worker>>, Status> {
    let workers = db::worker::list_workers(pool, page, per_page).await.map_err(|_| Status::InternalServerError)?;
    Ok(Json(workers))
}

/// Get a worker by its ID.
#[get("/workers/<worker_id>")]
pub async fn get_worker_by_id(
    worker_id: i64,
    pool: &State<sqlx::Pool<MySql>>,
) -> Result<Json<Worker>, Status> {
    let worker = db::worker::get_worker_by_id(pool, worker_id).await.map_err(|_| Status::NotFound)?;
    Ok(Json(worker))
}