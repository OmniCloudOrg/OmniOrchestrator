//! Alert management module for handeling CRUD operations on alerts.
//! 
//! This module provides functionality to create, read, update, and delete
//! alerts in the system. It includes endpoints for managing alerts
//! associated with applications and organizations.

use crate::db::tables::Alert;
use crate::db::v1::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, http::ContentType, post, put, Data, State};
use serde::{Deserialize, Serialize};
use sqlx::MySql;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[get("/alerts?<page>&<per_page>")]
pub async fn list_alerts(
    page: i64,
    per_page: i64,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Json<Vec<Alert>>, Status> {
    let alerts = db::alert::list_alerts(pool, page, per_page).await.map_err(|e| {
        log::error!("Failed to fetch alerts: {}", e);
        Status::InternalServerError
    })?;
    Ok(Json(alerts))
}