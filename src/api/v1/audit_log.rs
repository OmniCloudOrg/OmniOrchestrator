//! Audit log management module for tracking user actions.
//!
//! This module provides a REST API for managing audit logs, including:
//! - Creating new audit log entries
//! - Listing audit logs with pagination support

use crate::db::v1::queries as db;
use rocket::get;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::MySql;

/// Creates a new audit log entry in the system.
///
/// This endpoint records an action performed by a user on a specific resource,
/// which is useful for compliance, security monitoring, and troubleshooting.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `audit_log` - JSON data containing audit log details
///
/// # Returns
///
/// The newly created audit log entry with server-generated fields (like ID and timestamps)
#[post("/audit_log", format = "json", data = "<audit_log>")]
pub async fn create_audit_log(
    pool: &State<sqlx::Pool<MySql>>,
    audit_log: Json<crate::db::v1::tables::AuditLog>,
) -> Json<crate::db::v1::tables::AuditLog> {
    let audit_log_result = db::audit_log::create_audit_log(
        pool,
        audit_log.user_id,
        audit_log.org_id,
        &audit_log.action,
        &audit_log.resource_type,
        //TODO: We should look into not cloning this in the future if possible
        audit_log.resource_id.clone(),
    )
    .await
    .unwrap();

    Json(audit_log_result)
}

/// List audit log entries with pagination support.
///
/// This endpoint retrieves a paginated list of audit log entries, allowing
/// administrators to review user actions within the system.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `page` - Optional page number for pagination (defaults to 1)
/// * `per_page` - Optional number of items per page (defaults to 10)
///
/// # Returns
///
/// A JSON array of audit log entries
#[get("/audit_logs?<page>&<per_page>")]
pub async fn list_audit_logs(
    pool: &State<sqlx::Pool<MySql>>,
    page: Option<u32>,
    per_page: Option<u32>,
) -> Json<Vec<crate::db::v1::tables::AuditLog>> {
    let page: i64 = page.unwrap_or(1).into();
    let per_page: i64 = per_page.unwrap_or(10).into();

    let audit_logs = db::audit_log::list_audit_logs_paginated(pool, per_page, page)
        .await
        .unwrap();

    Json(audit_logs)
}