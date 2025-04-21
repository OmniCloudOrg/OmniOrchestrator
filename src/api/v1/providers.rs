//! Provider management module for handeling CRUD operations on providers.
//! 
//! This module provides functionality to create, read, update, and delete providers in the system. It includes API endpoints for managing providers and their associated resources.
//! It also includes a function to retrieve a paginated list of providers from the database.
//! It is designed to be used primarily by the dashboard to add new providers and manage existing ones.
//! The resulting database table is read at runtime by the various directories to determine which provider configs they need to have access to.

use crate::db::v1::tables::Provider;
use rocket::http::Status;
use rocket::serde::json::{Json, Value};
use serde_json::json;

/// List all providers in the system with pagination support.
/// 
/// # Arguments
/// * `page` - The page number to retrieve.
/// * `per_page` - The number of providers to retrieve per page.
/// * `pool` - The database connection pool.
/// 
/// # Returns
/// A JSON response containing the list of providers and pagination information.
#[get("/providers?<page>&<per_page>")]
pub async fn list_providers(
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &rocket::State<sqlx::Pool<sqlx::MySql>>,
) -> Result<Json<Value>, Status> {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    let providers: Vec<Provider> = match crate::db::v1::queries::provider::get_providers_paginated(&pool, page, per_page).await {
        Ok(providers) => providers,
        Err(_) => return Err(Status::InternalServerError),
    };

    let total_count = match crate::db::v1::queries::provider::get_provider_count(&pool).await {
        Ok(count) => count,
        Err(_) => return Err(Status::InternalServerError),
    };

    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

    // TODO: This pagination format should be the new golden standard for all paginated responses from the API
    let response = json!({
        "providers": providers,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    });

    Ok(Json(response))
}