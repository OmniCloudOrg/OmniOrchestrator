use super::super::tables::{
    Notification, NotificationWithCount, UserNotification, RoleNotification, 
    NotificationAcknowledgment, UserNotificationWithRoleNotifications
};
use anyhow::Context;
use serde::Serialize;
use sqlx::{MySql, Pool};

// =================== User Notifications ===================

/// Retrieves a paginated list of notifications for a specific user.
///
/// This function fetches a subset of notifications for a given user based on pagination
/// parameters, ordering them by creation date in descending order (newest first).
/// This helps manage large notification lists by retrieving only a specific "page" of results.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - ID of the user whose notifications to retrieve
/// * `page` - Zero-based page number (e.g., 0 for first page, 1 for second page)
/// * `per_page` - Number of records to fetch per page
/// * `include_read` - Whether to include notifications that have been marked as read
///
/// # Returns
///
/// * `Ok(Vec<UserNotification>)` - Successfully retrieved list of notifications
/// * `Err(anyhow::Error)` - Failed to fetch notifications, with context
pub async fn list_user_notifications(
    pool: &Pool<MySql>,
    user_id: i64,
    page: i64,
    per_page: i64,
    include_read: bool,
) -> anyhow::Result<Vec<UserNotification>> {
    println!("Attempting to fetch user notifications from database...");

    // Build query based on whether to include read notifications
    let query = if include_read {
        r#"
        SELECT * FROM user_notifications
        WHERE user_id = ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#
    } else {
        r#"
        SELECT * FROM user_notifications
        WHERE user_id = ? AND read_status = FALSE
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#
    };

    let result = sqlx::query_as::<_, UserNotification>(query)
        .bind(user_id)
        .bind(per_page)
        .bind(page * per_page)
        .fetch_all(pool)
        .await;

    match result {
        Ok(notifications) => {
            println!(
                "Successfully fetched {} user notifications",
                notifications.len()
            );
            Ok(notifications)
        }
        Err(e) => {
            eprintln!("Error fetching user notifications: {:#?}", e);
            Err(anyhow::Error::new(e).context("Failed to fetch user notifications"))
        }
    }
}

/// Counts unread notifications for a user.
///
/// This function retrieves the count of unread notifications for a specific user,
/// which is useful for displaying notification badges or alerts.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - ID of the user whose unread notifications to count
///
/// # Returns
///
/// * `Ok(i64)` - Successfully retrieved count of unread notifications
/// * `Err(anyhow::Error)` - Failed to count notifications
pub async fn count_unread_user_notifications(
    pool: &Pool<MySql>,
    user_id: i64,
) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM user_notifications WHERE user_id = ? AND read_status = FALSE",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .context("Failed to count unread user notifications")?;

    Ok(count)
}

/// Retrieves a specific user notification by its unique identifier.
///
/// This function fetches a single user notification record matching the provided ID.
/// It's typically used for retrieving detailed information about a specific notification.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the notification to retrieve
///
/// # Returns
///
/// * `Ok(UserNotification)` - Successfully retrieved notification
/// * `Err(anyhow::Error)` - Failed to fetch notification (including if not found)
pub async fn get_user_notification_by_id(
    pool: &Pool<MySql>,
    id: i64,
) -> anyhow::Result<UserNotification> {
    let notification = sqlx::query_as::<_, UserNotification>(
        "SELECT * FROM user_notifications WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch user notification")?;

    Ok(notification)
}

/// Creates a new notification for a user.
///
/// This function inserts a new user notification record with the provided parameters.
/// It handles both required fields and optional fields.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - ID of the user to notify
/// * `message` - The notification message text
/// * `notification_type` - Type of notification (info, warning, error, success)
/// * `org_id` - Optional organization ID related to the notification
/// * `app_id` - Optional application ID related to the notification
/// * `importance` - Optional importance level (default is "normal")
/// * `action_url` - Optional URL for a related action
/// * `action_label` - Optional label for the action button
/// * `expires_at` - Optional expiration date for the notification
///
/// # Returns
///
/// * `Ok(UserNotification)` - Successfully created notification, including database-assigned fields
/// * `Err(anyhow::Error)` - Failed to create notification
pub async fn create_user_notification(
    pool: &Pool<MySql>,
    user_id: i64,
    message: &str,
    notification_type: &str,
    org_id: Option<i64>,
    app_id: Option<i64>,
    importance: Option<&str>,
    action_url: Option<&str>,
    action_label: Option<&str>,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
) -> anyhow::Result<UserNotification> {
    // Begin transaction
    let mut tx = pool.begin().await?;

    let notification = sqlx::query_as::<_, UserNotification>(
        r#"INSERT INTO user_notifications (
            user_id, org_id, app_id, notification_type, message, 
            importance, action_url, action_label, created_at, expires_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, ?)"#,
    )
    .bind(user_id)
    .bind(org_id)
    .bind(app_id)
    .bind(notification_type)
    .bind(message)
    .bind(importance.unwrap_or("normal"))
    .bind(action_url)
    .bind(action_label)
    .bind(expires_at)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create user notification")?;

    // Commit transaction
    tx.commit().await?;

    // Return newly created notification
    Ok(notification)
}

