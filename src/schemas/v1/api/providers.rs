//! Provider management module for handeling CRUD operations on providers.
//! 
//! This module provides functionality to create, read, update, and delete providers in the system. It includes API endpoints for managing providers and their associated resources.
//! It also includes a function to retrieve a paginated list of providers from the database.
//! It is designed to be used primarily by the dashboard to add new providers and manage existing ones.
//! The resulting database table is read at runtime by the various directories to determine which provider configs they need to have access to.

use crate::db::tables::ProviderAuditLog;
use crate::schemas::v1::db::tables::{Provider, Instance};
use crate::schemas::v1::db::queries::{self as queries};
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

    let providers: Vec<Provider> = match queries::provider::get_providers_paginated(&pool, page, per_page).await {
        Ok(providers) => providers,
        Err(_) => return Err(Status::InternalServerError),
    };

    let total_count = match queries::provider::get_provider_count(&pool).await {
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

/// Retrieves a pagnated list of audit logs for a specific provider.
#[get("/providers/<provider_id>/audit_logs?<page>&<per_page>")]
pub async fn get_provider_audit_logs_paginated(
    provider_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &rocket::State<sqlx::Pool<sqlx::MySql>>,
) -> Result<Json<Value>, Status> {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    let audit_logs: Vec<ProviderAuditLog> = match queries::provider::get_provider_audit_logs_paginated(&pool, provider_id, page, per_page).await {
        Ok(audit_logs) => audit_logs,
        Err(_) => return Err(Status::InternalServerError),
    };

    let total_count = match queries::provider::get_provider_audit_log_count(&pool, provider_id).await {
        Ok(count) => count,
        Err(_) => return Err(Status::InternalServerError),
    };

    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

    let response = json!({
        "audit_logs": audit_logs,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    });

    Ok(Json(response))
}

/// Fetch all instances for a given provider.
#[get("/providers/<provider_id>/instances?<page>&<per_page>")]
pub async fn get_provider_instances(
    provider_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &rocket::State<sqlx::Pool<sqlx::MySql>>,
) -> Result<Json<Value>, Status> {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    tracing::info!("Fetching instances for provider {} (page: {}, per_page: {})", provider_id, page, per_page);

    let instances: Vec<Instance> = match queries::provider::get_provider_instances(&pool, provider_id, page, per_page).await {
        Ok(instances) => {
            tracing::debug!("Retrieved {} instances", instances.len());
            instances
        }
        Err(e) => {
            tracing::error!("Failed to fetch provider instances: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let total_count = match queries::provider::get_provider_instance_count(&pool, provider_id).await {
        Ok(count) => {
            tracing::debug!("Total instance count: {}", count);
            count
        }
        Err(e) => {
            tracing::error!("Failed to get provider instance count: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    tracing::debug!("Total pages: {}", total_pages);

    let response = json!({
        "instances": instances,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    });

    tracing::info!("Successfully retrieved instances for provider {}", provider_id);
    Ok(Json(response))
}