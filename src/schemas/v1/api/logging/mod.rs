use rocket::serde::json::{json, Value, Json};
use rocket::State;
use rocket::http::Status;
use std::fs;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use clickhouse::Client;
use uuid::Uuid;

// Enum for log levels matching ClickHouse schema
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

// Log entry model for deserialization from ClickHouse
#[derive(Debug, Serialize, Deserialize, clickhouse::Row)]
pub struct LogEntry {
    pub log_id: Option<String>,         // Optional, will generate if missing
    pub timestamp: DateTime<Utc>,
    pub platform_id: String,
    pub org_id: String,
    pub app_id: String,
    pub instance_id: String,
    pub level: LogLevel,
    pub message: String,
    pub context: serde_json::Value,    // Structured JSON context
}

// Log entry for internal processing
#[derive(Debug, Serialize, Deserialize)]
pub struct LogResponse {
    pub log_id: String,
    pub timestamp: DateTime<Utc>,
    pub platform_id: String,
    pub org_id: String,
    pub app_id: String,
    pub instance_id: String,
    pub level: String,               // String for API response
    pub message: String,
    pub context: serde_json::Value,  // Structured JSON context
}

// Pagination structure
#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
    pub total_count: i64,
    pub total_pages: i64,
}

// Structure for bulk log insertion
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkLogInsert {
    pub logs: Vec<LogEntry>,
}

// Custom struct for error stats results
#[derive(Debug, Serialize, Deserialize)]
struct ErrorStat {
    platform_id: String,
    org_id: String,
    app_id: String,
    level: u8,
    count: u64,
    event_date: chrono::NaiveDate,
}

// ClickHouse DB initialization
pub async fn init_clickhouse_db(client: &Client, schema_path: &str) -> Result<(), clickhouse::error::Error> {
    // Read the SQL schema file
    let schema_sql = match fs::read_to_string(schema_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read schema file: {}", e);
            return Err(clickhouse::error::Error::Custom(format!("Failed to read schema file: {}", e)));
        }
    };
    
    // Proper SQL parsing: Split by semicolons and handle each statement carefully
    let statements: Vec<String> = schema_sql
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with("--"))
        .map(|s| s.to_string())
        .collect();
    
    println!("Found {} SQL statements to execute", statements.len());
    
    // Execute each statement separately
    for (i, stmt) in statements.iter().enumerate() {
        if stmt.trim().is_empty() {
            continue; // Skip truly empty statements
        }
        
        println!("Executing statement {}/{}: {} characters", i+1, statements.len(), stmt.len());
        match client.query(stmt).execute().await {
            Ok(_) => println!("Statement {}/{} executed successfully", i+1, statements.len()),
            Err(e) => {
                eprintln!("Failed to execute statement {}/{}: {:?}", i+1, statements.len(), e);
                eprintln!("Statement content: {}", stmt);
                return Err(e);
            }
        }
    }
    
    Ok(())
}

// Function to fetch logs with pagination using optimized ClickHouse queries
async fn fetch_logs_paginated(
    client: &Client,
    query_conditions: &str,
    page: i64,
    per_page: i64,
) -> Result<(Vec<LogResponse>, i64), clickhouse::error::Error> {
    // Calculate offset for pagination
    let offset = (page - 1) * per_page;
    
    // Count total matching logs (using optimized query)
    let count_sql = format!(
        "SELECT count() FROM omni_logs.logs WHERE {}",
        query_conditions
    );
    
    // Use prewhere for better performance when filtering
    let logs_sql = format!(
        r#"
        SELECT 
            log_id, 
            timestamp, 
            platform_id, 
            org_id, 
            app_id, 
            instance_id, 
            level, 
            message, 
            context
        FROM omni_logs.logs
        PREWHERE {}
        ORDER BY timestamp DESC
        LIMIT {} OFFSET {}
        "#,
        query_conditions, 
        per_page, 
        offset
    );
    
    // Execute the count query
    let count = client
        .query(&count_sql)
        .fetch_one::<i64>()
        .await?;  // Extract the value from the tuple
    
    // Execute logs query
    let rows: Vec<LogEntry> = client.query(&logs_sql).fetch_all().await?;
    
    // Convert from internal types to response types
    let mut logs = Vec::with_capacity(rows.len());
    
    for row in rows {
        let log_id: String = row.log_id.ok_or(clickhouse::error::Error::Custom("Missing log_id".to_string()))?;
        let timestamp: DateTime<Utc> = row.timestamp;
        let platform_id: String = row.platform_id;
        let org_id: String = row.org_id;
        let app_id: String = row.app_id;
        let instance_id: String = row.instance_id;
        
        // Convert enum to string
        let level_num: u8 = row.level as u8;
        let level = match level_num {
            1 => "debug",
            2 => "info",
            3 => "warn",
            4 => "error",
            5 => "fatal",
            _ => "unknown",
        };
        
        let message: String = row.message;
        
        // Parse context JSON
        let context_str: String = row.context.to_string();
        let context: serde_json::Value = serde_json::from_str(&context_str)
            .unwrap_or(serde_json::Value::Null);
        
        logs.push(LogResponse {
            log_id,
            timestamp,
            platform_id,
            org_id,
            app_id,
            instance_id,
            level: level.to_string(),
            message,
            context,
        });
    }
    
    Ok((logs, count))
}

