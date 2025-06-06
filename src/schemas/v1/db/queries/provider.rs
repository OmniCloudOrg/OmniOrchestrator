use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use anyhow::Context;

use libomni::types::db::v1 as types;
use types::provider::{Provider, ProviderAuditLog};
use types::instance::Instance;

/// Retrieves a paginated  list of providers from the database.
pub async fn get_providers_paginated(
    pool: &Pool<MySql>,
    page: i64,
    page_size: i64,
) -> anyhow::Result<Vec<Provider>> {
    let offset = page * page_size;
    let query = sqlx::query_as::<_, Provider>(
        r#"SELECT * FROM providers LIMIT ? OFFSET ?"#,
    )
    .bind(page_size)
    .bind(offset);

    query.fetch_all(pool).await.context("Failed to fetch providers")
}

/// Counts the total number of providers in the database.
pub async fn get_provider_count(pool: &Pool<MySql>) -> anyhow::Result<i64> {
    let query = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM providers");
    query.fetch_one(pool).await.context("Failed to count providers")
}

/// Retrieves a pagnated list of audit logs for a specific provider.
/// 
/// # Arguments
/// * `pool` - The database connection pool.
/// * `provider_id` - The ID of the provider to retrieve audit logs for.
/// * `page` - The page number to retrieve.
/// * `per_page` - The number of audit logs to retrieve per page.
/// 
/// # Returns
/// A JSON response containing the list of audit logs and pagination information.
pub async fn get_provider_audit_logs_paginated(
    pool: &Pool<MySql>,
    provider_id: i64,
    page: i64,
    per_page: i64,
) -> anyhow::Result<Vec<ProviderAuditLog>> {
    let offset = page * per_page;
    let query = sqlx::query_as::<_, ProviderAuditLog>(
        r#"SELECT * FROM provider_audit_logs WHERE provider_id = ? LIMIT ? OFFSET ?"#,
    )
    .bind(provider_id)
    .bind(per_page)
    .bind(offset);

    let data = query.fetch_all(pool).await.with_context(|| format!(
        "Failed to fetch audit logs for provider_id: {}, page: {}, per_page: {}",
        provider_id, page, per_page
    ));
    match data {
        Ok(logs) => Ok(logs),
        Err(e) => {
            println!("Error fetching provider audit logs: {}", e);
            Err(anyhow::anyhow!("Failed to fetch provider audit logs: {}", e))
        },
    }
}

/// Counts the total number of audit logs for a given provider.
pub async fn get_provider_audit_log_count(
    pool: &Pool<MySql>,
    provider_id: i64,
) -> anyhow::Result<i64> {
    let query = sqlx::query_scalar::<_, i64>(
        r#"SELECT COUNT(*) FROM provider_audit_logs WHERE provider_id = ?"#,
    )
    .bind(provider_id);

    query.fetch_one(pool).await.context("Failed to count provider audit logs")
}

/// Fetch all the instances a provider is responsible for via the regions table between them, with pagination
pub async fn get_provider_instances(
    pool: &Pool<MySql>,
    provider_id: i64,
    page: i64,
    page_size: i64,
) -> anyhow::Result<Vec<Instance>> {
    let offset = page * page_size;
    let query = sqlx::query_as::<_, Instance>(
        r#"
        SELECT i.* 
        FROM instances i
        INNER JOIN regions r ON r.id = i.region_id
        WHERE r.provider = ?
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(provider_id)
    .bind(page_size)
    .bind(offset);

    query.fetch_all(pool).await.context("Failed to fetch instances for provider")
}

/// Counts the total number of instances associated with a provider.
pub async fn get_provider_instance_count(
    pool: &Pool<MySql>,
    provider_id: i64,
) -> anyhow::Result<i64> {
    let query = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) 
        FROM instances i
        INNER JOIN regions r ON r.id = i.region_id
        WHERE r.provider = ?
        "#,
    )
    .bind(provider_id);

    query.fetch_one(pool).await.context("Failed to count provider instances")
}