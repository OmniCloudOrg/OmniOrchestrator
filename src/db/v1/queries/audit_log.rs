use super::super::tables::AuditLog;
use anyhow::Context;
use sqlx::{MySql, Pool};

/// Creates a new audit log entry in the system.
///
/// This function records an action performed within the system, tracking who
/// performed the action, what organization they belong to, what action was
/// performed, on what type of resource, and which specific resource was affected.
/// It serves as a critical component for maintaining accountability and tracking
/// system changes.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - Optional ID of the user who performed the action (None for system actions)
/// * `org_id` - Optional ID of the organization in which the action occurred
/// * `action` - Description of the action performed (e.g., "create", "update", "delete")
/// * `resource_type` - Type of resource affected (e.g., "app", "deployment", "user")
/// * `resource_id` - Optional identifier of the specific resource affected
///
/// # Returns
///
/// * `Ok(AuditLog)` - Successfully created audit log entry with all database-assigned fields
/// * `Err(anyhow::Error)` - Failed to create the audit log entry
///
/// # Examples
///
/// ```
/// // Log a user creating an application
/// create_audit_log(
///     &pool,
///     Some(user_id),
///     Some(org_id),
///     "create",
///     "app",
///     Some(app_id.to_string())
/// ).await?;
///
/// // Log a system maintenance action
/// create_audit_log(
///     &pool,
///     None,
///     None,
///     "maintenance",
///     "system",
///     None
/// ).await?;
/// ```
pub async fn create_audit_log(
    pool: &Pool<MySql>,
    user_id: Option<i64>,
    org_id: Option<i64>,
    action: &str,
    resource_type: &str,
    resource_id: Option<String>,
) -> anyhow::Result<AuditLog> {
    let audit_log = sqlx::query_as::<_, AuditLog>(
        r#"
            INSERT INTO audit_logs (
            user_id, org_id, action, resource_type, resource_id
            ) VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind(org_id)
    .bind(action)
    .bind(resource_type)
    .bind(resource_id)
    .fetch_one(pool)
    .await
    .context("Failed to create audit log")?;

    Ok(audit_log)
}

/// Retrieves a paginated list of audit logs ordered by creation time.
///
/// This function fetches audit logs with pagination support, allowing for
/// efficient browsing through potentially large numbers of log entries.
/// The most recent logs are returned first.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `limit` - Maximum number of logs to retrieve in this page
/// * `page` - Zero-based page offset (e.g., 0 for first page, 1 for second page)
///
/// # Returns
///
/// * `Ok(Vec<AuditLog>)` - Successfully retrieved audit logs for the requested page
/// * `Err(anyhow::Error)` - Failed to fetch the audit logs
///
/// # Pagination
///
/// The function calculates the appropriate OFFSET based on the page and limit values.
/// For example, with a limit of 10:
/// - page 0 → entries 0-9
/// - page 1 → entries 10-19
/// - page 2 → entries 20-29
///
/// # Notes
///
/// The page parameter in this function represents the page number, not the offset value.
/// The actual offset is calculated internally as `page * limit`.
pub async fn list_audit_logs_paginated(
    pool: &Pool<MySql>,
    limit: i64,
    page: i64,
) -> anyhow::Result<Vec<AuditLog>> {
    let audit_logs = sqlx::query_as::<_, AuditLog>(
        r#"
            SELECT * FROM audit_logs 
            ORDER BY created_at DESC 
            LIMIT ? OFFSET ?
        "#,
    )
    .bind(limit)
    .bind(page)
    .fetch_all(pool)
    .await
    .context("Failed to fetch audit logs")?;

    Ok(audit_logs)
}

/// Retrieves audit logs for a specific resource.
///
/// This function fetches audit logs related to a particular resource, identified
/// by its type and ID. This is useful for viewing the history of actions performed
/// on a specific entity in the system.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `resource_type` - Type of resource to filter by (e.g., "app", "deployment")
/// * `resource_id` - Identifier of the specific resource to get logs for
/// * `limit` - Maximum number of logs to retrieve
///
/// # Returns
///
/// * `Ok(Vec<AuditLog>)` - Successfully retrieved audit logs for the resource
/// * `Err(anyhow::Error)` - Failed to fetch the resource audit logs
///
/// # Ordering
///
/// Results are ordered by creation time in descending order, so the most
/// recent actions appear first in the returned list.
///
/// # Example
///
/// ```
/// // Get the most recent 20 actions on an application
/// let app_history = get_audit_logs_by_resource(
///     &pool, 
///     "app", 
///     &app_id.to_string(), 
///     20
/// ).await?;
/// ```
pub async fn get_audit_logs_by_resource(
    pool: &Pool<MySql>,
    resource_type: &str,
    resource_id: &str,
    limit: i64,
) -> anyhow::Result<Vec<AuditLog>> {
    let audit_logs = sqlx::query_as::<_, AuditLog>(
        r#"
            SELECT * FROM audit_logs 
            WHERE resource_type = ? AND resource_id = ?
            ORDER BY created_at DESC 
            LIMIT ?
        "#,
    )
    .bind(resource_type)
    .bind(resource_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("Failed to fetch resource audit logs")?;

    Ok(audit_logs)
}

/// Retrieves audit logs for actions performed by a specific user.
///
/// This function fetches audit logs for activities carried out by a particular user,
/// identified by their user ID. This is useful for monitoring user activities,
/// security auditing, or providing users with a history of their actions.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - Unique identifier of the user whose logs to retrieve
/// * `limit` - Maximum number of logs to retrieve
///
/// # Returns
///
/// * `Ok(Vec<AuditLog>)` - Successfully retrieved audit logs for the user
/// * `Err(anyhow::Error)` - Failed to fetch the user audit logs
///
/// # Use Cases
///
/// Common use cases for this function include:
/// - Security monitoring for suspicious user activity
/// - User activity history displays in admin interfaces
/// - Accountability tracking for administrative actions
/// - Providing users with a history of their own actions in the system
///
/// # Ordering
///
/// Results are ordered by creation time in descending order, so the most
/// recent actions appear first in the returned list.
pub async fn get_user_audit_logs(
    pool: &Pool<MySql>,
    user_id: i64,
    limit: i64,
) -> anyhow::Result<Vec<AuditLog>> {
    let audit_logs = sqlx::query_as::<_, AuditLog>(
        r#"
            SELECT * FROM audit_logs 
            WHERE user_id = ?
            ORDER BY created_at DESC 
            LIMIT ?
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("Failed to fetch user audit logs")?;

    Ok(audit_logs)
}

/// Retrieves audit logs for actions within a specific organization.
///
/// This function fetches audit logs for all activities that occurred within
/// a particular organization, identified by its organization ID. This is useful
/// for organizational-level auditing, compliance reporting, and activity monitoring.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `org_id` - Unique identifier of the organization whose logs to retrieve
/// * `limit` - Maximum number of logs to retrieve
///
/// # Returns
///
/// * `Ok(Vec<AuditLog>)` - Successfully retrieved audit logs for the organization
/// * `Err(anyhow::Error)` - Failed to fetch the organization audit logs
///
/// # Use Cases
///
/// Common use cases for this function include:
/// - Compliance reporting for organization activities
/// - Administrative oversight of organization-wide actions
/// - Organizational security auditing
/// - Activity dashboards for organization managers
///
/// # Ordering
///
/// Results are ordered by creation time in descending order, so the most
/// recent actions appear first in the returned list.
///
/// # Note
///
/// This function retrieves all actions associated with the organization,
/// regardless of which user performed them. This includes system actions
/// that affect the organization but weren't initiated by a specific user.
pub async fn get_org_audit_logs(
    pool: &Pool<MySql>,
    org_id: i64,
    limit: i64,
) -> anyhow::Result<Vec<AuditLog>> {
    let audit_logs = sqlx::query_as::<_, AuditLog>(
        r#"
            SELECT * FROM audit_logs 
            WHERE org_id = ?
            ORDER BY created_at DESC 
            LIMIT ?
        "#,
    )
    .bind(org_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("Failed to fetch organization audit logs")?;

    Ok(audit_logs)
}