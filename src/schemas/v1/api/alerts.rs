//! Alert management module for handling CRUD operations on alerts.
//! 
//! This module provides functionality to create, read, update, and delete
//! alerts in the system. It includes endpoints for managing alerts
//! associated with applications and organizations.

use crate::db::tables::{
    Alert, AlertWithRelatedData, AlertAcknowledgment, AlertEscalation
};
use super::super::db::queries as db;
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, http::ContentType, post, put, State};
use serde::{Deserialize, Serialize};
use rocket::time::OffsetDateTime;
use std::collections::HashMap;
use crate::db::tables::User; // Add this at the top if not already present

// Request and response structs

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAlertRequest {
    alert_type: String,
    severity: String,
    service: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAlertStatusRequest {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgeAlertRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
    #[serde(default)]
    update_status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEscalationRequest {
    escalation_level: i64,
    escalated_to: serde_json::Value,
    escalation_method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_required_by: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkUpdateStatusRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_id: Option<i64>,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
}

// API Routes

/// Get a paginated list of alerts with filtering options
#[get("/alerts?<page>&<per_page>&<status>&<severity>&<org_id>&<app_id>&<service>&<from_date>&<to_date>")]
pub async fn list_alerts(
    page: Option<i64>,
    per_page: Option<i64>,
    status: Option<String>,
    severity: Option<String>,
    org_id: Option<i64>,
    app_id: Option<i64>,
    service: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // Set default pagination if not provided
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(20);
    
    // Convert Optional String to Optional &str
    let status_ref = status.as_deref();
    let severity_ref = severity.as_deref();
    let service_ref = service.as_deref();

    // Fetch alerts with filters
    let alerts = db::alert::list_alerts(
        pool, 
        page, 
        per_page,
        status_ref,
        severity_ref,
        org_id,
        app_id,
        service_ref,
        from_date.and_then(|date_str| chrono::DateTime::parse_from_rfc3339(&date_str).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
        to_date.and_then(|date_str| chrono::DateTime::parse_from_rfc3339(&date_str).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
    ).await.map_err(|e| {
        log::error!("Failed to fetch alerts: {}", e);
        Status::InternalServerError
    })?;

    // Count total alerts with same filters for pagination
    let total_count = db::alert::count_alerts(
        pool,
        status_ref,
        severity_ref,
        org_id,
        app_id,
    ).await.map_err(|e| {
        log::error!("Failed to fetch alert count: {}", e);
        Status::InternalServerError
    })?;

    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    let response = json!({
        "alerts": alerts,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    });

    Ok(response)
}

/// Get details of a specific alert including related data
#[get("/alerts/<id>")]
pub async fn get_alert(
    id: i64,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let alert_data = db::alert::get_alert_with_related_data(pool, id).await
        .map_err(|e| {
            log::error!("Failed to fetch alert {}: {}", id, e);
            if e.to_string().contains("no rows") {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })?;

    Ok(json!(alert_data))
}

/// Create a new alert
#[post("/alerts", format = "json", data = "<alert_data>")]
pub async fn create_alert(
    alert_data: Json<CreateAlertRequest>,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let data = alert_data.into_inner();
    
    let alert = db::alert::create_alert(
        pool,
        &data.alert_type,
        &data.severity,
        &data.service,
        &data.message,
        data.metadata,
        data.org_id,
        data.app_id,
        data.instance_id,
        data.region_id,
        data.node_id,
    ).await.map_err(|e| {
        log::error!("Failed to create alert: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "message": "Alert created successfully",
        "alert": alert
    }))
}

/// Update an alert's status
#[put("/alerts/<id>/status", format = "json", data = "<status_data>")]
pub async fn update_alert_status(
    id: i64,
    status_data: Json<UpdateAlertStatusRequest>,
    user: User, // Extract user from request guard
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let data = status_data.into_inner();
    
    // Validate the status is a valid value
    match data.status.as_str() {
        "active" | "acknowledged" | "resolved" | "auto_resolved" => {},
        _ => return Err(Status::BadRequest)
    }
    
    let user_id = user.id;

    let updated_alert = db::alert::update_alert_status(
        pool,
        id,
        &data.status,
        Some(user_id),
        data.notes.as_deref(),
    ).await.map_err(|e| {
        log::error!("Failed to update alert status: {}", e);
        if e.to_string().contains("no rows") {
            Status::NotFound
        } else {
            Status::InternalServerError
        }
    })?;

    Ok(json!({
        "message": "Alert status updated successfully",
        "alert": updated_alert
    }))
}

/// Acknowledge an alert
#[post("/alerts/<id>/acknowledge", format = "json", data = "<ack_data>")]
pub async fn acknowledge_alert(
    id: i64,
    ack_data: Json<AcknowledgeAlertRequest>,
    user: User, // Extract user from request guard
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let data = ack_data.into_inner();
    
    let acknowledgment = db::alert::acknowledge_alert(
        pool,
        id,
        user.id,
        data.notes.as_deref(),
        data.update_status,
    ).await.map_err(|e| {
        log::error!("Failed to acknowledge alert: {}", e);
        if e.to_string().contains("no rows") {
            Status::NotFound
        } else {
            Status::InternalServerError
        }
    })?;

    Ok(json!({
        "message": "Alert acknowledged successfully",
        "acknowledgment": acknowledgment
    }))
}

/// Resolve an alert
#[post("/alerts/<id>/resolve", format = "json", data = "<resolve_data>")]
pub async fn resolve_alert(
    id: i64,
    resolve_data: Option<Json<HashMap<String, String>>>,
    user: User, // Extract user from request guard
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // Extract notes if provided
    let notes = resolve_data
        .and_then(|data| data.get("notes").cloned());
    
    let resolved_alert = db::alert::resolve_alert(
        pool,
        id,
        user.id,
        notes.as_deref(),
    ).await.map_err(|e| {
        log::error!("Failed to resolve alert: {}", e);
        if e.to_string().contains("no rows") {
            Status::NotFound
        } else {
            Status::InternalServerError
        }
    })?;

    Ok(json!({
        "message": "Alert resolved successfully",
        "alert": resolved_alert
    }))
}

/// Create an escalation for an alert
#[post("/alerts/<id>/escalate", format = "json", data = "<escalation_data>")]
pub async fn escalate_alert(
    id: i64,
    escalation_data: Json<CreateEscalationRequest>,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let data = escalation_data.into_inner();
    
    let escalation = db::alert::create_alert_escalation(
        pool,
        id,
        data.escalation_level,
        data.escalated_to,
        &data.escalation_method,
        data.response_required_by,
    ).await.map_err(|e| {
        log::error!("Failed to escalate alert: {}", e);
        if e.to_string().contains("no rows") {
            Status::NotFound
        } else {
            Status::InternalServerError
        }
    })?;

    Ok(json!({
        "message": "Alert escalated successfully",
        "escalation": escalation
    }))
}

/// Get alerts for a specific application
#[get("/apps/<app_id>/alerts?<limit>&<include_resolved>")]
pub async fn get_app_alerts(
    app_id: i64,
    limit: Option<i64>,
    include_resolved: Option<bool>,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let limit = limit.unwrap_or(20);
    let include_resolved = include_resolved.unwrap_or(false);
    
    let alerts = db::alert::get_recent_app_alerts(
        pool,
        app_id,
        limit,
        include_resolved,
    ).await.map_err(|e| {
        log::error!("Failed to fetch app alerts: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({ "alerts": alerts }))
}

/// Get active alerts for an organization
#[get("/orgs/<org_id>/active-alerts?<limit>")]
pub async fn get_org_active_alerts(
    org_id: i64,
    limit: Option<i64>,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let limit = limit.unwrap_or(20);
    
    let alerts = db::alert::get_org_active_alerts(
        pool,
        org_id,
        limit,
    ).await.map_err(|e| {
        log::error!("Failed to fetch org active alerts: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({ "alerts": alerts }))
}

/// Get alert statistics for an organization
#[get("/orgs/<org_id>/alert-stats?<days>")]
pub async fn get_org_alert_stats(
    org_id: i64,
    days: Option<i64>,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let days = days.unwrap_or(30); // Default to last 30 days
    
    let stats = db::alert::get_alert_stats(
        pool,
        org_id,
        days,
    ).await.map_err(|e| {
        log::error!("Failed to fetch alert stats: {}", e);
        Status::InternalServerError
    })?;

    Ok(stats)
}

/// Get alerts needing escalation
#[get("/alerts/needing-escalation?<org_id>&<hours_threshold>")]
pub async fn get_alerts_needing_escalation(
    org_id: Option<i64>,
    hours_threshold: Option<i64>,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let hours_threshold = hours_threshold.unwrap_or(4); // Default to 4 hours
    
    let alerts = db::alert::get_alerts_needing_escalation(
        pool,
        org_id,
        hours_threshold,
    ).await.map_err(|e| {
        log::error!("Failed to fetch alerts needing escalation: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({ "alerts": alerts }))
}

/// Auto-resolve old alerts
#[post("/alerts/auto-resolve?<days_threshold>&<severity_level>")]
pub async fn auto_resolve_old_alerts(
    days_threshold: Option<i64>,
    severity_level: Option<Vec<String>>, // Can provide multiple severity levels
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let days_threshold = days_threshold.unwrap_or(7); // Default to 7 days
    
    // Convert Vec<String> to Vec<&str>
    let severity_refs: Option<Vec<&str>> = severity_level
        .as_ref()
        .map(|levels| levels.iter().map(AsRef::as_ref).collect());
    
    let count = db::alert::auto_resolve_old_alerts(
        pool,
        days_threshold,
        severity_refs,
    ).await.map_err(|e| {
        log::error!("Failed to auto-resolve old alerts: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "message": "Successfully auto-resolved old alerts",
        "count": count
    }))
}

/// Search for alerts
#[get("/alerts/search?<query>&<org_id>&<page>&<per_page>")]
pub async fn search_alerts(
    query: String,
    org_id: Option<i64>,
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(20);
    
    let alerts = db::alert::search_alerts(
        pool,
        &query,
        org_id,
        page,
        per_page,
    ).await.map_err(|e| {
        log::error!("Failed to search alerts: {}", e);
        Status::InternalServerError
    })?;
    
    let total_count = db::alert::count_search_alerts(
        pool,
        &query,
        org_id,
    ).await.map_err(|e| {
        log::error!("Failed to count search results: {}", e);
        Status::InternalServerError
    })?;
    
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    
    Ok(json!({
        "alerts": alerts,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    }))
}

/// Bulk update alert status
#[put("/alerts/bulk-status", format = "json", data = "<update_data>")]
pub async fn bulk_update_alert_status(
    update_data: Json<BulkUpdateStatusRequest>,
    user: User, // Extract user from request guard
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let data = update_data.into_inner();
    
    // Validate the status is a valid value
    match data.status.as_str() {
        "active" | "acknowledged" | "resolved" | "auto_resolved" => {},
        _ => return Err(Status::BadRequest)
    }
    
    // Validate that at least one filter is provided
    if data.ids.is_none() && data.service.is_none() && data.app_id.is_none() {
        return Err(Status::BadRequest);
    }

    let count = db::alert::bulk_update_alert_status(
        pool,
        data.ids,
        data.service.as_deref(),
        data.app_id,
        &data.status,
        user.id, // Use user.id instead of user_id
        data.notes.as_deref(),
    ).await.map_err(|e| {
        log::error!("Failed to bulk update alert status: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "message": "Successfully updated alert status",
        "count": count
    }))
}