/// Marks a user notification as read.
///
/// This function updates the read_status of a user notification to indicate
/// that the user has viewed or acknowledged it.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the notification to mark as read
///
/// # Returns
///
/// * `Ok(UserNotification)` - Successfully updated notification
/// * `Err(anyhow::Error)` - Failed to update notification
pub async fn mark_user_notification_as_read(
    pool: &Pool<MySql>,
    id: i64,
) -> anyhow::Result<UserNotification> {
    let mut tx = pool.begin().await?;

    let notification = sqlx::query_as::<_, UserNotification>(
        "UPDATE user_notifications SET read_status = TRUE WHERE id = ?",
    )
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to mark user notification as read")?;

    tx.commit().await?;
    Ok(notification)
}

/// Marks all notifications for a user as read.
///
/// This function updates the read_status of all notifications for a specific user
/// to indicate that the user has viewed or acknowledged them.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - ID of the user whose notifications to mark as read
///
/// # Returns
///
/// * `Ok(())` - Successfully updated notifications
/// * `Err(anyhow::Error)` - Failed to update notifications
pub async fn mark_all_user_notifications_as_read(
    pool: &Pool<MySql>,
    user_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "UPDATE user_notifications SET read_status = TRUE WHERE user_id = ? AND read_status = FALSE",
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .context("Failed to mark all user notifications as read")?;

    tx.commit().await?;
    Ok(())
}

/// Deletes a user notification.
///
/// This function permanently removes a user notification record with the specified ID.
/// The operation is performed within a transaction to ensure data consistency.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the notification to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the notification
/// * `Err(anyhow::Error)` - Failed to delete the notification
pub async fn delete_user_notification(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM user_notifications WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete user notification")?;

    tx.commit().await?;
    Ok(())
}

/// Deletes all read notifications for a user.
///
/// This function permanently removes all notifications that have been marked as read
/// for a specific user. This can be used as a "clear all" feature to help users
/// maintain a clean notification list.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - ID of the user whose read notifications should be deleted
///
/// # Returns
///
/// * `Ok(i64)` - Number of notifications deleted
/// * `Err(anyhow::Error)` - Failed to delete notifications
pub async fn delete_read_user_notifications(
    pool: &Pool<MySql>,
    user_id: i64,
) -> anyhow::Result<i64> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query("DELETE FROM user_notifications WHERE user_id = ? AND read_status = TRUE")
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete read user notifications")?;

    tx.commit().await?;
    Ok(result.rows_affected() as i64)
}

// =================== Role Notifications ===================

/// Retrieves a paginated list of role notifications.
///
/// This function fetches a subset of role notifications based on pagination parameters,
/// ordering them by creation date in descending order (newest first).
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `role_id` - ID of the role whose notifications to retrieve
/// * `page` - Zero-based page number
/// * `per_page` - Number of records to fetch per page
///
/// # Returns
///
/// * `Ok(Vec<RoleNotification>)` - Successfully retrieved list of notifications
/// * `Err(anyhow::Error)` - Failed to fetch notifications
pub async fn list_role_notifications(
    pool: &Pool<MySql>,
    role_id: i64,
    page: i64,
    per_page: i64,
) -> anyhow::Result<Vec<RoleNotification>> {
    let notifications = sqlx::query_as::<_, RoleNotification>(
        r#"
        SELECT * FROM role_notifications
        WHERE role_id = ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(role_id)
    .bind(per_page)
    .bind(page * per_page)
    .fetch_all(pool)
    .await
    .context("Failed to fetch role notifications")?;

    Ok(notifications)
}

/// Creates a new notification for a role.
///
/// This function inserts a new role notification record that will be visible
/// to all users with the specified role.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `role_id` - ID of the role to notify
/// * `message` - The notification message text
/// * `notification_type` - Type of notification (info, warning, error, success)
/// * `org_id` - Optional organization ID related to the notification
/// * `app_id` - Optional application ID related to the notification
/// * `importance` - Optional importance level (default is "normal")
/// * `action_url` - Optional URL for a related action
/// * `action_label` - Optional label for the action button
/// * `expires_at` - Optional expiration date for the notification
///
/// # Returns
///
/// * `Ok(RoleNotification)` - Successfully created notification
/// * `Err(anyhow::Error)` - Failed to create notification
pub async fn create_role_notification(
    pool: &Pool<MySql>,
    role_id: i64,
    message: &str,
    notification_type: &str,
    org_id: Option<i64>,
    app_id: Option<i64>,
    importance: Option<&str>,
    action_url: Option<&str>,
    action_label: Option<&str>,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
) -> anyhow::Result<RoleNotification> {
    let mut tx = pool.begin().await?;

    let notification = sqlx::query_as::<_, RoleNotification>(
        r#"INSERT INTO role_notifications (
            role_id, org_id, app_id, notification_type, message, 
            importance, action_url, action_label, created_at, expires_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, ?)"#,
    )
    .bind(role_id)
    .bind(org_id)
    .bind(app_id)
    .bind(notification_type)
    .bind(message)
    .bind(importance.unwrap_or("normal"))
    .bind(action_url)
    .bind(action_label)
    .bind(expires_at)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create role notification")?;

    tx.commit().await?;
    Ok(notification)
}

