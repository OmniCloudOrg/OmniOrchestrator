//! Notification management module for handling CRUD operations on notifications.
//! 
//! This module provides functionality to create, read, update, and delete
//! notifications in the system. It includes endpoints for managing both
//! user notifications and role-based notifications.

use crate::db::tables::{
    UserNotification, RoleNotification, NotificationAcknowledgment,
    NotificationWithCount, UserNotificationWithRoleNotifications
};
use crate::db::v1::queries as db;
use crate::api::auth::User;
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
#[get("/notifications/user/<user_id>?<page>&<per_page>&<include_read>")]
pub async fn list_user_notifications(
    user_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    include_read: Option<bool>,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // Authorization - only allow users to see their own notifications
    // or administrators to see others' notifications
    if user.id != user_id && !user.roles.contains(&"admin".to_string()) {
        return Err(Status::Forbidden);
    }

    // Default pagination parameters
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(20);
    let include_read = include_read.unwrap_or(false);

    let notifications = db::notification::list_user_notifications(
        pool,
        user_id,
        page,
        per_page,
        include_read,
    ).await.map_err(|e| {
        log::error!("Failed to fetch user notifications: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "notifications": notifications,
        "pagination": {
            "page": page,
            "per_page": per_page
        }
    }))
}

/// Count unread notifications for a user (for badges)
#[get("/notifications/user/<user_id>/count")]
pub async fn count_unread_user_notifications(
    user_id: i64,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // Authorization - only allow users to see their own count
    // or administrators to see others' counts
    if user.id != user_id && !user.roles.contains(&"admin".to_string()) {
        return Err(Status::Forbidden);
    }

    let count = db::notification::count_unread_user_notifications(
        pool,
        user_id,
    ).await.map_err(|e| {
        log::error!("Failed to count unread notifications: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({ "unread_count": count }))
}

/// Get a specific notification by ID
#[get("/notifications/<id>")]
pub async fn get_user_notification_by_id(
    id: i64,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let notification = db::notification::get_user_notification_by_id(
        pool,
        id,
    ).await.map_err(|e| {
        log::error!("Failed to fetch notification: {}", e);
        if e.to_string().contains("no rows") {
            Status::NotFound
        } else {
            Status::InternalServerError
        }
    })?;

    // Authorization - only allow users to see their own notifications
    // or administrators to see others' notifications
    if notification.user_id != user.id && !user.roles.contains(&"admin".to_string()) {
        return Err(Status::Forbidden);
    }

    Ok(json!({ "notification": notification }))
}

/// Create a new notification for a user
#[post("/notifications/user", format = "json", data = "<notification_data>")]
pub async fn create_user_notification(
    notification_data: Json<CreateUserNotificationRequest>,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // Only administrators and certain roles can create notifications
    if !user.roles.contains(&"admin".to_string()) && !user.roles.contains(&"notifier".to_string()) {
        return Err(Status::Forbidden);
    }

    let data = notification_data.into_inner();
    
    // Target user ID would normally come from the request
    // For this example, we're using the authenticated user's ID
    let target_user_id = user.id;

    let notification = db::notification::create_user_notification(
        pool,
        target_user_id,
        &data.message,
        &data.notification_type,
        data.org_id,
        data.app_id,
        data.importance.as_deref(),
        data.action_url.as_deref(),
        data.action_label.as_deref(),
        data.expires_at,
    ).await.map_err(|e| {
        log::error!("Failed to create notification: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "message": "Notification created successfully",
        "notification": notification
    }))
}

/// Mark a notification as read
#[put("/notifications/<id>/read")]
pub async fn mark_user_notification_as_read(
    id: i64,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // First, get the notification to check ownership
    let notification = db::notification::get_user_notification_by_id(
        pool,
        id,
    ).await.map_err(|e| {
        log::error!("Failed to fetch notification: {}", e);
        if e.to_string().contains("no rows") {
            Status::NotFound
        } else {
            Status::InternalServerError
        }
    })?;

    // Authorization - only allow users to mark their own notifications as read
    // or administrators to mark others' notifications
    if notification.user_id != user.id && !user.roles.contains(&"admin".to_string()) {
        return Err(Status::Forbidden);
    }

    let updated_notification = db::notification::mark_user_notification_as_read(
        pool,
        id,
    ).await.map_err(|e| {
        log::error!("Failed to mark notification as read: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "message": "Notification marked as read",
        "notification": updated_notification
    }))
}

/// Mark all notifications for a user as read
#[put("/notifications/user/<user_id>/read-all")]
pub async fn mark_all_user_notifications_as_read(
    user_id: i64,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // Authorization - only allow users to mark their own notifications as read
    // or administrators to mark others' notifications
    if user.id != user_id && !user.roles.contains(&"admin".to_string()) {
        return Err(Status::Forbidden);
    }

    db::notification::mark_all_user_notifications_as_read(
        pool,
        user_id,
    ).await.map_err(|e| {
        log::error!("Failed to mark all notifications as read: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "message": "All notifications marked as read",
    }))
}

/// Delete a notification
#[delete("/notifications/<id>")]
pub async fn delete_user_notification(
    id: i64,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // First, get the notification to check ownership
    let notification = db::notification::get_user_notification_by_id(
        pool,
        id,
    ).await.map_err(|e| {
        log::error!("Failed to fetch notification: {}", e);
        if e.to_string().contains("no rows") {
            Status::NotFound
        } else {
            Status::InternalServerError
        }
    })?;

    // Authorization - only allow users to delete their own notifications
    // or administrators to delete others' notifications
    if notification.user_id != user.id && !user.roles.contains(&"admin".to_string()) {
        return Err(Status::Forbidden);
    }

    db::notification::delete_user_notification(
        pool,
        id,
    ).await.map_err(|e| {
        log::error!("Failed to delete notification: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "message": "Notification deleted successfully",
    }))
}

/// Delete all read notifications for a user
#[delete("/notifications/user/<user_id>/read")]
pub async fn delete_read_user_notifications(
    user_id: i64,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // Authorization - only allow users to delete their own notifications
    // or administrators to delete others' notifications
    if user.id != user_id && !user.roles.contains(&"admin".to_string()) {
        return Err(Status::Forbidden);
    }

    let count = db::notification::delete_read_user_notifications(
        pool,
        user_id,
    ).await.map_err(|e| {
        log::error!("Failed to delete read notifications: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "message": "Read notifications deleted successfully",
        "count": count
    }))
}

/// Get a paginated list of role notifications
#[get("/notifications/role/<role_id>?<page>&<per_page>")]
pub async fn list_role_notifications(
    role_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // Authorization - only users with the role or administrators can view role notifications
    // This would require a check against user roles from your auth system
    if !user.roles.contains(&"admin".to_string()) {
        // In a real implementation, you'd check if the user has the specific role
        // For this example, we'll use a simplified check
        return Err(Status::Forbidden);
    }

    // Default pagination parameters
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(20);

    let notifications = db::notification::list_role_notifications(
        pool,
        role_id,
        page,
        per_page,
    ).await.map_err(|e| {
        log::error!("Failed to fetch role notifications: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "notifications": notifications,
        "pagination": {
            "page": page,
            "per_page": per_page
        }
    }))
}

/// Create a new notification for a role
#[post("/notifications/role", format = "json", data = "<notification_data>")]
pub async fn create_role_notification(
    notification_data: Json<CreateRoleNotificationRequest>,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // Only administrators and certain roles can create notifications
    if !user.roles.contains(&"admin".to_string()) && !user.roles.contains(&"notifier".to_string()) {
        return Err(Status::Forbidden);
    }

    let data = notification_data.into_inner();

    let notification = db::notification::create_role_notification(
        pool,
        data.role_id,
        &data.message,
        &data.notification_type,
        data.org_id,
        data.app_id,
        data.importance.as_deref(),
        data.action_url.as_deref(),
        data.action_label.as_deref(),
        data.expires_at,
    ).await.map_err(|e| {
        log::error!("Failed to create role notification: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "message": "Role notification created successfully",
        "notification": notification
    }))
}

/// Acknowledge a notification
#[post("/notifications/acknowledge", format = "json", data = "<ack_data>")]
pub async fn acknowledge_notification(
    ack_data: Json<AcknowledgeNotificationRequest>,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    let data = ack_data.into_inner();

    // Validate input - either notification_id or role_notification_id must be provided
    if data.notification_id.is_none() && data.role_notification_id.is_none() {
        return Err(Status::BadRequest);
    }
    if data.notification_id.is_some() && data.role_notification_id.is_some() {
        return Err(Status::BadRequest);
    }

    // If it's a user notification, verify ownership
    if let Some(notification_id) = data.notification_id {
        let notification = db::notification::get_user_notification_by_id(
            pool,
            notification_id,
        ).await.map_err(|e| {
            log::error!("Failed to fetch notification: {}", e);
            if e.to_string().contains("no rows") {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })?;

        // Authorization - only allow users to acknowledge their own notifications
        // or administrators to acknowledge others' notifications
        if notification.user_id != user.id && !user.roles.contains(&"admin".to_string()) {
            return Err(Status::Forbidden);
        }
    }

    let acknowledgment = db::notification::create_notification_acknowledgment(
        pool,
        user.id,
        data.notification_id,
        data.role_notification_id,
    ).await.map_err(|e| {
        log::error!("Failed to acknowledge notification: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!({
        "message": "Notification acknowledged successfully",
        "acknowledgment": acknowledgment
    }))
}

/// Get all notifications for a user including role notifications
#[get("/notifications/user/<user_id>/all?<page>&<per_page>")]
pub async fn get_all_user_notifications_with_count(
    user_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    user: User, // For authentication
    pool: &State<sqlx::MySqlPool>,
) -> Result<Value, Status> {
    // Authorization - only allow users to see their own notifications
    // or administrators to see others' notifications
    if user.id != user_id && !user.roles.contains(&"admin".to_string()) {
        return Err(Status::Forbidden);
    }

    // Default pagination parameters
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(20);

    let notifications_with_count = db::notification::get_all_user_notifications_with_count(
        pool,
        user_id,
        page,
        per_page,
    ).await.map_err(|e| {
        log::error!("Failed to fetch notifications with count: {}", e);
        Status::InternalServerError
    })?;

    Ok(json!(notifications_with_count))
}