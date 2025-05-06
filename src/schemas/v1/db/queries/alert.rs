use super::super::tables::{
    Alert, AlertWithAcknowledgments, AlertAcknowledgment, 
    AlertEscalation, AlertHistory, AlertWithRelatedData
};
use anyhow::Context;
use serde::Serialize;
use sqlx::{MySql, Pool, Row};
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

// =================== Alert Management ===================

/// Retrieves a paginated list of alerts from the database.
///
/// This function fetches a subset of alerts based on pagination parameters,
/// ordering them by timestamp in descending order (newest first). Filtering
/// options allow for narrowing down results by various criteria.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `page` - Zero-based page number (e.g., 0 for first page, 1 for second page)
/// * `per_page` - Number of records to fetch per page
/// * `status` - Optional filter for alert status
/// * `severity` - Optional filter for alert severity
/// * `org_id` - Optional filter for organization ID
/// * `app_id` - Optional filter for application ID
/// * `service` - Optional filter for service name
/// * `from_date` - Optional filter for alerts after this timestamp
/// * `to_date` - Optional filter for alerts before this timestamp
///
/// # Returns
///
/// * `Ok(Vec<Alert>)` - Successfully retrieved list of alerts
/// * `Err(anyhow::Error)` - Failed to fetch alerts, with context
pub async fn list_alerts(
    pool: &Pool<MySql>,
    page: i64,
    per_page: i64,
    status: Option<&str>,
    severity: Option<&str>,
    org_id: Option<i64>,
    app_id: Option<i64>,
    service: Option<&str>,
    from_date: Option<DateTime<Utc>>,
    to_date: Option<DateTime<Utc>>,
) -> anyhow::Result<Vec<Alert>> {
    println!("Attempting to fetch alerts from database with filtering...");

    // Start building the query with base selection
    let mut query_string = String::from(
        "SELECT * FROM alerts WHERE 1=1"
    );
    
    // Add optional filters
    if let Some(s) = status {
        query_string.push_str(" AND status = ?");
    }
    if let Some(s) = severity {
        query_string.push_str(" AND severity = ?");
    }
    if let Some(_) = org_id {
        query_string.push_str(" AND org_id = ?");
    }
    if let Some(_) = app_id {
        query_string.push_str(" AND app_id = ?");
    }
    if let Some(s) = service {
        query_string.push_str(" AND service = ?");
    }
    if let Some(_) = from_date {
        query_string.push_str(" AND timestamp >= ?");
    }
    if let Some(_) = to_date {
        query_string.push_str(" AND timestamp <= ?");
    }
    
    // Add order and limit
    query_string.push_str(" ORDER BY timestamp DESC LIMIT ? OFFSET ?");
    
    // Build the query
    let mut query = sqlx::query_as::<_, Alert>(&query_string);
    
    // Bind parameters in the same order
    if let Some(s) = status {
        query = query.bind(s);
    }
    if let Some(s) = severity {
        query = query.bind(s);
    }
    if let Some(id) = org_id {
        query = query.bind(id);
    }
    if let Some(id) = app_id {
        query = query.bind(id);
    }
    if let Some(s) = service {
        query = query.bind(s);
    }
    if let Some(date) = from_date {
        query = query.bind(date);
    }
    if let Some(date) = to_date {
        query = query.bind(date);
    }
    
    // Bind pagination params
    query = query.bind(per_page).bind(page * per_page);
    
    // Execute the query
    let result = query.fetch_all(pool).await;

    match result {
        Ok(alerts) => {
            println!("Successfully fetched {} alerts", alerts.len());
            Ok(alerts)
        }
        Err(e) => {
            eprintln!("Error fetching alerts: {:#?}", e);
            Err(anyhow::Error::new(e).context("Failed to fetch alerts"))
        }
    }
}

