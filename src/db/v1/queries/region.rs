use sqlx::{MySql, Pool};
use anyhow::Context;
use super::super::tables::Region;

pub async fn list_regions(
    pool: &Pool<MySql>, 
    limit: Option<i64>,
    offset: Option<i64>
) -> anyhow::Result<Vec<Region>> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT * FROM regions ORDER BY created_at DESC"
    );

    if limit.is_some() || offset.is_some() {
        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit.unwrap_or(100));
        
        if offset.is_some() {
            query_builder.push(" OFFSET ");
            query_builder.push_bind(offset.unwrap());
        }
    }

    let query = query_builder.build_query_as::<Region>();
    
    let regions = query
        .fetch_all(pool)
        .await
        .context("Failed to fetch regions")?;

    Ok(regions)
}

pub async fn get_region_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Region> {
    let region = sqlx::query_as::<_, Region>(
        "SELECT * FROM regions WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch region")?;

    Ok(region)
}

pub async fn create_region(
    pool: &Pool<MySql>,
    name: &str,
    provider: &str,
    status: &str,
) -> anyhow::Result<Region> {
    let mut tx = pool.begin().await?;

    let region = sqlx::query_as::<_, Region>(
        "INSERT INTO regions (name, provider, status) VALUES (?, ?, ?)"
    )
    .bind(name)
    .bind(provider)
    .bind(status)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create region")?;

    tx.commit().await?;
    Ok(region)
}

pub async fn update_region_status(
    pool: &Pool<MySql>,
    id: i64,
    status: &str,
) -> anyhow::Result<Region> {
    let mut tx = pool.begin().await?;

    let region = sqlx::query_as::<_, Region>(
        "UPDATE regions SET status = ? WHERE id = ?"
    )
    .bind(status)
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to update region status")?;

    tx.commit().await?;
    Ok(region)
}

pub async fn delete_region(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM regions WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete region")?;

    tx.commit().await?;
    Ok(())
}

pub async fn get_active_regions(pool: &Pool<MySql>) -> anyhow::Result<Vec<Region>> {
    let regions = sqlx::query_as::<_, Region>(
        "SELECT * FROM regions WHERE status = 'active' ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch active regions")?;

    Ok(regions)
}