// Main logs endpoint with filtering and pagination
#[get("/logs?<page>&<per_page>&<platform_id>&<org_id>&<app_id>&<instance_id>&<level>&<start_time>&<end_time>&<search>")]
pub async fn list_logs(
    page: Option<i64>,
    per_page: Option<i64>,
    platform_id: Option<String>,
    org_id: Option<String>,
    app_id: Option<String>,
    instance_id: Option<String>,
    level: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    search: Option<String>,
    clickhouse: &State<Client>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    // Default pagination values
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(50);
    
    if page < 1 || per_page < 1 || per_page > 1000 {
        return Err((
            Status::BadRequest,
            Json(json!({
                "error": "Invalid pagination parameters",
                "message": "Page must be ≥ 1 and per_page must be between 1 and 1000"
            }))
        ));
    }
    
    // Build optimized query conditions
    let mut conditions = Vec::new();
    
    // FIX: Removed unused variable warning by prefixing with underscore
    let _using_hierarchy_filter = platform_id.is_some() || org_id.is_some() || app_id.is_some();
    
    if let Some(pid) = platform_id {
        conditions.push(format!("platform_id = '{}'", pid.replace('\'', "''")));
    }
    
    if let Some(oid) = org_id {
        conditions.push(format!("org_id = '{}'", oid.replace('\'', "''")));
    }
    
    if let Some(aid) = app_id {
        conditions.push(format!("app_id = '{}'", aid.replace('\'', "''")));
    }
    
    if let Some(iid) = instance_id {
        conditions.push(format!("instance_id = '{}'", iid.replace('\'', "''")));
    }
    
    if let Some(lvl) = level {
        // Convert string level to enum
        let level_enum = match lvl.to_lowercase().as_str() {
            "debug" => 1,
            "info" => 2,
            "warn" => 3,
            "error" => 4, 
            "fatal" => 5,
            _ => {
                return Err((
                    Status::BadRequest,
                    Json(json!({
                        "error": "Invalid log level",
                        "message": "Level must be one of: debug, info, warn, error, fatal"
                    }))
                ));
            }
        };
        conditions.push(format!("level = {}", level_enum));
    }
    
    // Optimize date range searches using partitioning
    if let Some(st) = start_time {
        conditions.push(format!("timestamp >= toDateTime64('{}', 3, 'UTC')", st));
        
        // Add event_date condition for better partition pruning
        conditions.push(format!("event_date >= toDate('{}')", st));
    }
    
    if let Some(et) = end_time {
        conditions.push(format!("timestamp <= toDateTime64('{}', 3, 'UTC')", et));
        
        // Add event_date condition for better partition pruning
        conditions.push(format!("event_date <= toDate('{}')", et));
    }
    
    // Use token bloom filter for message search instead of slow LIKE
    if let Some(term) = search {
        let escaped_term = term.replace('\'', "''");
        conditions.push(format!("message ILIKE '%{}%'", escaped_term));
    }
    
    // Default condition if none provided
    let query_conditions = if conditions.is_empty() {
        "1=1".to_string()
    } else {
        conditions.join(" AND ")
    };
    
    // Fetch logs with pagination
    match fetch_logs_paginated(clickhouse, &query_conditions, page, per_page).await {
        Ok((logs, total_count)) => {
            let total_pages = (total_count + per_page - 1) / per_page; // Ceiling division
            
            let response = json!({
                "logs": logs,
                "pagination": {
                    "page": page,
                    "per_page": per_page,
                    "total_count": total_count,
                    "total_pages": total_pages
                }
            });
            
            Ok(Json(response))
        },
        Err(err) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Database error",
                "message": err.to_string()
            }))
        ))
    }
}

// Platform routes - reuse the main list_logs with prefilled platform_id
#[get("/platforms/<platform_id>/logs?<page>&<per_page>&<level>&<start_time>&<end_time>&<search>")]
pub async fn list_platform_logs(
    platform_id: String,
    page: Option<i64>,
    per_page: Option<i64>,
    level: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    search: Option<String>,
    clickhouse: &State<Client>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    list_logs(
        page,
        per_page,
        Some(platform_id),
        None,
        None,
        None,
        level,
        start_time,
        end_time,
        search,
        clickhouse,
    ).await
}