/// Retrieves a specific alert by its ID, along with related acknowledgments and escalations.
///
/// This function fetches a single alert record along with its associated acknowledgments,
/// escalations, and history records to provide comprehensive information about the alert.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the alert to retrieve
///
/// # Returns
///
/// * `Ok(AlertWithRelatedData)` - Successfully retrieved alert with related data
/// * `Err(anyhow::Error)` - Failed to fetch alert or related data
pub async fn get_alert_with_related_data(
    pool: &Pool<MySql>,
    id: i64,
) -> anyhow::Result<AlertWithRelatedData> {
    // First fetch the alert
    let alert = sqlx::query_as::<_, Alert>("SELECT * FROM alerts WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch alert")?;
        
    // Fetch acknowledgments
    let acknowledgments = sqlx::query_as::<_, AlertAcknowledgment>(
        "SELECT * FROM alert_acknowledgments WHERE alert_id = ? ORDER BY acknowledged_at DESC"
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch alert acknowledgments")?;
    
    // Fetch escalations
    let escalations = sqlx::query_as::<_, AlertEscalation>(
        "SELECT * FROM alert_escalations WHERE alert_id = ? ORDER BY escalated_at DESC"
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch alert escalations")?;
    
    // Fetch history
    let history = sqlx::query_as::<_, AlertHistory>(
        "SELECT * FROM alert_history WHERE alert_id = ? ORDER BY performed_at DESC"
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch alert history")?;
    
    Ok(AlertWithRelatedData {
        alert,
        acknowledgments,
        escalations,
        history,
    })
}

/// Counts the total number of alerts in the database with optional filtering.
///
/// This function retrieves the total count of alerts that match the provided filter criteria,
/// which is useful for pagination and reporting purposes.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `status` - Optional filter for alert status
/// * `severity` - Optional filter for alert severity
/// * `org_id` - Optional filter for organization ID
/// * `app_id` - Optional filter for application ID
///
/// # Returns
///
/// * `Ok(i64)` - Successfully retrieved count of alerts
/// * `Err(anyhow::Error)` - Failed to count alerts
pub async fn count_alerts(
    pool: &Pool<MySql>,
    status: Option<&str>,
    severity: Option<&str>,
    org_id: Option<i64>,
    app_id: Option<i64>,
) -> anyhow::Result<i64> {
    // Start building the query
    let mut query_string = String::from("SELECT COUNT(*) FROM alerts WHERE 1=1");
    
    // Add optional filters
    if let Some(_) = status {
        query_string.push_str(" AND status = ?");
    }
    if let Some(_) = severity {
        query_string.push_str(" AND severity = ?");
    }
    if let Some(_) = org_id {
        query_string.push_str(" AND org_id = ?");
    }
    if let Some(_) = app_id {
        query_string.push_str(" AND app_id = ?");
    }
    
    // Build the query
    let mut query = sqlx::query_scalar::<_, i64>(&query_string);
    
    // Bind parameters in the same order
    if let Some(s) = status {
        query = query.bind(s);
    }
    if let Some(s) = severity {
        query = query.bind(s);
    }
    if let Some(id) = org_id {
        query = query.bind(id);
    }
    if let Some(id) = app_id {
        query = query.bind(id);
    }
    
    // Execute the query
    let count = query
        .fetch_one(pool)
        .await
        .context("Failed to count alerts")?;

    Ok(count)
}

/// Creates a new alert in the database.
///
/// This function inserts a new alert record with the provided parameters and
/// adds an initial history record to document the alert creation.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `alert_type` - Type of alert (e.g., "cpu_usage", "memory_usage", "disk_space")
/// * `severity` - Severity level of the alert (critical, warning, info)
/// * `service` - Name of the service that generated the alert
/// * `message` - Alert message text describing the issue
/// * `metadata` - Optional JSON data with additional alert details
/// * `org_id` - Optional organization ID related to the alert
/// * `app_id` - Optional application ID related to the alert
/// * `instance_id` - Optional instance ID related to the alert
/// * `region_id` - Optional region ID related to the alert
/// * `node_id` - Optional node/worker ID related to the alert
///
/// # Returns
///
/// * `Ok(Alert)` - Successfully created alert, including database-assigned fields
/// * `Err(anyhow::Error)` - Failed to create alert
pub async fn create_alert(
    pool: &Pool<MySql>,
    alert_type: &str,
    severity: &str,
    service: &str,
    message: &str,
    metadata: Option<JsonValue>,
    org_id: Option<i64>,
    app_id: Option<i64>,
    instance_id: Option<i64>,
    region_id: Option<i64>,
    node_id: Option<i64>,
) -> anyhow::Result<Alert> {
    // Begin transaction
    let mut tx = pool.begin().await?;

    // Insert alert
    let alert = sqlx::query_as::<_, Alert>(
        r#"INSERT INTO alerts (
            alert_type, severity, service, message, timestamp, status,
            metadata, org_id, app_id, instance_id, region_id, node_id
        ) VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP, 'active', ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(alert_type)
    .bind(severity)
    .bind(service)
    .bind(message)
    .bind(metadata)
    .bind(org_id)
    .bind(app_id)
    .bind(instance_id)
    .bind(region_id)
    .bind(node_id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create alert")?;
    
    // Add history record for alert creation
    sqlx::query(
        r#"INSERT INTO alert_history (
            alert_id, action, performed_at, previous_state, new_state
        ) VALUES (?, 'created', CURRENT_TIMESTAMP, NULL, ?)"#,
    )
    .bind(alert.id)
    .bind(serde_json::to_value(&alert).unwrap_or(serde_json::Value::Null))
    .execute(&mut *tx)
    .await
    .context("Failed to create alert history record")?;

    // Commit transaction
    tx.commit().await?;

    // Return newly created alert
    Ok(alert)
}

/// Updates the status of an alert.
///
/// This function changes the status of an alert and records the change
/// in the alert history table. It can mark alerts as acknowledged, resolved,
/// or auto-resolved based on system or user actions.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the alert to update
/// * `new_status` - New status for the alert (active, acknowledged, resolved, auto_resolved)
/// * `user_id` - Optional ID of the user who performed the action
/// * `notes` - Optional notes about the status change
///
/// # Returns
///
/// * `Ok(Alert)` - Successfully updated alert
/// * `Err(anyhow::Error)` - Failed to update alert
pub async fn update_alert_status(
    pool: &Pool<MySql>,
    id: i64,
    new_status: &str,
    user_id: Option<i64>,
    notes: Option<&str>,
) -> anyhow::Result<Alert> {
    // Begin transaction
    let mut tx = pool.begin().await?;
    
    // Get current alert state
    let current_alert = sqlx::query_as::<_, Alert>("SELECT * FROM alerts WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to fetch current alert state")?;
    
    // Prepare update query with resolved_at and resolved_by if needed
    let (query, resolved_at, resolved_by) = if new_status == "resolved" || new_status == "auto_resolved" {
        (
            "UPDATE alerts SET status = ?, resolved_at = CURRENT_TIMESTAMP, resolved_by = ? WHERE id = ?",
            Some(chrono::Utc::now()),
            user_id,
        )
    } else {
        (
            "UPDATE alerts SET status = ? WHERE id = ?",
            None,
            None,
        )
    };
    
    // Execute update based on query type
    let updated_alert = if new_status == "resolved" || new_status == "auto_resolved" {
        sqlx::query_as::<_, Alert>(query)
            .bind(new_status)
            .bind(user_id)
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .context("Failed to update alert status")?
    } else {
        sqlx::query_as::<_, Alert>(query)
            .bind(new_status)
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .context("Failed to update alert status")?
    };
    
    // Add history record
    sqlx::query(
        r#"INSERT INTO alert_history (
            alert_id, action, performed_by, performed_at, 
            previous_state, new_state, notes
        ) VALUES (?, ?, ?, CURRENT_TIMESTAMP, ?, ?, ?)"#,
    )
    .bind(id)
    .bind(format!("status_change_to_{}", new_status))
    .bind(user_id)
    .bind(serde_json::to_value(&current_alert).unwrap_or(serde_json::Value::Null))
    .bind(serde_json::to_value(&updated_alert).unwrap_or(serde_json::Value::Null))
    .bind(notes)
    .execute(&mut *tx)
    .await
    .context("Failed to create alert history record for status update")?;
    
    // Create acknowledgment record if acknowledging
    if new_status == "acknowledged" && user_id.is_some() {
        sqlx::query(
            r#"INSERT INTO alert_acknowledgments (
                alert_id, user_id, acknowledged_at, notes
            ) VALUES (?, ?, CURRENT_TIMESTAMP, ?)"#,
        )
        .bind(id)
        .bind(user_id.unwrap())
        .bind(notes)
        .execute(&mut *tx)
        .await
        .context("Failed to create alert acknowledgment record")?;
    }

    // Commit transaction
    tx.commit().await?;
    
    Ok(updated_alert)
}

/// Acknowledges an alert by a specific user.
///
/// This function creates an acknowledgment record for the alert and
/// optionally updates the alert status to 'acknowledged' if it's currently 'active'.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `alert_id` - ID of the alert to acknowledge
/// * `user_id` - ID of the user acknowledging the alert
/// * `notes` - Optional notes about the acknowledgment
/// * `update_status` - Whether to update the alert status to 'acknowledged'
///
/// # Returns
///
/// * `Ok(AlertAcknowledgment)` - Successfully created acknowledgment
/// * `Err(anyhow::Error)` - Failed to acknowledge alert
pub async fn acknowledge_alert(
    pool: &Pool<MySql>,
    alert_id: i64,
    user_id: i64,
    notes: Option<&str>,
    update_status: bool,
) -> anyhow::Result<AlertAcknowledgment> {
    // Begin transaction
    let mut tx = pool.begin().await?;
    
    // Create acknowledgment
    let acknowledgment = sqlx::query_as::<_, AlertAcknowledgment>(
        r#"INSERT INTO alert_acknowledgments (
            alert_id, user_id, acknowledged_at, notes
        ) VALUES (?, ?, CURRENT_TIMESTAMP, ?)"#,
    )
    .bind(alert_id)
    .bind(user_id)
    .bind(notes)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create alert acknowledgment")?;
    
    // Update alert status if requested
    if update_status {
        // Fetch current alert status
        let current_alert = sqlx::query_as::<_, Alert>("SELECT * FROM alerts WHERE id = ?")
            .bind(alert_id)
            .fetch_one(&mut *tx)
            .await
            .context("Failed to fetch current alert state")?;
        
        // Only update if currently active
        if current_alert.status == "active" {
            // Update status
            let updated_alert = sqlx::query_as::<_, Alert>(
                "UPDATE alerts SET status = 'acknowledged' WHERE id = ?"
            )
            .bind(alert_id)
            .fetch_one(&mut *tx)
            .await
            .context("Failed to update alert status to acknowledged")?;
            
            // Add history record
            sqlx::query(
                r#"INSERT INTO alert_history (
                    alert_id, action, performed_by, performed_at, 
                    previous_state, new_state, notes
                ) VALUES (?, 'status_change_to_acknowledged', ?, CURRENT_TIMESTAMP, ?, ?, ?)"#,
            )
            .bind(alert_id)
            .bind(user_id)
            .bind(serde_json::to_value(&current_alert).unwrap_or(serde_json::Value::Null))
            .bind(serde_json::to_value(&updated_alert).unwrap_or(serde_json::Value::Null))
            .bind(notes)
            .execute(&mut *tx)
            .await
            .context("Failed to create alert history record for acknowledgment")?;
        }
    }
    
    // Commit transaction
    tx.commit().await?;
    
    Ok(acknowledgment)
}

