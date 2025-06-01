//! Provider management module for handeling CRUD operations on providers.
//! 
//! This module provides functionality to create, read, update, and delete providers in the system. It includes API endpoints for managing providers and their associated resources.
//! It also includes a function to retrieve a paginated list of providers from the database.
//! It is designed to be used primarily by the dashboard to add new providers and manage existing ones.
//! The resulting database table is read at runtime by the various directories to determine which provider configs they need to have access to.

use crate::schemas::v1::db::queries::{self as db};
use rocket::serde::json::{Json, Value};
use rocket::http::Status;
use serde_json::json;
use std::sync::Arc;
use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::provider::{ProviderAuditLog, Provider};
use types::instance::Instance;

/// List all providers in the system with pagination support.
/// 
/// # Arguments
/// * `platform_id` - The ID of the platform to retrieve providers for.
/// * `page` - The page number to retrieve.
/// * `per_page` - The number of providers to retrieve per page.
/// * `db_manager` - The database manager for accessing platform-specific database pools.
/// 
/// # Returns
/// A JSON response containing the list of providers and pagination information.
#[get("/platform/<platform_id>/providers?<page>&<per_page>")]
pub async fn list_providers(
    platform_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    db_manager: &rocket::State<Arc<DatabaseManager>>,
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

    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    let providers: Vec<Provider> = match db::provider::get_providers_paginated(&pool, page, per_page).await {
        Ok(providers) => providers,
        Err(e) => {
            tracing::error!("Failed to fetch providers: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch providers"
                }))
            ));
        }
    };

    let total_count = match db::provider::get_provider_count(&pool).await {
        Ok(count) => count,
        Err(e) => {
            tracing::error!("Failed to fetch provider count: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count providers"
                }))
            ));
        }
    };

    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

    // Pagination format as the standard for all paginated responses from the API
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

/// Retrieves a paginated list of audit logs for a specific provider.
#[get("/platform/<platform_id>/providers/<provider_id>/audit_logs?<page>&<per_page>")]
pub async fn get_provider_audit_logs_paginated(
    platform_id: i64,
    provider_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    db_manager: &rocket::State<Arc<DatabaseManager>>,
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

    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    let audit_logs: Vec<ProviderAuditLog> = match db::provider::get_provider_audit_logs_paginated(&pool, provider_id, page, per_page).await {
        Ok(audit_logs) => audit_logs,
        Err(e) => {
            tracing::error!("Failed to fetch provider audit logs: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch provider audit logs"
                }))
            ));
        }
    };

    let total_count = match db::provider::get_provider_audit_log_count(&pool, provider_id).await {
        Ok(count) => count,
        Err(e) => {
            tracing::error!("Failed to fetch provider audit log count: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count provider audit logs"
                }))
            ));
        }
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
#[get("/platform/<platform_id>/providers/<provider_id>/instances?<page>&<per_page>")]
pub async fn get_provider_instances(
    platform_id: i64,
    provider_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    db_manager: &rocket::State<Arc<DatabaseManager>>,
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

    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    tracing::info!("Fetching instances for provider {} (page: {}, per_page: {})", provider_id, page, per_page);

    let instances: Vec<Instance> = match db::provider::get_provider_instances(&pool, provider_id, page, per_page).await {
        Ok(instances) => {
            tracing::debug!("Retrieved {} instances", instances.len());
            instances
        }
        Err(e) => {
            tracing::error!("Failed to fetch provider instances: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch provider instances"
                }))
            ));
        }
    };

    let total_count = match db::provider::get_provider_instance_count(&pool, provider_id).await {
        Ok(count) => {
            tracing::debug!("Total instance count: {}", count);
            count
        }
        Err(e) => {
            tracing::error!("Failed to get provider instance count: {}", e);
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count provider instances"
                }))
            ));
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