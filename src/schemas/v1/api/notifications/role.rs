use std::sync::Arc;
use crate::DatabaseManager;
use crate::schemas::v1::db::queries::{self as db};
use super::types::CreateRoleNotificationRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, post, State};

use libomni::types::db::v1 as types;
use types::user::User;

/// Get a paginated list of role notifications
#[get("/platform/<platform_id>/notifications/role/<role_id>?<page>&<per_page>")]
pub async fn list_role_notifications(
    platform_id: i64,
    role_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    user: User, // For authentication
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

    // Authorization - only users with the role or administrators can view role notifications
    // This would require a check against user roles from your auth system
    // if !user.roles.contains(&"admin".to_string()) {
    //     // In a real implementation, we'd check if the user has the specific role
    //     // For this example, we'll use a simplified check
    //     return Err((
    //         Status::Forbidden,
    //         Json(json!({
    //             "error": "Forbidden",
    //             "message": "You do not have permission to view notifications for this role"
    //         }))
    //     ));
    // }

    // Default pagination parameters
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(20);

    match db::notification::list_role_notifications(
        &pool,
        role_id,
        page,
        per_page,
    ).await {
        Ok(notifications) => Ok(Json(json!({
            "notifications": notifications,
            "pagination": {
                "page": page,
                "per_page": per_page
            }
        }))),
        Err(e) => {
            log::error!("Failed to fetch role notifications: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch role notifications"
                }))
            ))
        }
    }
}

/// Create a new notification for a role
#[post("/platform/<platform_id>/notifications/role", format = "json", data = "<notification_data>")]
pub async fn create_role_notification(
    platform_id: i64,
    notification_data: Json<CreateRoleNotificationRequest>,
    user: User, // For authentication
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

    // Only administrators and certain roles can create notifications
    // if !user.roles.contains(&"admin".to_string()) && !user.roles.contains(&"notifier".to_string()) {
    //     return Err((
    //         Status::Forbidden,
    //         Json(json!({
    //             "error": "Forbidden",
    //             "message": "You do not have permission to create role notifications"
    //         }))
    //     ));
    // }

    let data = notification_data.into_inner();

    match db::notification::create_role_notification(
        &pool,
        data.role_id,
        &data.message,
        &data.notification_type,
        data.org_id,
        data.app_id,
        data.importance.as_deref(),
        data.action_url.as_deref(),
        data.action_label.as_deref(),
        data.expires_at,
    ).await {
        Ok(notification) => Ok(Json(json!({
            "message": "Role notification created successfully",
            "notification": notification
        }))),
        Err(e) => {
            log::error!("Failed to create role notification: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to create role notification"
                }))
            ))
        }
    }
}