/// Resolves an alert by a specific user.
///
/// This function updates the alert status to 'resolved', sets the resolved_at timestamp
/// and resolved_by user ID, and creates a history record for the resolution.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - ID of the alert to resolve
/// * `user_id` - ID of the user resolving the alert
/// * `notes` - Optional notes about the resolution
///
/// # Returns
///
/// * `Ok(Alert)` - Successfully resolved alert
/// * `Err(anyhow::Error)` - Failed to resolve alert
pub async fn resolve_alert(
    pool: &Pool<MySql>,
    id: i64,
    user_id: i64,
    notes: Option<&str>,
) -> anyhow::Result<Alert> {
    // Begin transaction
    let mut tx = pool.begin().await?;
    
    // Get current alert state
    let current_alert = sqlx::query_as::<_, Alert>("SELECT * FROM alerts WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to fetch current alert state")?;
    
    // Update alert
    let updated_alert = sqlx::query_as::<_, Alert>(
        r#"UPDATE alerts 
           SET status = 'resolved', 
               resolved_at = CURRENT_TIMESTAMP, 
               resolved_by = ? 
           WHERE id = ?"#,
    )
    .bind(user_id)
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to resolve alert")?;
    
    // Add history record
    sqlx::query(
        r#"INSERT INTO alert_history (
            alert_id, action, performed_by, performed_at, 
            previous_state, new_state, notes
        ) VALUES (?, 'resolved', ?, CURRENT_TIMESTAMP, ?, ?, ?)"#,
    )
    .bind(id)
    .bind(user_id)
    .bind(serde_json::to_value(&current_alert).unwrap_or(serde_json::Value::Null))
    .bind(serde_json::to_value(&updated_alert).unwrap_or(serde_json::Value::Null))
    .bind(notes)
    .execute(&mut *tx)
    .await
    .context("Failed to create alert history record for resolution")?;
    
    // Commit transaction
    tx.commit().await?;
    
    Ok(updated_alert)
}

