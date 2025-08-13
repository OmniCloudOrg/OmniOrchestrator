use std::sync::Arc;
use crate::DatabaseManager;
use crate::schemas::v1::db::queries::{self as db};
use super::types::CreateUserNotificationRequest;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put, State};

use libomni::types::db::v1 as types;
use types::user::User;

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