/// Retrieves a specific role notification by its unique identifier.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the role notification to retrieve
///
/// # Returns
///
/// * `Ok(RoleNotification)` - Successfully retrieved notification
/// * `Err(anyhow::Error)` - Failed to fetch notification
pub async fn get_role_notification_by_id(
    pool: &Pool<MySql>,
    id: i64,
) -> anyhow::Result<RoleNotification> {
    let notification = sqlx::query_as::<_, RoleNotification>(
        "SELECT * FROM role_notifications WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch role notification")?;

    Ok(notification)
}

/// Deletes a role notification.
///
/// This function permanently removes a role notification record with the specified ID.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the notification to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the notification
/// * `Err(anyhow::Error)` - Failed to delete the notification
pub async fn delete_role_notification(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM role_notifications WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete role notification")?;

    tx.commit().await?;
    Ok(())
}

// =================== Notification Acknowledgments ===================

/// Creates a notification acknowledgment for a user.
///
/// This function records that a user has acknowledged a notification,
/// which is useful for role-based notifications that need individual tracking.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - ID of the user acknowledging the notification
/// * `notification_id` - Optional ID of a user notification being acknowledged
/// * `role_notification_id` - Optional ID of a role notification being acknowledged
///
/// # Returns
///
/// * `Ok(NotificationAcknowledgment)` - Successfully created acknowledgment
/// * `Err(anyhow::Error)` - Failed to create acknowledgment
pub async fn create_notification_acknowledgment(
    pool: &Pool<MySql>,
    user_id: i64,
    notification_id: Option<i64>,
    role_notification_id: Option<i64>,
) -> anyhow::Result<NotificationAcknowledgment> {
    // Validate that exactly one notification type is provided
    if (notification_id.is_some() && role_notification_id.is_some()) || 
       (notification_id.is_none() && role_notification_id.is_none()) {
        return Err(anyhow::anyhow!("Either notification_id OR role_notification_id must be provided, not both or neither"));
    }

    let mut tx = pool.begin().await?;

    let acknowledgment = sqlx::query_as::<_, NotificationAcknowledgment>(
        r#"INSERT INTO notification_acknowledgments (
            user_id, notification_id, role_notification_id, acknowledged_at
        ) VALUES (?, ?, ?, CURRENT_TIMESTAMP)"#,
    )
    .bind(user_id)
    .bind(notification_id)
    .bind(role_notification_id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create notification acknowledgment")?;

    tx.commit().await?;
    Ok(acknowledgment)
}

/// Checks if a user has acknowledged a specific role notification.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - ID of the user to check
/// * `role_notification_id` - ID of the role notification to check
///
/// # Returns
///
/// * `Ok(bool)` - True if acknowledged, false otherwise
/// * `Err(anyhow::Error)` - Failed to check acknowledgment status
pub async fn has_acknowledged_role_notification(
    pool: &Pool<MySql>,
    user_id: i64,
    role_notification_id: i64,
) -> anyhow::Result<bool> {
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) FROM notification_acknowledgments
        WHERE user_id = ? AND role_notification_id = ?
        "#,
    )
    .bind(user_id)
    .bind(role_notification_id)
    .fetch_one(pool)
    .await
    .context("Failed to check notification acknowledgment status")?;

    Ok(count > 0)
}