/// Creates an escalation record for an alert.
///
/// This function adds an escalation record to indicate that an alert has been
/// escalated to another level of attention, such as notifying administrators
/// or external systems when an alert has not been addressed in a timely manner.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `alert_id` - ID of the alert being escalated
/// * `escalation_level` - Level of escalation (typically 1, 2, 3, etc.)
/// * `escalated_to` - JSON data specifying where/who the alert was escalated to
/// * `escalation_method` - Method used for escalation (email, SMS, webhook, etc.)
/// * `response_required_by` - Optional deadline for when a response is required
///
/// # Returns
///
/// * `Ok(AlertEscalation)` - Successfully created escalation record
/// * `Err(anyhow::Error)` - Failed to create escalation record
pub async fn create_alert_escalation(
    pool: &Pool<MySql>,
    alert_id: i64,
    escalation_level: i64,
    escalated_to: JsonValue,
    escalation_method: &str,
    response_required_by: Option<DateTime<Utc>>,
) -> anyhow::Result<AlertEscalation> {
    // Begin transaction
    let mut tx = pool.begin().await?;
    
    // Create escalation record
    let escalation = sqlx::query_as::<_, AlertEscalation>(
        r#"INSERT INTO alert_escalations (
            alert_id, escalation_level, escalated_at, 
            escalated_to, escalation_method, response_required_by
        ) VALUES (?, ?, CURRENT_TIMESTAMP, ?, ?, ?)"#,
    )
    .bind(alert_id)
    .bind(escalation_level)
    .bind(escalated_to)
    .bind(escalation_method)
    .bind(response_required_by)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create alert escalation")?;
    
    // Add history record
    sqlx::query(
        r#"INSERT INTO alert_history (
            alert_id, action, performed_at, new_state
        ) VALUES (?, ?, CURRENT_TIMESTAMP, ?)"#,
    )
    .bind(alert_id)
    .bind(format!("escalated_level_{}", escalation_level))
    .bind(serde_json::to_value(&escalation).unwrap_or(serde_json::Value::Null))
    .execute(&mut *tx)
    .await
    .context("Failed to create alert history record for escalation")?;
    
    // Commit transaction
    tx.commit().await?;
    
    Ok(escalation)
}

/// Adds a custom history entry for an alert.
///
/// This function allows adding arbitrary history records for an alert,
/// which is useful for tracking manual interventions or system actions
/// that don't fit into predefined categories.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `alert_id` - ID of the alert
/// * `action` - Description of the action being recorded
/// * `performed_by` - Optional ID of the user who performed the action
/// * `notes` - Optional notes about the action
/// * `previous_state` - Optional JSON data representing the state before the action
/// * `new_state` - Optional JSON data representing the state after the action
///
/// # Returns
///
/// * `Ok(AlertHistory)` - Successfully created history record
/// * `Err(anyhow::Error)` - Failed to create history record
pub async fn add_alert_history(
    pool: &Pool<MySql>,
    alert_id: i64,
    action: &str,
    performed_by: Option<i64>,
    notes: Option<&str>,
    previous_state: Option<JsonValue>,
    new_state: Option<JsonValue>,
) -> anyhow::Result<AlertHistory> {
    let history = sqlx::query_as::<_, AlertHistory>(
        r#"INSERT INTO alert_history (
            alert_id, action, performed_by, performed_at, 
            previous_state, new_state, notes
        ) VALUES (?, ?, ?, CURRENT_TIMESTAMP, ?, ?, ?)"#,
    )
    .bind(alert_id)
    .bind(action)
    .bind(performed_by)
    .bind(previous_state)
    .bind(new_state)
    .bind(notes)
    .fetch_one(pool)
    .await
    .context("Failed to create alert history record")?;
    
    Ok(history)
}

