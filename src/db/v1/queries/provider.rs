use super::super::tables::Provider;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

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