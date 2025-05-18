//! Notification management module for handling CRUD operations on notifications.
//! 
//! This module provides functionality to create, read, update, and delete
//! notifications in the system. It includes endpoints for managing both
//! user notifications and role-based notifications.

use std::sync::Arc;
use crate::DatabaseManager;
use crate::models::{
    user::User,
    notification::{
        NotificationAcknowledgment,
        NotificationWithCount,
        RoleNotification,
        UserNotification,
        UserNotificationWithRoleNotifications
    },
};
use crate::schemas::v1::db::queries::{self as db};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put, State};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Request and response structs

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserNotificationRequest {
    pub message: String,
    pub notification_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleNotificationRequest {
    pub role_id: i64,
    pub message: String,
    pub notification_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgeNotificationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_notification_id: Option<i64>,
}

// API Routes

/// Get a paginated list of notifications for a user
#[get("/platform/<platform_id>/notifications/user/<user_id>?<page>&<per_page>&<include_read>")]
pub async fn list_user_notifications(
    platform_id: i64,
    user_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    include_read: Option<bool>,
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

    // Authorization - only allow users to see their own notifications
    // or administrators to see others' notifications
    if user.id != user_id {
        return Err((
            Status::Forbidden,
            Json(json!({
                "error": "Forbidden",
                "message": "You do not have permission to view this user's notifications"
            }))
        ));
    }

    // Default pagination parameters
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(20);
    let include_read = include_read.unwrap_or(false);

    match db::notification::list_user_notifications(
        &pool,
        user_id,
        page,
        per_page,
        include_read,
    ).await {
        Ok(notifications) => Ok(Json(json!({
            "notifications": notifications,
            "pagination": {
                "page": page,
                "per_page": per_page
            }
        }))),
        Err(e) => {
            log::error!("Failed to fetch user notifications: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch user notifications"
                }))
            ))
        }
    }
}

/// Count unread notifications for a user (for badges)
#[get("/platform/<platform_id>/notifications/user/count/<user_id>")]
pub async fn count_unread_user_notifications(
    platform_id: i64,
    user_id: i64,
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

    // Authorization - only allow users to see their own count
    // or administrators to see others' counts
    if user.id != user_id {
        return Err((
            Status::Forbidden,
            Json(json!({
                "error": "Forbidden",
                "message": "You do not have permission to view this user's notification count"
            }))
        ));
    }

    match db::notification::count_unread_user_notifications(
        &pool,
        user_id,
    ).await {
        Ok(count) => Ok(Json(json!({ "unread_count": count }))),
        Err(e) => {
            log::error!("Failed to count unread notifications: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count unread notifications"
                }))
            ))
        }
    }
}

/// Get a specific notification by ID
#[get("/platform/<platform_id>/notifications/<id>")]
pub async fn get_user_notification_by_id(
    platform_id: i64,
    id: i64,
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

    let notification = match db::notification::get_user_notification_by_id(
        &pool,
        id,
    ).await {
        Ok(notification) => notification,
        Err(e) => {
            log::error!("Failed to fetch notification: {}", e);
            if e.to_string().contains("no rows") {
                return Err((
                    Status::NotFound,
                    Json(json!({
                        "error": "Not found",
                        "message": format!("Notification with ID {} does not exist", id)
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

    // Authorization - only allow users to see their own notifications
    // or administrators to see others' notifications
    if notification.user_id != user.id {
        return Err((
            Status::Forbidden,
            Json(json!({
                "error": "Forbidden",
                "message": "You do not have permission to view this notification"
            }))
        ));
    }

    Ok(Json(json!({ "notification": notification })))
}

/// Create a new notification for a user
#[post("/platform/<platform_id>/notifications/user", format = "json", data = "<notification_data>")]
pub async fn create_user_notification(
    platform_id: i64,
    notification_data: Json<CreateUserNotificationRequest>,
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
    //             "message": "You do not have permission to create notifications"
    //         }))
    //     ));
    // }

    let data = notification_data.into_inner();
    
    // Target user ID would normally come from the request
    // For this example, we're using the authenticated user's ID
    let target_user_id = user.id;

    match db::notification::create_user_notification(
        &pool,
        target_user_id,
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
            "message": "Notification created successfully",
            "notification": notification
        }))),
        Err(e) => {
            log::error!("Failed to create notification: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to create notification"
                }))
            ))
        }
    }
}