/// Retrieves a list of recent alerts for a specific application.
///
/// This function fetches alerts associated with an application,
/// typically for display on an application dashboard or status page.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - ID of the application
/// * `limit` - Maximum number of alerts to retrieve
/// * `include_resolved` - Whether to include resolved alerts
///
/// # Returns
///
/// * `Ok(Vec<Alert>)` - Successfully retrieved list of alerts
/// * `Err(anyhow::Error)` - Failed to fetch alerts
pub async fn get_recent_app_alerts(
    pool: &Pool<MySql>,
    app_id: i64,
    limit: i64,
    include_resolved: bool,
) -> anyhow::Result<Vec<Alert>> {
    // Build query based on whether to include resolved alerts
    let query = if include_resolved {
        r#"
        SELECT * FROM alerts
        WHERE app_id = ?
        ORDER BY timestamp DESC
        LIMIT ?
        "#
    } else {
        r#"
        SELECT * FROM alerts
        WHERE app_id = ? AND status IN ('active', 'acknowledged')
        ORDER BY timestamp DESC
        LIMIT ?
        "#
    };
    
    let alerts = sqlx::query_as::<_, Alert>(query)
        .bind(app_id)
        .bind(limit)
        .fetch_all(pool)
        .await
        .context("Failed to fetch app alerts")?;
    
    Ok(alerts)
}

