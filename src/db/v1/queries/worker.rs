use super::super::tables::Worker;
use anyhow::Context;
use sqlx::{MySql, Pool};
use tracing;

/// Retrieves a paginated list of workers from the database.
///
/// This function fetches workers from the database with optional pagination support.
/// Results are ordered by creation time with the most recently created workers first.
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool for executing the query
/// * `limit` - Optional maximum number of workers to return (defaults to 100 if not specified)
/// * `offset` - Optional number of workers to skip (for pagination)
/// 
/// # Returns
/// 
/// * `Ok(Vec<Worker>)` - Successfully retrieved list of workers
/// * `Err(anyhow::Error)` - Failed to fetch workers
/// 
// Check your database connection code
pub async fn list_workers(
    pool: &sqlx::Pool<sqlx::MySql>,
    page: Option<u64>,
    per_page: Option<u64>
) -> Result<Vec<Worker>, sqlx::Error> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    
    // Use a simple query first to test
    let workers = sqlx::query_as::<_, Worker>(
        "SELECT * FROM workers LIMIT ? OFFSET ?"
    )
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;
    
    Ok(workers)
}