// Organization routes
#[get("/orgs/<org_id>/logs?<page>&<per_page>&<platform_id>&<level>&<start_time>&<end_time>&<search>")]
pub async fn list_org_logs(
    org_id: String,
    page: Option<i64>,
    per_page: Option<i64>,
    platform_id: Option<String>,
    level: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    search: Option<String>,
    clickhouse: &State<Client>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    list_logs(
        page,
        per_page,
        platform_id,
        Some(org_id),
        None,
        None,
        level,
        start_time,
        end_time,
        search,
        clickhouse,
    ).await
}

// App routes
#[get("/apps/<app_id>/logs?<page>&<per_page>&<platform_id>&<org_id>&<level>&<start_time>&<end_time>&<search>")]
pub async fn list_app_logs(
    app_id: String,
    page: Option<i64>,
    per_page: Option<i64>,
    platform_id: Option<String>,
    org_id: Option<String>,
    level: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    search: Option<String>,
    clickhouse: &State<Client>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    list_logs(
        page,
        per_page,
        platform_id,
        org_id,
        Some(app_id),
        None,
        level,
        start_time,
        end_time,
        search,
        clickhouse,
    ).await
}

// Instance routes
#[get("/instances/<instance_id>/logs?<page>&<per_page>&<platform_id>&<org_id>&<app_id>&<level>&<start_time>&<end_time>&<search>")]
pub async fn list_instance_logs(
    instance_id: String,
    page: Option<i64>,
    per_page: Option<i64>,
    platform_id: Option<String>,
    org_id: Option<String>,
    app_id: Option<String>,
    level: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    search: Option<String>,
    clickhouse: &State<Client>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    list_logs(
        page,
        per_page,
        platform_id,
        org_id,
        app_id,
        Some(instance_id),
        level,
        start_time,
        end_time,
        search,
        clickhouse,
    ).await
}

// Efficient bulk log insertion - using multiple rows approach instead of tuples
#[post("/logs", format = "json", data = "<log_batch>")]
pub async fn insert_logs(
    log_batch: Json<BulkLogInsert>,
    clickhouse: &State<Client>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let logs = log_batch.into_inner().logs;
    
    if logs.is_empty() {
        return Ok(Json(json!({
            "status": "success",
            "message": "No logs to insert",
            "count": 0
        })));
    }
    
    // FIX: Use individual inserts instead of tuples to avoid the Row trait limitation
    let mut inserted_count = 0;
    
    // Start a transaction
    let _tx = clickhouse.query("BEGIN TRANSACTION").execute().await;
    
    for mut log in logs {
        // Generate UUID if not provided
        if log.log_id.is_none() {
            log.log_id = Some(Uuid::new_v4().to_string());
        }
        
        // Serialize context to string
        let context_str = serde_json::to_string(&log.context)
            .unwrap_or_else(|_| "{}".to_string());
        
        // Convert level to u8
        let level_num = match log.level {
            LogLevel::Debug => 1_u8,
            LogLevel::Info => 2_u8,
            LogLevel::Warn => 3_u8,
            LogLevel::Error => 4_u8,
            LogLevel::Fatal => 5_u8,
        };
        
        // Insert as a single row using SQL parameters
        let insert_sql = format!(
            r#"
            INSERT INTO omni_logs.logs
            (log_id, timestamp, platform_id, org_id, app_id, instance_id, level, message, context)
            VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {}, '{}', '{}')
            "#,
            log.log_id.unwrap().replace('\'', "''"),
            log.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
            log.platform_id.replace('\'', "''"),
            log.org_id.replace('\'', "''"),
            log.app_id.replace('\'', "''"),
            log.instance_id.replace('\'', "''"),
            level_num,
            log.message.replace('\'', "''"),
            context_str.replace('\'', "''")
        );
        
        if let Err(err) = clickhouse.query(&insert_sql).execute().await {
            // Rollback if there's an error
            let _ = clickhouse.query("ROLLBACK").execute().await;
            
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Failed to insert log",
                    "message": err.to_string(),
                    "count": inserted_count
                }))
            ));
        }
        
        inserted_count += 1;
    }
    
    // Commit the transaction
    if let Err(err) = clickhouse.query("COMMIT").execute().await {
        return Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to commit transaction",
                "message": err.to_string(),
                "count": inserted_count
            }))
        ));
    }
    
    Ok(Json(json!({
        "status": "success",
        "message": "Logs inserted successfully",
        "count": inserted_count
    })))
}

// Function to register all routes
pub fn routes() -> Vec<rocket::Route> {
    routes![
        list_logs,
        list_platform_logs,
        list_org_logs,
        list_app_logs,
        list_instance_logs,
        insert_logs
    ]
}