/// Retrieves a list of active alerts for a specific organization.
///
/// This function fetches active and acknowledged alerts across all applications
/// and services within an organization, ordered by severity and timestamp.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `org_id` - ID of the organization
/// * `limit` - Maximum number of alerts to retrieve
///
/// # Returns
///
/// * `Ok(Vec<Alert>)` - Successfully retrieved list of alerts
/// * `Err(anyhow::Error)` - Failed to fetch alerts
pub async fn get_org_active_alerts(
    pool: &Pool<MySql>,
    org_id: i64,
    limit: i64,
) -> anyhow::Result<Vec<Alert>> {
    let alerts = sqlx::query_as::<_, Alert>(
        r#"
        SELECT * FROM alerts
        WHERE org_id = ? AND status IN ('active', 'acknowledged')
        ORDER BY 
            CASE 
                WHEN severity = 'critical' THEN 1
                WHEN severity = 'warning' THEN 2
                WHEN severity = 'info' THEN 3
                ELSE 4
            END,
            timestamp DESC
        LIMIT ?
        "#
    )
    .bind(org_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("Failed to fetch org active alerts")?;
    
    Ok(alerts)
}

/// Gets statistics about alerts for an organization grouped by severity and status.
///
/// This function retrieves counts of alerts for an organization, grouped by
/// different categories to provide an overview of the alert landscape.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `org_id` - ID of the organization
/// * `days` - Number of days to look back for statistics
///
/// # Returns
///
/// * `Ok(JsonValue)` - JSON object with alert statistics
/// * `Err(anyhow::Error)` - Failed to fetch alert statistics
pub async fn get_alert_stats(
    pool: &Pool<MySql>,
    org_id: i64,
    days: i64,
) -> anyhow::Result<JsonValue> {
    // Get counts by severity
    let severity_counts = sqlx::query(
        r#"
        SELECT severity, COUNT(*) as count
        FROM alerts
        WHERE org_id = ? AND timestamp >= DATE_SUB(CURRENT_TIMESTAMP, INTERVAL ? DAY)
        GROUP BY severity
        "#
    )
    .bind(org_id)
    .bind(days)
    .fetch_all(pool)
    .await
    .context("Failed to fetch severity counts")?;
    
    // Get counts by status
    let status_counts = sqlx::query(
        r#"
        SELECT status, COUNT(*) as count
        FROM alerts
        WHERE org_id = ? AND timestamp >= DATE_SUB(CURRENT_TIMESTAMP, INTERVAL ? DAY)
        GROUP BY status
        "#
    )
    .bind(org_id)
    .bind(days)
    .fetch_all(pool)
    .await
    .context("Failed to fetch status counts")?;
    
    // Get counts by service
    let service_counts = sqlx::query(
        r#"
        SELECT service, COUNT(*) as count
        FROM alerts
        WHERE org_id = ? AND timestamp >= DATE_SUB(CURRENT_TIMESTAMP, INTERVAL ? DAY)
        GROUP BY service
        "#
    )
    .bind(org_id)
    .bind(days)
    .fetch_all(pool)
    .await
    .context("Failed to fetch service counts")?;
    
    // Get daily trend data
    let daily_trends = sqlx::query(
        r#"
        SELECT 
            DATE(timestamp) as date,
            COUNT(*) as total,
            SUM(CASE WHEN severity = 'critical' THEN 1 ELSE 0 END) as critical,
            SUM(CASE WHEN severity = 'warning' THEN 1 ELSE 0 END) as warning,
            SUM(CASE WHEN severity = 'info' THEN 1 ELSE 0 END) as info
        FROM alerts
        WHERE org_id = ? AND timestamp >= DATE_SUB(CURRENT_TIMESTAMP, INTERVAL ? DAY)
        GROUP BY DATE(timestamp)
        ORDER BY DATE(timestamp)
        "#
    )
    .bind(org_id)
    .bind(days)
    .fetch_all(pool)
    .await
    .context("Failed to fetch daily trend data")?;
    
    // Format all results as JSON
    let mut severity_json = serde_json::Map::new();
    for row in severity_counts {
        let severity: String = row.get("severity");
        let count: i64 = row.get("count");
        severity_json.insert(severity, serde_json::Value::Number(count.into()));
    }
    
    let mut status_json = serde_json::Map::new();
    for row in status_counts {
        let status: String = row.get("status");
        let count: i64 = row.get("count");
        status_json.insert(status, serde_json::Value::Number(count.into()));
    }
    
    let mut service_json = serde_json::Map::new();
    for row in service_counts {
        let service: String = row.get("service");
        let count: i64 = row.get("count");
        service_json.insert(service, serde_json::Value::Number(count.into()));
    }
    
    let mut daily_json = Vec::new();
    for row in daily_trends {
        let date: chrono::NaiveDate = row.get("date");
        let total: i64 = row.get("total");
        let critical: i64 = row.get("critical");
        let warning: i64 = row.get("warning");
        let info: i64 = row.get("info");
        
        let day_data = serde_json::json!({
            "date": date.format("%Y-%m-%d").to_string(),
            "total": total,
            "critical": critical,
            "warning": warning,
            "info": info
        });
        
        daily_json.push(day_data);
    }
    
    // Combine all statistics
    let stats = serde_json::json!({
        "by_severity": severity_json,
        "by_status": status_json,
        "by_service": service_json,
        "daily_trends": daily_json,
        "period_days": days
    });
    
    Ok(stats)
}

/// Retrieves alerts that need escalation based on age and status.
///
/// This function identifies alerts that have been active for longer than a specified
/// threshold period without being acknowledged or resolved, which may indicate
/// they need to be escalated to ensure appropriate attention.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `org_id` - Optional organization ID to filter alerts
/// * `hours_threshold` - Age in hours after which an alert should be considered for escalation
///
/// # Returns
///
/// * `Ok(Vec<Alert>)` - List of alerts needing escalation
/// * `Err(anyhow::Error)` - Failed to fetch alerts
pub async fn get_alerts_needing_escalation(
    pool: &Pool<MySql>,
    org_id: Option<i64>,
    hours_threshold: i64,
) -> anyhow::Result<Vec<Alert>> {
    // Start building the query
    let mut query_string = String::from(
        r#"
        SELECT * FROM alerts
        WHERE status = 'active'
          AND timestamp <= DATE_SUB(CURRENT_TIMESTAMP, INTERVAL ? HOUR)
        "#
    );
    
    // Add organization filter if provided
    if let Some(_) = org_id {
        query_string.push_str(" AND org_id = ?");
    }
    
    // Add order by
    query_string.push_str(
        r#"
        ORDER BY 
            CASE 
                WHEN severity = 'critical' THEN 1
                WHEN severity = 'warning' THEN 2
                ELSE 3
            END,
            timestamp ASC
        "#
    );
    
    // Build and execute query
    let mut query = sqlx::query_as::<_, Alert>(&query_string)
        .bind(hours_threshold);
    
    if let Some(id) = org_id {
        query = query.bind(id);
    }
    
    let alerts = query
        .fetch_all(pool)
        .await
        .context("Failed to fetch alerts needing escalation")?;
    
    Ok(alerts)
}

/// Auto-resolves alerts that have been active for longer than a specified period.
///
/// This function updates the status of old alerts to 'auto_resolved' based on criteria
/// such as age, severity, and current status. It's typically used for housekeeping
/// to prevent the accumulation of stale alerts.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `days_threshold` - Age in days after which an alert should be auto-resolved
/// * `severity_levels` - Optional vector of severity levels to include (e.g., only auto-resolve 'info' alerts)
///
/// # Returns
///
/// * `Ok(i64)` - Number of alerts auto-resolved
/// * `Err(anyhow::Error)` - Failed to auto-resolve alerts
pub async fn auto_resolve_old_alerts(
    pool: &Pool<MySql>,
    days_threshold: i64,
    severity_levels: Option<Vec<&str>>,
) -> anyhow::Result<i64> {
    // Begin transaction
    let mut tx = pool.begin().await?;
    
    // Build base query
    let mut query_string = String::from(
        r#"
        UPDATE alerts
        SET status = 'auto_resolved',
            resolved_at = CURRENT_TIMESTAMP
        WHERE 
            status IN ('active', 'acknowledged')
            AND timestamp <= DATE_SUB(CURRENT_TIMESTAMP, INTERVAL ? DAY)
        "#
    );
    
    // Add severity filter if provided
    if let Some(levels) = &severity_levels {
        if !levels.is_empty() {
            query_string.push_str(" AND severity IN (");
            query_string.push_str(&std::iter::repeat("?")
                .take(levels.len())
                .collect::<Vec<_>>()
                .join(", "));
            query_string.push_str(")");
        }
    }
    
    // Build query
    let mut query = sqlx::query(&query_string)
        .bind(days_threshold);
    
    // Bind severity levels if provided
    if let Some(levels) = severity_levels {
        for level in levels {
            query = query.bind(level);
        }
    }
    
    // Execute update
    let result = query
        .execute(&mut *tx)
        .await
        .context("Failed to auto-resolve old alerts")?;
    
    // Get the affected alerts to create history records
    let affected_alerts = sqlx::query_as::<_, Alert>(
        r#"
        SELECT * FROM alerts
        WHERE 
            status = 'auto_resolved'
            AND resolved_at >= DATE_SUB(CURRENT_TIMESTAMP, INTERVAL 1 MINUTE)
        "#
    )
    .fetch_all(&mut *tx)
    .await
    .context("Failed to fetch auto-resolved alerts")?;
    
    // Create history records for each affected alert
    for alert in &affected_alerts {
        sqlx::query(
            r#"
            INSERT INTO alert_history (
                alert_id, action, performed_at, new_state, notes
            ) VALUES (?, 'auto_resolved', CURRENT_TIMESTAMP, ?, 'Alert auto-resolved due to age')
            "#
        )
        .bind(alert.id)
        .bind(serde_json::to_value(&alert).unwrap_or(serde_json::Value::Null))
        .execute(&mut *tx)
        .await
        .context("Failed to create history record for auto-resolved alert")?;
    }
    
    // Commit transaction
    tx.commit().await?;
    
    Ok(result.rows_affected() as i64)
}

/// Retrieves alerts that match a specific search query.
///
/// This function searches for alerts containing specific text in their message,
/// type, or service fields. It's useful for implementing search functionality
/// in the alerts UI.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `search_query` - Text to search for in alert fields
/// * `org_id` - Optional organization ID to filter alerts
/// * `page` - Zero-based page number for pagination
/// * `per_page` - Number of records per page
///
/// # Returns
///
/// * `Ok(Vec<Alert>)` - List of matching alerts
/// * `Err(anyhow::Error)` - Failed to search for alerts
pub async fn search_alerts(
    pool: &Pool<MySql>,
    search_query: &str,
    org_id: Option<i64>,
    page: i64,
    per_page: i64,
) -> anyhow::Result<Vec<Alert>> {
    // Prepare search pattern
    let pattern = format!("%{}%", search_query);
    
    // Start building the query
    let mut query_string = String::from(
        r#"
        SELECT * FROM alerts
        WHERE (
            message LIKE ? OR
            alert_type LIKE ? OR
            service LIKE ?
        )
        "#
    );
    
    // Add organization filter if provided
    if let Some(_) = org_id {
        query_string.push_str(" AND org_id = ?");
    }
    
    // Add order by and pagination
    query_string.push_str(
        r#"
        ORDER BY timestamp DESC
        LIMIT ? OFFSET ?
        "#
    );
    
    // Build and execute query
    let mut query = sqlx::query_as::<_, Alert>(&query_string)
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern);
    
    if let Some(id) = org_id {
        query = query.bind(id);
    }
    
    query = query
        .bind(per_page)
        .bind(page * per_page);
    
    let alerts = query
        .fetch_all(pool)
        .await
        .context("Failed to search alerts")?;
    
    Ok(alerts)
}

