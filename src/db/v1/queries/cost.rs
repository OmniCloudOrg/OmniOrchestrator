use super::super::tables::{ResourceType, CostMetric, CostMetricWithType, ResourcePricing, CostBudget, CostProjection, CostAllocationTag};
use anyhow::Context;
use serde::Serialize;
use sqlx::{MySql, Pool};
use sqlx::Row;
use chrono::{DateTime, Utc};

/// Retrieves a paginated list of resource types from the database.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `page` - Zero-based page number (e.g., 0 for first page, 1 for second page)
/// * `per_page` - Number of records to fetch per page
///
/// # Returns
///
/// * `Ok(Vec<ResourceType>)` - Successfully retrieved list of resource types
/// * `Err(anyhow::Error)` - Failed to fetch resource types, with context
pub async fn list_resource_types(pool: &Pool<MySql>, page: i64, per_page: i64) -> anyhow::Result<Vec<ResourceType>> {
    println!("Attempting to fetch resource types from database...");

    let result = sqlx::query_as::<_, ResourceType>(
        r#"
        SELECT * FROM resource_types
        ORDER BY name ASC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(per_page)
    .bind(page * per_page)
    .fetch_all(pool)
    .await;

    match result {
        Ok(types) => {
            println!("Successfully fetched {} resource types", types.len());
            Ok(types)
        }
        Err(e) => {
            eprintln!("Error fetching resource types: {:#?}", e);
            Err(anyhow::Error::new(e).context("Failed to fetch resource types"))
        }
    }
}

/// Counts the total number of resource types in the database.
pub async fn count_resource_types(pool: &Pool<MySql>) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM resource_types")
        .fetch_one(pool)
        .await
        .context("Failed to count resource types")?;

    Ok(count)
}

/// Retrieves a specific resource type by its unique identifier.
pub async fn get_resource_type_by_id(pool: &Pool<MySql>, id: i32) -> anyhow::Result<ResourceType> {
    let resource_type = sqlx::query_as::<_, ResourceType>("SELECT * FROM resource_types WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch resource type")?;

    Ok(resource_type)
}

/// Creates a new resource type in the database.
pub async fn create_resource_type(
    pool: &Pool<MySql>,
    name: &str,
    category: &str,
    unit_of_measurement: &str,
    description: Option<&str>
) -> anyhow::Result<ResourceType> {
    let mut tx = pool.begin().await?;

    let resource_type = sqlx::query_as::<_, ResourceType>(
        r#"INSERT INTO resource_types (
            name, category, unit_of_measurement, description
        ) VALUES (?, ?, ?, ?)"#,
    )
    .bind(name)
    .bind(category)
    .bind(unit_of_measurement)
    .bind(description)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create resource type")?;

    tx.commit().await?;
    Ok(resource_type)
}

/// Updates an existing resource type in the database.
pub async fn update_resource_type(
    pool: &Pool<MySql>,
    id: i32,
    name: Option<&str>,
    category: Option<&str>,
    unit_of_measurement: Option<&str>,
    description: Option<&str>
) -> anyhow::Result<ResourceType> {
    // Define which fields are being updated
    let update_fields = [
        (name.is_some(), "name = ?"),
        (category.is_some(), "category = ?"),
        (unit_of_measurement.is_some(), "unit_of_measurement = ?"),
        (description.is_some(), "description = ?"),
    ];

    // Build update query with only the fields that have values
    let field_clauses = update_fields
        .iter()
        .filter(|(has_value, _)| *has_value)
        .map(|(_, field)| format!(", {}", field))
        .collect::<String>();

    let query = format!(
        "UPDATE resource_types SET updated_at = CURRENT_TIMESTAMP{} WHERE id = ?",
        field_clauses
    );

    // Start binding parameters
    let mut db_query = sqlx::query_as::<_, ResourceType>(&query);

    // Bind parameters
    if let Some(val) = name {
        db_query = db_query.bind(val);
    }
    if let Some(val) = category {
        db_query = db_query.bind(val);
    }
    if let Some(val) = unit_of_measurement {
        db_query = db_query.bind(val);
    }
    if let Some(val) = description {
        db_query = db_query.bind(val);
    }

    // Bind the ID parameter
    db_query = db_query.bind(id);

    // Execute the query in a transaction
    let mut tx = pool.begin().await?;
    let resource_type = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update resource type")?;

    tx.commit().await?;
    Ok(resource_type)
}