/// Retrieves all role notifications for a user with acknowledgment status.
///
/// This function fetches role notifications for all roles a user has,
/// along with information about whether the user has acknowledged each notification.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - ID of the user
/// * `page` - Zero-based page number
/// * `per_page` - Number of records to fetch per page
///
/// # Returns
///
/// * `Ok(Vec<RoleNotificationWithAcknowledgment>)` - Successfully retrieved notifications with acknowledgment status
/// * `Err(anyhow::Error)` - Failed to fetch notifications
pub async fn get_user_role_notifications(
    pool: &Pool<MySql>,
    user_id: i64,
    page: i64,
    per_page: i64,
) -> anyhow::Result<Vec<UserNotificationWithRoleNotifications>> {
    // First, get user notifications
    let user_notifications = list_user_notifications(pool, user_id, page, per_page, true).await?;
    
    // Then get role notifications for the user's roles
    let role_notifications = sqlx::query_as::<_, RoleNotification>(
        r#"
        SELECT rn.* FROM role_notifications rn
        JOIN user_roles ur ON rn.role_id = ur.role_id
        WHERE ur.user_id = ?
        ORDER BY rn.created_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(user_id)
    .bind(per_page)
    .bind(page * per_page)
    .fetch_all(pool)
    .await
    .context("Failed to fetch role notifications for user")?;
    
    // Get acknowledgments for these role notifications
    let acknowledgments = sqlx::query_as::<_, NotificationAcknowledgment>(
        r#"
        SELECT * FROM notification_acknowledgments
        WHERE user_id = ? AND role_notification_id IN (
            SELECT rn.id FROM role_notifications rn
            JOIN user_roles ur ON rn.role_id = ur.role_id
            WHERE ur.user_id = ?
        )
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch notification acknowledgments")?;
    
    // Combine into a single result
    let result = UserNotificationWithRoleNotifications {
        user_notifications,
        role_notifications,
        acknowledgments,
    };
    
    Ok(vec![result])
}

/// Gets notifications for a user from all sources with unread count.
///
/// This function provides a comprehensive view of all notifications relevant to a user,
/// including both direct user notifications and role-based notifications applicable
/// to the user's roles. It also includes a count of unread notifications.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - ID of the user
/// * `page` - Zero-based page number
/// * `per_page` - Number of records to fetch per page
///
/// # Returns
///
/// * `Ok(NotificationWithCount)` - Successfully retrieved notifications with count
/// * `Err(anyhow::Error)` - Failed to fetch notifications
pub async fn get_all_user_notifications_with_count(
    pool: &Pool<MySql>,
    user_id: i64,
    page: i64,
    per_page: i64,
) -> anyhow::Result<NotificationWithCount> {
    // Get user notifications
    let user_notifications = list_user_notifications(pool, user_id, page, per_page, true).await?;
    
    // Get role notifications
    let role_notifications = sqlx::query_as::<_, RoleNotification>(
        r#"
        SELECT rn.* FROM role_notifications rn
        JOIN user_roles ur ON rn.role_id = ur.role_id
        WHERE ur.user_id = ?
        ORDER BY rn.created_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(user_id)
    .bind(per_page)
    .bind(page * per_page)
    .fetch_all(pool)
    .await
    .context("Failed to fetch role notifications")?;
    
    // Count unread user notifications
    let unread_count = count_unread_user_notifications(pool, user_id).await?;
    
    // Get acknowledgments for role notifications
    let acknowledgments = sqlx::query_as::<_, NotificationAcknowledgment>(
        r#"
        SELECT * FROM notification_acknowledgments
        WHERE user_id = ? AND role_notification_id IN (
            SELECT rn.id FROM role_notifications rn
            JOIN user_roles ur ON rn.role_id = ur.role_id
            WHERE ur.user_id = ?
        )
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch acknowledgments")?;
    
    // Calculate unacknowledged role notifications
    let acknowledged_role_notification_ids: Vec<i64> = acknowledgments
        .iter()
        .filter_map(|ack| ack.role_notification_id)
        .collect();
    
    let unacknowledged_role_count = role_notifications
        .iter()
        .filter(|rn| !acknowledged_role_notification_ids.contains(&rn.id))
        .count() as i64;
    
    // Combine results
    let result = NotificationWithCount {
        user_notifications,
        role_notifications,
        acknowledgments,
        unread_user_count: unread_count,
        unacknowledged_role_count,
        total_unread_count: unread_count + unacknowledged_role_count,
    };
    
    Ok(result)
}