/// Gets the count of matching alerts for a search query.
///
/// This function counts the number of alerts that match a specific search query,
/// which is useful for pagination in search results.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `search_query` - Text to search for in alert fields
/// * `org_id` - Optional organization ID to filter alerts
///
/// # Returns
///
/// * `Ok(i64)` - Count of matching alerts
/// * `Err(anyhow::Error)` - Failed to count alerts
pub async fn count_search_alerts(
    pool: &Pool<MySql>,
    search_query: &str,
    org_id: Option<i64>,
) -> anyhow::Result<i64> {
    // Prepare search pattern
    let pattern = format!("%{}%", search_query);
    
    // Start building the query
    let mut query_string = String::from(
        r#"
        SELECT COUNT(*) FROM alerts
        WHERE (
            message LIKE ? OR
            alert_type LIKE ? OR
            service LIKE ?
        )
        "#
    );
    
    // Add organization filter if provided
    if let Some(_) = org_id {
        query_string.push_str(" AND org_id = ?");
    }
    
    // Build and execute query
    let mut query = sqlx::query_scalar::<_, i64>(&query_string)
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern);
    
    if let Some(id) = org_id {
        query = query.bind(id);
    }
    
    let count = query
        .fetch_one(pool)
        .await
        .context("Failed to count search alerts")?;
    
    Ok(count)
}

