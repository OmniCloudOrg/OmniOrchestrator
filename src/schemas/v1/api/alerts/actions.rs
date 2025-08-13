use super::super::super::db::queries as db;
use super::types::{AcknowledgeAlertRequest, CreateEscalationRequest};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{post, State};
use std::collections::HashMap;
use std::sync::Arc;
use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::user::User;

/// Acknowledge an alert
#[post("/platform/<platform_id>/alerts/<id>/acknowledge", format = "json", data = "<ack_data>")]
pub async fn acknowledge_alert(
    platform_id: i64,
    id: i64,
    ack_data: Json<AcknowledgeAlertRequest>,
    user: User, // Extract user from request guard
    db_manager: &State<Arc<DatabaseManager>>,
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

    let data = ack_data.into_inner();
    
    let acknowledgment = match db::alert::acknowledge_alert(
        &pool,
        id,
        user.id,
        data.notes.as_deref(),
        data.update_status,
    ).await {
        Ok(ack) => ack,
        Err(e) => {
            log::error!("Failed to acknowledge alert: {}", e);
            return Err((
                if e.to_string().contains("no rows") {
                    Status::NotFound
                } else {
                    Status::InternalServerError
                },
                Json(json!({
                    "error": if e.to_string().contains("no rows") { "Alert not found" } else { "Database error" },
                    "message": if e.to_string().contains("no rows") { 
                        format!("Alert with ID {} does not exist", id) 
                    } else { 
                        "Failed to acknowledge alert".to_string() 
                    }
                }))
            ));
        }
    };

    Ok(Json(json!({
        "message": "Alert acknowledged successfully",
        "acknowledgment": acknowledgment
    })))
}

/// Resolve an alert
#[post("/platform/<platform_id>/alerts/<id>/resolve", format = "json", data = "<resolve_data>")]
pub async fn resolve_alert(
    platform_id: i64,
    id: i64,
    resolve_data: Option<Json<HashMap<String, String>>>,
    user: User, // Extract user from request guard
    db_manager: &State<Arc<DatabaseManager>>,
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

    // Extract notes if provided
    let notes = resolve_data
        .and_then(|data| data.get("notes").cloned());
    
    let resolved_alert = match db::alert::resolve_alert(
        &pool,
        id,
        user.id,
        notes.as_deref(),
    ).await {
        Ok(alert) => alert,
        Err(e) => {
            log::error!("Failed to resolve alert: {}", e);
            return Err((
                if e.to_string().contains("no rows") {
                    Status::NotFound
                } else {
                    Status::InternalServerError
                },
                Json(json!({
                    "error": if e.to_string().contains("no rows") { "Alert not found" } else { "Database error" },
                    "message": if e.to_string().contains("no rows") { 
                        format!("Alert with ID {} does not exist", id) 
                    } else { 
                        "Failed to resolve alert".to_string() 
                    }
                }))
            ));
        }
    };

    Ok(Json(json!({
        "message": "Alert resolved successfully",
        "alert": resolved_alert
    })))
}

/// Create an escalation for an alert
#[post("/platform/<platform_id>/alerts/<id>/escalate", format = "json", data = "<escalation_data>")]
pub async fn escalate_alert(
    platform_id: i64,
    id: i64,
    escalation_data: Json<CreateEscalationRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
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

    let data = escalation_data.into_inner();
    
    let escalation = match db::alert::create_alert_escalation(
        &pool,
        id,
        data.escalation_level,
        data.escalated_to,
        &data.escalation_method,
        data.response_required_by,
    ).await {
        Ok(esc) => esc,
        Err(e) => {
            log::error!("Failed to escalate alert: {}", e);
            return Err((
                if e.to_string().contains("no rows") {
                    Status::NotFound
                } else {
                    Status::InternalServerError
                },
                Json(json!({
                    "error": if e.to_string().contains("no rows") { "Alert not found" } else { "Database error" },
                    "message": if e.to_string().contains("no rows") { 
                        format!("Alert with ID {} does not exist", id) 
                    } else { 
                        "Failed to escalate alert".to_string() 
                    }
                }))
            ));
        }
    };

    Ok(Json(json!({
        "message": "Alert escalated successfully",
        "escalation": escalation
    })))
}