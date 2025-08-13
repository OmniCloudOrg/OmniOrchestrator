use std::sync::Arc;
use crate::DatabaseManager;
use crate::schemas::v1::db::queries::{self as db};
use super::types::AcknowledgeNotificationRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{post, State};

use libomni::types::db::v1 as types;
use types::user::User;

/// Acknowledge a notification
#[post("/platform/<platform_id>/notifications/acknowledge", format = "json", data = "<ack_data>")]
pub async fn acknowledge_notification(
    platform_id: i64,
    ack_data: Json<AcknowledgeNotificationRequest>,
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

    let data = ack_data.into_inner();

    // Validate input - either notification_id or role_notification_id must be provided
    if data.notification_id.is_none() && data.role_notification_id.is_none() {
        return Err((
            Status::BadRequest,
            Json(json!({
                "error": "Bad request",
                "message": "Either notification_id or role_notification_id must be provided"
            }))
        ));
    }
    if data.notification_id.is_some() && data.role_notification_id.is_some() {
        return Err((
            Status::BadRequest,
            Json(json!({
                "error": "Bad request",
                "message": "Only one of notification_id or role_notification_id should be provided"
            }))
        ));
    }

    // If it's a user notification, verify ownership
    if let Some(notification_id) = data.notification_id {
        let notification = match db::notification::get_user_notification_by_id(
            &pool,
            notification_id,
        ).await {
            Ok(notification) => notification,
            Err(e) => {
                log::error!("Failed to fetch notification: {}", e);
                if e.to_string().contains("no rows") {
                    return Err((
                        Status::NotFound,
                        Json(json!({
                            "error": "Not found",
                            "message": format!("Notification with ID {} does not exist", notification_id)
                        }))
                    ));
                } else {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to fetch notification"
                        }))
                    ));
                }
            }
        };

        // Authorization - only allow users to acknowledge their own notifications
        // or administrators to acknowledge others' notifications
        if notification.user_id != user.id {
            return Err((
                Status::Forbidden,
                Json(json!({
                    "error": "Forbidden",
                    "message": "You do not have permission to acknowledge this notification"
                }))
            ));
        }
    }

    match db::notification::create_notification_acknowledgment(
        &pool,
        user.id,
        data.notification_id,
        data.role_notification_id,
    ).await {
        Ok(acknowledgment) => Ok(Json(json!({
            "message": "Notification acknowledged successfully",
            "acknowledgment": acknowledgment
        }))),
        Err(e) => {
            log::error!("Failed to acknowledge notification: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to acknowledge notification"
                }))
            ))
        }
    }
}