/// Bulk updates the status of multiple alerts.
///
/// This function changes the status of multiple alerts at once based on provided criteria,
/// which is useful for operations like resolving all alerts for a specific service or application.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `ids` - Optional vector of alert IDs to update
/// * `service` - Optional service name to filter alerts
/// * `app_id` - Optional application ID to filter alerts
/// * `new_status` - Status to set for matching alerts
/// * `user_id` - ID of the user performing the bulk update
/// * `notes` - Optional notes about the bulk update
///
/// # Returns
///
/// * `Ok(i64)` - Number of alerts updated
/// * `Err(anyhow::Error)` - Failed to update alerts
pub async fn bulk_update_alert_status(
    pool: &Pool<MySql>,
    ids: Option<Vec<i64>>,
    service: Option<&str>,
    app_id: Option<i64>,
    new_status: &str,
    user_id: i64,
    notes: Option<&str>,
) -> anyhow::Result<i64> {
    // Validate that at least one filter is provided
    if ids.is_none() && service.is_none() && app_id.is_none() {
        return Err(anyhow::anyhow!("At least one filter (ids, service, or app_id) must be provided"));
    }
    
    // Begin transaction
    let mut tx = pool.begin().await?;
    
    // First, get all alerts that will be affected to create history records
    let mut select_query_string = String::from("SELECT * FROM alerts WHERE status IN ('active', 'acknowledged')");
    
    if let Some(alert_ids) = &ids {
        if !alert_ids.is_empty() {
            select_query_string.push_str(" AND id IN (");
            select_query_string.push_str(&std::iter::repeat("?")
                .take(alert_ids.len())
                .collect::<Vec<_>>()
                .join(", "));
            select_query_string.push_str(")");
        }
    }
    
    if let Some(_) = service {
        select_query_string.push_str(" AND service = ?");
    }
    
    if let Some(_) = app_id {
        select_query_string.push_str(" AND app_id = ?");
    }
    
    // Build select query
    let mut select_query = sqlx::query_as::<_, Alert>(&select_query_string);
    
    // Bind parameters
    if let Some(alert_ids) = &ids {
        for id in alert_ids {
            select_query = select_query.bind(*id);
        }
    }
    
    if let Some(s) = service {
        select_query = select_query.bind(s);
    }
    
    if let Some(id) = app_id {
        select_query = select_query.bind(id);
    }
    
    // Execute select query
    let affected_alerts = select_query
        .fetch_all(&mut *tx)
        .await
        .context("Failed to fetch alerts for bulk update")?;
    
    // If no alerts match the criteria, return early
    if affected_alerts.is_empty() {
        return Ok(0);
    }
    
    // Now prepare the update query
    let mut update_query_string = String::from(
        "UPDATE alerts SET status = ?"
    );
    
    // Add resolved_at and resolved_by if applicable
    if new_status == "resolved" || new_status == "auto_resolved" {
        update_query_string.push_str(", resolved_at = CURRENT_TIMESTAMP, resolved_by = ?");
    }
    
    // Add WHERE clause
    update_query_string.push_str(" WHERE status IN ('active', 'acknowledged')");
    
    if let Some(alert_ids) = &ids {
        if !alert_ids.is_empty() {
            update_query_string.push_str(" AND id IN (");
            update_query_string.push_str(&std::iter::repeat("?")
                .take(alert_ids.len())
                .collect::<Vec<_>>()
                .join(", "));
            update_query_string.push_str(")");
        }
    }
    
    if let Some(_) = service {
        update_query_string.push_str(" AND service = ?");
    }
    
    if let Some(_) = app_id {
        update_query_string.push_str(" AND app_id = ?");
    }
    
    // Build update query
    let mut update_query = sqlx::query(&update_query_string)
        .bind(new_status);
    
    // Bind resolved_by if applicable
    if new_status == "resolved" || new_status == "auto_resolved" {
        update_query = update_query.bind(user_id);
    }
    
    // Bind filter parameters
    if let Some(alert_ids) = &ids {
        for id in alert_ids {
            update_query = update_query.bind(*id);
        }
    }
    
    if let Some(s) = service {
        update_query = update_query.bind(s);
    }
    
    if let Some(id) = app_id {
        update_query = update_query.bind(id);
    }
    
    // Execute update
    let update_result = update_query
        .execute(&mut *tx)
        .await
        .context("Failed to bulk update alert status")?;
    
    // Create history records for each affected alert
    for alert in &affected_alerts {
        // Create a new Alert instance with updated fields
        let updated_alert = Alert {
            id: alert.id,
            alert_type: alert.alert_type.clone(),
            severity: alert.severity.clone(),
            service: alert.service.clone(),
            message: alert.message.clone(),
            timestamp: alert.timestamp,
            status: new_status.to_string(),
            metadata: alert.metadata.clone(),
            org_id: alert.org_id,
            app_id: alert.app_id,
            instance_id: alert.instance_id,
            region_id: alert.region_id,
            node_id: alert.node_id,
            resolved_at: if new_status == "resolved" || new_status == "auto_resolved" {
                Some(chrono::Utc::now())
            } else {
                alert.resolved_at
            },
            resolved_by: if new_status == "resolved" || new_status == "auto_resolved" {
                Some(user_id)
            } else {
                alert.resolved_by
            }
        };
        
        // Add history record
        sqlx::query(
            r#"
            INSERT INTO alert_history (
                alert_id, action, performed_by, performed_at, 
                previous_state, new_state, notes
            ) VALUES (?, ?, ?, CURRENT_TIMESTAMP, ?, ?, ?)
            "#
        )
        .bind(alert.id)
        .bind(format!("bulk_status_change_to_{}", new_status))
        .bind(user_id)
        .bind(serde_json::to_value(&alert).unwrap_or(serde_json::Value::Null))
        .bind(serde_json::to_value(&updated_alert).unwrap_or(serde_json::Value::Null))
        .bind(notes)
        .execute(&mut *tx)
        .await
        .context("Failed to create history record for bulk update")?;
        
        // If acknowledging, create acknowledgment records
        if new_status == "acknowledged" {
            sqlx::query(
                r#"
                INSERT INTO alert_acknowledgments (
                    alert_id, user_id, acknowledged_at, notes
                ) VALUES (?, ?, CURRENT_TIMESTAMP, ?)
                "#
            )
            .bind(alert.id)
            .bind(user_id)
            .bind(notes)
            .execute(&mut *tx)
            .await
            .context("Failed to create acknowledgment record for bulk update")?;
        }
    }
    
    // Commit transaction
    tx.commit().await?;
    
    Ok(update_result.rows_affected() as i64)
}