/// Deletes a resource type from the database.
pub async fn delete_resource_type(pool: &Pool<MySql>, id: i32) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM resource_types WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete resource type")?;

    tx.commit().await?;
    Ok(())
}

/// Retrieves a paginated list of cost metrics from the database, with optional filtering.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `page` - Zero-based page number (e.g., 0 for first page, 1 for second page)
/// * `per_page` - Number of records to fetch per page
/// * `resource_type_id` - Optional filter by resource type
/// * `provider_id` - Optional filter by provider
/// * `app_id` - Optional filter by application
/// * `start_date` - Optional filter for metrics after this date
/// * `end_date` - Optional filter for metrics before this date
/// * `billing_period` - Optional filter by billing period (e.g., "2025-05")
///
/// # Returns
///
/// * `Ok(Vec<CostMetricWithType>)` - Successfully retrieved list of cost metrics with type information
/// * `Err(anyhow::Error)` - Failed to fetch cost metrics, with context
pub async fn list_cost_metrics(
    pool: &Pool<MySql>, 
    page: i64, 
    per_page: i64,
    resource_type_id: Option<i32>,
    provider_id: Option<i64>,
    app_id: Option<i64>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    billing_period: Option<&str>
) -> anyhow::Result<Vec<CostMetricWithType>> {
    println!("Attempting to fetch cost metrics from database with filters...");

    // Start building the query with filters
    let mut query = String::from(
        r#"
        SELECT 
            cm.*,
            rt.name as resource_type_name,
            rt.category as resource_type_category,
            rt.unit_of_measurement
        FROM 
            cost_metrics cm
        JOIN 
            resource_types rt ON cm.resource_type_id = rt.id
        WHERE 1=1
        "#
    );
    
    let mut params: Vec<String> = Vec::new();
    
    if resource_type_id.is_some() {
        query.push_str(" AND cm.resource_type_id = ?");
        params.push("resource_type_id".to_string());
    }
    
    if provider_id.is_some() {
        query.push_str(" AND cm.provider_id = ?");
        params.push("provider_id".to_string());
    }
    
    if app_id.is_some() {
        query.push_str(" AND cm.app_id = ?");
        params.push("app_id".to_string());
    }
    
    if start_date.is_some() {
        query.push_str(" AND cm.start_time >= ?");
        params.push("start_date".to_string());
    }
    
    if end_date.is_some() {
        query.push_str(" AND cm.end_time <= ?");
        params.push("end_date".to_string());
    }
    
    if billing_period.is_some() {
        query.push_str(" AND cm.billing_period = ?");
        params.push("billing_period".to_string());
    }
    
    // Add ordering and pagination
    query.push_str(" ORDER BY cm.start_time DESC LIMIT ? OFFSET ?");
    
    // Prepare the query
    let mut db_query = sqlx::query_as::<_, CostMetricWithType>(&query);
    
    // Bind parameters
    if let Some(val) = resource_type_id {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = provider_id {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = app_id {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = start_date {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = end_date {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = billing_period {
        db_query = db_query.bind(val);
    }
    
    // Add pagination parameters
    db_query = db_query.bind(per_page).bind(page * per_page);
    
    // Execute query
    let result = db_query.fetch_all(pool).await;

    match result {
        Ok(metrics) => {
            println!("Successfully fetched {} cost metrics", metrics.len());
            Ok(metrics)
        }
        Err(e) => {
            eprintln!("Error fetching cost metrics: {:#?}", e);
            Err(anyhow::Error::new(e).context("Failed to fetch cost metrics"))
        }
    }
}

/// Counts the total number of cost metrics in the database with optional filtering.
pub async fn count_cost_metrics(
    pool: &Pool<MySql>,
    resource_type_id: Option<i32>,
    provider_id: Option<i64>,
    app_id: Option<i64>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    billing_period: Option<&str>
) -> anyhow::Result<i64> {
    // Start building the query with filters
    let mut query = String::from(
        "SELECT COUNT(*) FROM cost_metrics WHERE 1=1"
    );
    
    if resource_type_id.is_some() {
        query.push_str(" AND resource_type_id = ?");
    }
    
    if provider_id.is_some() {
        query.push_str(" AND provider_id = ?");
    }
    
    if app_id.is_some() {
        query.push_str(" AND app_id = ?");
    }
    
    if start_date.is_some() {
        query.push_str(" AND start_time >= ?");
    }
    
    if end_date.is_some() {
        query.push_str(" AND end_time <= ?");
    }
    
    if billing_period.is_some() {
        query.push_str(" AND billing_period = ?");
    }
    
    // Prepare the query
    let mut db_query = sqlx::query_scalar::<_, i64>(&query);
    
    // Bind parameters
    if let Some(val) = resource_type_id {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = provider_id {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = app_id {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = start_date {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = end_date {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = billing_period {
        db_query = db_query.bind(val);
    }
    
    let count = db_query
        .fetch_one(pool)
        .await
        .context("Failed to count cost metrics")?;

    Ok(count)
}

/// Creates a new cost metric in the database.
pub async fn create_cost_metric(
    pool: &Pool<MySql>,
    resource_type_id: i32,
    provider_id: Option<i64>,
    region_id: Option<i64>,
    app_id: Option<i64>,
    worker_id: Option<i64>,
    org_id: Option<i64>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    usage_quantity: f64,
    unit_cost: f64,
    currency: &str,
    total_cost: f64,
    discount_percentage: Option<f64>,
    discount_reason: Option<&str>,
    billing_period: Option<&str>
) -> anyhow::Result<CostMetric> {
    let mut tx = pool.begin().await?;

    let cost_metric = sqlx::query_as::<_, CostMetric>(
        r#"INSERT INTO cost_metrics (
            resource_type_id, provider_id, region_id, app_id, worker_id, org_id,
            start_time, end_time, usage_quantity, unit_cost, currency, total_cost,
            discount_percentage, discount_reason, billing_period
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(resource_type_id)
    .bind(provider_id)
    .bind(region_id)
    .bind(app_id)
    .bind(worker_id)
    .bind(org_id)
    .bind(start_time)
    .bind(end_time)
    .bind(usage_quantity)
    .bind(unit_cost)
    .bind(currency)
    .bind(total_cost)
    .bind(discount_percentage)
    .bind(discount_reason)
    .bind(billing_period)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create cost metric")?;

    tx.commit().await?;
    Ok(cost_metric)
}

/// Retrieves a specific cost metric by its unique identifier, with resource type information.
pub async fn get_cost_metric_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<CostMetricWithType> {
    let cost_metric = sqlx::query_as::<_, CostMetricWithType>(
        r#"
        SELECT 
            cm.id,
            cm.resource_type_id,
            cm.provider_id,
            cm.region_id,
            cm.app_id,
            cm.worker_id,
            cm.org_id,
            cm.start_time,
            cm.end_time,
            cm.usage_quantity,
            cm.unit_cost,
            cm.currency,
            cm.total_cost,
            cm.discount_percentage,
            cm.discount_reason,
            cm.billing_period,
            cm.created_at,
            cm.updated_at,
            rt.name AS resource_type_name,
            rt.category AS resource_type_category,
            rt.unit_of_measurement
        FROM 
            cost_metrics cm
        JOIN 
            resource_types rt ON cm.resource_type_id = rt.id
        WHERE 
            cm.id = ?
        "#
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch cost metric by id {}: {:?}", id, e);
        anyhow::Error::new(e).context(format!("Failed to fetch cost metric with id {}", id))
    })?
;

    Ok(cost_metric)
}

/// Deletes a cost metric from the database.
pub async fn delete_cost_metric(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM cost_metrics WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete cost metric")?;

    tx.commit().await?;
    Ok(())
}

/// Get aggregate cost metrics grouped by a specific dimension.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `dimension` - Dimension to group by ('app', 'provider', 'resource_type', 'region', 'worker', 'org')
/// * `start_date` - Filter for metrics after this date
/// * `end_date` - Filter for metrics before this date
/// * `limit` - Maximum number of results to return
///
/// # Returns
///
/// * `Ok(Vec<(String, f64)>)` - Successfully retrieved aggregated costs by dimension
/// * `Err(anyhow::Error)` - Failed to fetch cost metrics, with context
pub async fn get_cost_metrics_by_dimension(
    pool: &Pool<MySql>,
    dimension: &str,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    limit: i64
) -> anyhow::Result<Vec<(String, f64)>> {
    // Validate and map the dimension to the appropriate SQL expression
    let (group_field, join_clause) = match dimension {
        "app" => ("apps.name", "LEFT JOIN apps ON cost_metrics.app_id = apps.id"),
        "provider" => ("providers.name", "LEFT JOIN providers ON cost_metrics.provider_id = providers.id"),
        "resource_type" => ("resource_types.name", "LEFT JOIN resource_types ON cost_metrics.resource_type_id = resource_types.id"),
        "region" => ("regions.name", "LEFT JOIN regions ON cost_metrics.region_id = regions.id"),
        "worker" => ("workers.name", "LEFT JOIN workers ON cost_metrics.worker_id = workers.id"),
        "org" => ("orgs.name", "LEFT JOIN orgs ON cost_metrics.org_id = orgs.id"),
        _ => return Err(anyhow::anyhow!("Invalid dimension: {}", dimension)),
    };

    let query = format!(
        r#"
        SELECT 
            {} as name,
            SUM(total_cost) as total_cost
        FROM 
            cost_metrics
        {}
        WHERE 
            start_time >= ? AND end_time <= ?
        GROUP BY 
            name
        ORDER BY 
            total_cost DESC
        LIMIT ?
        "#,
        group_field, join_clause
    );

    let results = sqlx::query(&query)
        .bind(start_date)
        .bind(end_date)
        .bind(limit)
        .map(|row: sqlx::mysql::MySqlRow| {
            let name: String = row.get("name");
            let total_cost: f64 = row.get("total_cost");
            (name, total_cost)
        })
        .fetch_all(pool)
        .await
        .context("Failed to fetch aggregated cost metrics")?;

    Ok(results)
}

/// Retrieves cost metrics over time for a specific application.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - ID of the application to analyze
/// * `interval` - Time interval ('day', 'week', 'month')
/// * `start_date` - Filter for metrics after this date
/// * `end_date` - Filter for metrics before this date
///
/// # Returns
///
/// * `Ok(Vec<(DateTime<Utc>, f64)>)` - Successfully retrieved costs over time
/// * `Err(anyhow::Error)` - Failed to fetch cost metrics, with context
pub async fn get_app_cost_over_time(
    pool: &Pool<MySql>,
    app_id: i64,
    interval: &str,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>
) -> anyhow::Result<Vec<(DateTime<Utc>, f64)>> {
    // Map the interval to the appropriate SQL date function
    let date_function = match interval {
        "day" => "DATE(start_time)",
        "week" => "DATE(start_time - INTERVAL WEEKDAY(start_time) DAY)", // First day of week
        "month" => "DATE_FORMAT(start_time, '%Y-%m-01')", // First day of month
        _ => return Err(anyhow::anyhow!("Invalid interval: {}", interval)),
    };

    let query = format!(
        r#"
        SELECT 
            {} as time_bucket,
            SUM(total_cost) as total_cost
        FROM 
            cost_metrics
        WHERE 
            app_id = ? AND start_time >= ? AND end_time <= ?
        GROUP BY 
            time_bucket
        ORDER BY 
            time_bucket ASC
        "#,
        date_function
    );

    let results = sqlx::query(&query)
        .bind(app_id)
        .bind(start_date)
        .bind(end_date)
        .map(|row: sqlx::mysql::MySqlRow| {
            let time_bucket: DateTime<Utc> = row.get("time_bucket");
            let total_cost: f64 = row.get("total_cost");
            (time_bucket, total_cost)
        })
        .fetch_all(pool)
        .await
        .context("Failed to fetch app cost over time")?;

    Ok(results)
}

/// Retrieves a paginated list of cost budgets from the database.
pub async fn list_cost_budgets(pool: &Pool<MySql>, page: i64, per_page: i64) -> anyhow::Result<Vec<CostBudget>> {
    println!("Attempting to fetch cost budgets from database...");

    let result = sqlx::query_as::<_, CostBudget>(
        r#"
        SELECT * FROM cost_budgets
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(per_page)
    .bind(page * per_page)
    .fetch_all(pool)
    .await;

    match result {
        Ok(budgets) => {
            println!("Successfully fetched {} cost budgets", budgets.len());
            Ok(budgets)
        }
        Err(e) => {
            eprintln!("Error fetching cost budgets: {:#?}", e);
            Err(anyhow::Error::new(e).context("Failed to fetch cost budgets"))
        }
    }
}

/// Counts the total number of cost budgets in the database.
pub async fn count_cost_budgets(pool: &Pool<MySql>) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM cost_budgets")
        .fetch_one(pool)
        .await
        .context("Failed to count cost budgets")?;

    Ok(count)
}

/// Creates a new cost budget in the database.
pub async fn create_cost_budget(
    pool: &Pool<MySql>,
    org_id: i64,
    app_id: Option<i64>,
    budget_name: &str,
    budget_amount: f64,
    currency: &str,
    budget_period: &str,
    period_start: DateTime<Utc>,
    period_end: DateTime<Utc>,
    alert_threshold_percentage: f64,
    alert_contacts: &str,
    created_by: i64
) -> anyhow::Result<CostBudget> {
    let mut tx = pool.begin().await?;

    let cost_budget = sqlx::query_as::<_, CostBudget>(
        r#"INSERT INTO cost_budgets (
            org_id, app_id, budget_name, budget_amount, currency, budget_period,
            period_start, period_end, alert_threshold_percentage, alert_contacts,
            is_active, created_by
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, TRUE, ?)"#,
    )
    .bind(org_id)
    .bind(app_id)
    .bind(budget_name)
    .bind(budget_amount)
    .bind(currency)
    .bind(budget_period)
    .bind(period_start)
    .bind(period_end)
    .bind(alert_threshold_percentage)
    .bind(alert_contacts)
    .bind(created_by)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create cost budget")?;

    tx.commit().await?;
    Ok(cost_budget)
}

/// Retrieves a specific cost budget by its unique identifier.
pub async fn get_cost_budget_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<CostBudget> {
    let cost_budget = sqlx::query_as::<_, CostBudget>("SELECT * FROM cost_budgets WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch cost budget")?;

    Ok(cost_budget)
}

/// Updates an existing cost budget in the database.
pub async fn update_cost_budget(
    pool: &Pool<MySql>,
    id: i64,
    budget_name: Option<&str>,
    budget_amount: Option<f64>,
    alert_threshold_percentage: Option<f64>,
    alert_contacts: Option<&str>,
    is_active: Option<bool>
) -> anyhow::Result<CostBudget> {
    // Define which fields are being updated
    let update_fields = [
        (budget_name.is_some(), "budget_name = ?"),
        (budget_amount.is_some(), "budget_amount = ?"),
        (alert_threshold_percentage.is_some(), "alert_threshold_percentage = ?"),
        (alert_contacts.is_some(), "alert_contacts = ?"),
        (is_active.is_some(), "is_active = ?"),
    ];

    // Build update query with only the fields that have values
    let field_clauses = update_fields
        .iter()
        .filter(|(has_value, _)| *has_value)
        .map(|(_, field)| format!(", {}", field))
        .collect::<String>();

    let query = format!(
        "UPDATE cost_budgets SET updated_at = CURRENT_TIMESTAMP{} WHERE id = ?",
        field_clauses
    );

    // Start binding parameters
    let mut db_query = sqlx::query_as::<_, CostBudget>(&query);

    // Bind parameters
    if let Some(val) = budget_name {
        db_query = db_query.bind(val);
    }
    if let Some(val) = budget_amount {
        db_query = db_query.bind(val);
    }
    if let Some(val) = alert_threshold_percentage {
        db_query = db_query.bind(val);
    }
    if let Some(val) = alert_contacts {
        db_query = db_query.bind(val);
    }
    if let Some(val) = is_active {
        db_query = db_query.bind(val);
    }

    // Bind the ID parameter
    db_query = db_query.bind(id);

    // Execute the query in a transaction
    let mut tx = pool.begin().await?;
    let cost_budget = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update cost budget")?;

    tx.commit().await?;
    Ok(cost_budget)
}

/// Deletes a cost budget from the database.
pub async fn delete_cost_budget(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM cost_budgets WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete cost budget")?;

    tx.commit().await?;
    Ok(())
}

/// Retrieves a paginated list of cost projections from the database.
pub async fn list_cost_projections(pool: &Pool<MySql>, page: i64, per_page: i64) -> anyhow::Result<Vec<CostProjection>> {
    println!("Attempting to fetch cost projections from database...");

    let result = sqlx::query_as::<_, CostProjection>(
        r#"
        SELECT * FROM cost_projections
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(per_page)
    .bind(page * per_page)
    .fetch_all(pool)
    .await;

    match result {
        Ok(projections) => {
            println!("Successfully fetched {} cost projections", projections.len());
            Ok(projections)
        }
        Err(e) => {
            eprintln!("Error fetching cost projections: {:#?}", e);
            Err(anyhow::Error::new(e).context("Failed to fetch cost projections"))
        }
    }
}

/// Creates a new cost projection in the database.
pub async fn create_cost_projection(
    pool: &Pool<MySql>,
    org_id: i64,
    app_id: Option<i64>,
    projection_period: &str,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    projected_cost: f64,
    currency: &str,
    projection_model: &str,
    confidence_level: Option<f64>,
    metadata: Option<&str>
) -> anyhow::Result<CostProjection> {
    let mut tx = pool.begin().await?;

    let cost_projection = sqlx::query_as::<_, CostProjection>(
        r#"INSERT INTO cost_projections (
            org_id, app_id, projection_period, start_date, end_date,
            projected_cost, currency, projection_model, confidence_level, metadata
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(org_id)
    .bind(app_id)
    .bind(projection_period)
    .bind(start_date)
    .bind(end_date)
    .bind(projected_cost)
    .bind(currency)
    .bind(projection_model)
    .bind(confidence_level)
    .bind(metadata)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create cost projection")?;

    tx.commit().await?;
    Ok(cost_projection)
}

/// Retrieves a specific cost projection by its unique identifier.
pub async fn get_cost_projection_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<CostProjection> {
    let cost_projection = sqlx::query_as::<_, CostProjection>("SELECT * FROM cost_projections WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch cost projection")?;

    Ok(cost_projection)
}

/// Deletes a cost projection from the database.
pub async fn delete_cost_projection(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM cost_projections WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete cost projection")?;

    tx.commit().await?;
    Ok(())
}

/// Retrieves a paginated list of resource pricing entries from the database.
pub async fn list_resource_pricing(
    pool: &Pool<MySql>, 
    page: i64, 
    per_page: i64,
    resource_type_id: Option<i32>,
    provider_id: Option<i64>,
    region_id: Option<i64>,
    pricing_model: Option<&str>,
    tier_name: Option<&str>
) -> anyhow::Result<Vec<ResourcePricing>> {
    println!("Attempting to fetch resource pricing from database with filters...");

    // Start building the query with filters
    let mut query = String::from(
        "SELECT * FROM resource_pricing WHERE 1=1"
    );
    
    if resource_type_id.is_some() {
        query.push_str(" AND resource_type_id = ?");
    }
    
    if provider_id.is_some() {
        query.push_str(" AND provider_id = ?");
    }
    
    if region_id.is_some() {
        query.push_str(" AND region_id = ?");
    }
    
    if pricing_model.is_some() {
        query.push_str(" AND pricing_model = ?");
    }
    
    if tier_name.is_some() {
        query.push_str(" AND tier_name = ?");
    }
    
    // Check for current pricing (effective_to is NULL or in the future)
    query.push_str(" AND (effective_to IS NULL OR effective_to > CURRENT_TIMESTAMP)");
    
    // Add ordering and pagination
    query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
    
    // Prepare the query
    let mut db_query = sqlx::query_as::<_, ResourcePricing>(&query);
    
    // Bind parameters
    if let Some(val) = resource_type_id {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = provider_id {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = region_id {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = pricing_model {
        db_query = db_query.bind(val);
    }
    
    if let Some(val) = tier_name {
        db_query = db_query.bind(val);
    }
    
    // Add pagination parameters
    db_query = db_query.bind(per_page).bind(page * per_page);
    
    // Execute query
    let result = db_query.fetch_all(pool).await;

    match result {
        Ok(pricing) => {
            println!("Successfully fetched {} resource pricing entries", pricing.len());
            Ok(pricing)
        }
        Err(e) => {
            eprintln!("Error fetching resource pricing: {:#?}", e);
            Err(anyhow::Error::new(e).context("Failed to fetch resource pricing"))
        }
    }
}

/// Creates a new resource pricing entry in the database.
pub async fn create_resource_pricing(
    pool: &Pool<MySql>,
    resource_type_id: i32,
    provider_id: i64,
    region_id: Option<i64>,
    tier_name: &str,
    unit_price: f64,
    currency: &str,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    pricing_model: &str,
    commitment_period: Option<&str>,
    volume_discount_tiers: Option<&str>
) -> anyhow::Result<ResourcePricing> {
    let mut tx = pool.begin().await?;

    let resource_pricing = sqlx::query_as::<_, ResourcePricing>(
        r#"INSERT INTO resource_pricing (
            resource_type_id, provider_id, region_id, tier_name, unit_price,
            currency, effective_from, effective_to, pricing_model, 
            commitment_period, volume_discount_tiers
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(resource_type_id)
    .bind(provider_id)
    .bind(region_id)
    .bind(tier_name)
    .bind(unit_price)
    .bind(currency)
    .bind(effective_from)
    .bind(effective_to)
    .bind(pricing_model)
    .bind(commitment_period)
    .bind(volume_discount_tiers)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create resource pricing")?;

    tx.commit().await?;
    Ok(resource_pricing)
}

/// Retrieves a specific resource pricing entry by its unique identifier.
pub async fn get_resource_pricing_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<ResourcePricing> {
    let resource_pricing = sqlx::query_as::<_, ResourcePricing>("SELECT * FROM resource_pricing WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch resource pricing")?;

    Ok(resource_pricing)
}

/// Updates an existing resource pricing entry in the database.
pub async fn update_resource_pricing(
    pool: &Pool<MySql>,
    id: i64,
    unit_price: Option<f64>,
    effective_to: Option<DateTime<Utc>>,
    volume_discount_tiers: Option<&str>
) -> anyhow::Result<ResourcePricing> {
    // Define which fields are being updated
    let update_fields = [
        (unit_price.is_some(), "unit_price = ?"),
        (effective_to.is_some(), "effective_to = ?"),
        (volume_discount_tiers.is_some(), "volume_discount_tiers = ?"),
    ];

    // Build update query with only the fields that have values
    let field_clauses = update_fields
        .iter()
        .filter(|(has_value, _)| *has_value)
        .map(|(_, field)| format!(", {}", field))
        .collect::<String>();

    let query = format!(
        "UPDATE resource_pricing SET updated_at = CURRENT_TIMESTAMP{} WHERE id = ?",
        field_clauses
    );

    // Start binding parameters
    let mut db_query = sqlx::query_as::<_, ResourcePricing>(&query);

    // Bind parameters
    if let Some(val) = unit_price {
        db_query = db_query.bind(val);
    }
    if let Some(val) = effective_to {
        db_query = db_query.bind(val);
    }
    if let Some(val) = volume_discount_tiers {
        db_query = db_query.bind(val);
    }

    // Bind the ID parameter
    db_query = db_query.bind(id);

    // Execute the query in a transaction
    let mut tx = pool.begin().await?;
    let resource_pricing = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update resource pricing")?;

    tx.commit().await?;
    Ok(resource_pricing)
}

/// Deletes a resource pricing entry from the database.
pub async fn delete_resource_pricing(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM resource_pricing WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete resource pricing")?;

    tx.commit().await?;
    Ok(())
}

/// Retrieves a list of cost allocation tags for a specific resource.
pub async fn get_cost_allocation_tags(
    pool: &Pool<MySql>,
    resource_id: i64,
    resource_type: &str
) -> anyhow::Result<Vec<CostAllocationTag>> {
    let tags = sqlx::query_as::<_, CostAllocationTag>(
        "SELECT * FROM cost_allocation_tags WHERE resource_id = ? AND resource_type = ?"
    )
    .bind(resource_id)
    .bind(resource_type)
    .fetch_all(pool)
    .await
    .context("Failed to fetch cost allocation tags")?;

    Ok(tags)
}

/// Creates a new cost allocation tag in the database.
pub async fn create_cost_allocation_tag(
    pool: &Pool<MySql>,
    tag_key: &str,
    tag_value: &str,
    resource_id: i64,
    resource_type: &str
) -> anyhow::Result<CostAllocationTag> {
    let mut tx = pool.begin().await?;

    let tag = sqlx::query_as::<_, CostAllocationTag>(
        r#"INSERT INTO cost_allocation_tags (
            tag_key, tag_value, resource_id, resource_type
        ) VALUES (?, ?, ?, ?)"#,
    )
    .bind(tag_key)
    .bind(tag_value)
    .bind(resource_id)
    .bind(resource_type)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create cost allocation tag")?;

    tx.commit().await?;
    Ok(tag)
}

/// Deletes a cost allocation tag from the database.
pub async fn delete_cost_allocation_tag(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM cost_allocation_tags WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete cost allocation tag")?;

    tx.commit().await?;
    Ok(())
}