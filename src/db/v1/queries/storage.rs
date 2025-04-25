use super::super::tables::{
    StorageVolume,
};
use anyhow::Context;
use sqlx::{MySql, Pool};

/// Retrieves a paginated list of storage volumes using `WHERE id BETWEEN`
pub async fn list_storage_volumes_paginated(
    pool: &Pool<MySql>,
    start_id: i64,
    end_id: i64,
) -> anyhow::Result<Vec<StorageVolume>> {
    let query = format!(
        "SELECT * FROM storage_volumes WHERE id BETWEEN {} AND {}",
        start_id, end_id
    );
    let storage_volumes = sqlx::query_as::<_, StorageVolume>(&query)
        .fetch_all(pool)
        .await
        .context("Failed to fetch storage volumes")?;
    Ok(storage_volumes)
}

/// Retrieves the total count of storage volumes
pub async fn count_storage_volumes(pool: &Pool<MySql>)
-> anyhow::Result<i64> {
    let query = "SELECT COUNT(*) FROM storage_volumes";
    let count: (i64,) = sqlx::query_as(query)
        .fetch_one(pool)
        .await
        .context("Failed to fetch storage volume count")?;
    Ok(count.0)
}