/// Mark a notification as read
#[put("/platform/<platform_id>/notifications/<id>/read")]
pub async fn mark_user_notification_as_read(
    platform_id: i64,
    id: i64,
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

    // First, get the notification to check ownership
    let notification = match db::notification::get_user_notification_by_id(
        &pool,
        id,
    ).await {
        Ok(notification) => notification,
        Err(e) => {
            log::error!("Failed to fetch notification: {}", e);
            if e.to_string().contains("no rows") {
                return Err((
                    Status::NotFound,
                    Json(json!({
                        "error": "Not found",
                        "message": format!("Notification with ID {} does not exist", id)
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

    // Authorization - only allow users to mark their own notifications as read
    // or administrators to mark others' notifications
    if notification.user_id != user.id {
        return Err((
            Status::Forbidden,
            Json(json!({
                "error": "Forbidden",
                "message": "You do not have permission to mark this notification as read"
            }))
        ));
    }

    match db::notification::mark_user_notification_as_read(
        &pool,
        id,
    ).await {
        Ok(updated_notification) => Ok(Json(json!({
            "message": "Notification marked as read",
            "notification": updated_notification
        }))),
        Err(e) => {
            log::error!("Failed to mark notification as read: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to mark notification as read"
                }))
            ))
        }
    }
}

/// Mark all notifications for a user as read
#[put("/platform/<platform_id>/notifications/user/<user_id>/read-all")]
pub async fn mark_all_user_notifications_as_read(
    platform_id: i64,
    user_id: i64,
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

    // Authorization - only allow users to mark their own notifications as read
    // or administrators to mark others' notifications
    if user.id != user_id {
        return Err((
            Status::Forbidden,
            Json(json!({
                "error": "Forbidden",
                "message": "You do not have permission to mark this user's notifications as read"
            }))
        ));
    }

    match db::notification::mark_all_user_notifications_as_read(
        &pool,
        user_id,
    ).await {
        Ok(_) => Ok(Json(json!({
            "message": "All notifications marked as read",
        }))),
        Err(e) => {
            log::error!("Failed to mark all notifications as read: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to mark all notifications as read"
                }))
            ))
        }
    }
}

/// Delete a notification
#[delete("/platform/<platform_id>/notifications/<id>")]
pub async fn delete_user_notification(
    platform_id: i64,
    id: i64,
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

    // First, get the notification to check ownership
    let notification = match db::notification::get_user_notification_by_id(
        &pool,
        id,
    ).await {
        Ok(notification) => notification,
        Err(e) => {
            log::error!("Failed to fetch notification: {}", e);
            if e.to_string().contains("no rows") {
                return Err((
                    Status::NotFound,
                    Json(json!({
                        "error": "Not found",
                        "message": format!("Notification with ID {} does not exist", id)
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

    // Authorization - only allow users to delete their own notifications
    // or administrators to delete others' notifications
    if notification.user_id != user.id {
        return Err((
            Status::Forbidden,
            Json(json!({
                "error": "Forbidden",
                "message": "You do not have permission to delete this notification"
            }))
        ));
    }

    match db::notification::delete_user_notification(
        &pool,
        id,
    ).await {
        Ok(_) => Ok(Json(json!({
            "message": "Notification deleted successfully",
        }))),
        Err(e) => {
            log::error!("Failed to delete notification: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to delete notification"
                }))
            ))
        }
    }
}

/// Delete all read notifications for a user
#[delete("/platform/<platform_id>/notifications/user/<user_id>/read")]
pub async fn delete_read_user_notifications(
    platform_id: i64,
    user_id: i64,
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

    // Authorization - only allow users to delete their own notifications
    // or administrators to delete others' notifications
    if user.id != user_id {
        return Err((
            Status::Forbidden,
            Json(json!({
                "error": "Forbidden",
                "message": "You do not have permission to delete this user's notifications"
            }))
        ));
    }

    match db::notification::delete_read_user_notifications(
        &pool,
        user_id,
    ).await {
        Ok(count) => Ok(Json(json!({
            "message": "Read notifications deleted successfully",
            "count": count
        }))),
        Err(e) => {
            log::error!("Failed to delete read notifications: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to delete read notifications"
                }))
            ))
        }
    }
}

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

/// Get all notifications for a user including role notifications
#[get("/platform/<platform_id>/notifications/user/<user_id>/all?<page>&<per_page>")]
pub async fn get_all_user_notifications_with_count(
    platform_id: i64,
    user_id: i64,
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

    // Authorization - only allow users to see their own notifications
    // or administrators to see others' notifications
    if user.id != user_id {
        return Err((
            Status::Forbidden,
            Json(json!({
                "error": "Forbidden",
                "message": "You do not have permission to view this user's notifications"
            }))
        ));
    }

    // Default pagination parameters
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(20);

    match db::notification::get_all_user_notifications_with_count(
        &pool,
        user_id,
        page,
        per_page,
    ).await {
        Ok(notifications_with_count) => Ok(Json(json!(notifications_with_count))),
        Err(e) => {
            log::error!("Failed to fetch notifications with count: {}", e);
            Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch notifications with count"
                }))
            ))
        }
    }
}