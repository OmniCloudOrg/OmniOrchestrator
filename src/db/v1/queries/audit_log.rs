use sqlx::{MySql, Pool};
use anyhow::Context;
use super::super::tables::AuditLog;

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
        "#
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

pub async fn list_audit_logs_paginated(
    pool: &Pool<MySql>,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<AuditLog>> {
    let audit_logs = sqlx::query_as::<_, AuditLog>(
        r#"
            SELECT * FROM audit_logs 
            ORDER BY created_at DESC 
            LIMIT ? OFFSET ?
        "#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .context("Failed to fetch audit logs")?;

    Ok(audit_logs)
}

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
        "#
    )
    .bind(resource_type)
    .bind(resource_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("Failed to fetch resource audit logs")?;

    Ok(audit_logs)
}

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
        "#
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("Failed to fetch user audit logs")?;

    Ok(audit_logs)
}

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
        "#
    )
    .bind(org_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("Failed to fetch organization audit logs")?;

    Ok(audit_logs)
}