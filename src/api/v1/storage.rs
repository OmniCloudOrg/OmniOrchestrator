use crate::db::v1::tables::{
    StorageClass,
    StorageMigration,
    StorageQosPolicy,
    StorageSnapshot,
    StorageVolume,
};
use crate::db::v1::queries as db;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, http::ContentType, post, put, Data, State};
use serde::{Deserialize, Serialize};
use sqlx::MySql;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Retrieves a paginated list of storage volumes
#[get("/storage/volumes?<page>&<per_page>")]
pub async fn list_storage_volumes_paginated(
    pool: &State<sqlx::Pool<MySql>>,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Json<Value> {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    let storage_volumes = db::storage::list_storage_volumes_paginated(pool, page, per_page)
        .await
        .expect("Failed to list storage volumes");

    let total_count = db::storage::count_storage_volumes(pool)
        .await
        .expect("Failed to get total count of storage volumes");

    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

    let response = json!({
        "storage_volumes": storage_volumes,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    });

    Json(response)
}

/// Retrieves a count of storage volumes
#[get("/storage/volumes/count")]
pub async fn count_storage_volumes(
    pool: &State<sqlx::Pool<MySql>>,
) -> Json<Value> {
    let count = db::storage::count_storage_volumes(pool)
        .await
        .expect("Failed to get total count of storage volumes");

    Json(json!({ "count": count }))
}