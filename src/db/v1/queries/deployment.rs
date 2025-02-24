use sqlx::{FromRow, MySql, Pool};
use anyhow::Context;
use chrono::{DateTime, Utc};
use super::super::tables::{Build, Deployment};

// Build Operations
pub async fn list_builds(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<Vec<Build>> {
    let builds = sqlx::query_as::<_, Build>(
        "SELECT * FROM builds WHERE app_id = ? ORDER BY created_at DESC"
    )
    .bind(app_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch builds")?;

    Ok(builds)
}

pub async fn get_build_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Build> {
    let build = sqlx::query_as::<_, Build>(
        "SELECT * FROM builds WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch build")?;

    Ok(build)
}

pub async fn create_build(
    pool: &Pool<MySql>,
    app_id: i64,
    source_version: Option<&str>,
) -> anyhow::Result<Build> {
    let mut tx = pool.begin().await?;

    let build = sqlx::query_as::<_, Build>(
        "INSERT INTO builds (app_id, source_version, status) VALUES (?, ?, 'pending')"
    )
    .bind(app_id)
    .bind(source_version)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create build")?;

    tx.commit().await?;
    Ok(build)
}

pub async fn update_build_status(
    pool: &Pool<MySql>,
    id: i64,
    status: &str,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
) -> anyhow::Result<Build> {
    let mut tx = pool.begin().await?;

    let build = sqlx::query_as::<_, Build>(
        r#"UPDATE builds 
        SET status = ?, started_at = ?, completed_at = ?
        WHERE id = ?"#
    )
    .bind(status)
    .bind(started_at)
    .bind(completed_at)
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to update build status")?;

    tx.commit().await?;
    Ok(build)
}

pub async fn get_latest_successful_build(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<Build> {
    let build = sqlx::query_as::<_, Build>(
        r#"SELECT * FROM builds 
        WHERE app_id = ? AND status = 'succeeded'
        ORDER BY created_at DESC LIMIT 1"#
    )
    .bind(app_id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch latest successful build")?;

    Ok(build)
}

pub async fn delete_build(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM builds WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete build")?;

    tx.commit().await?;
    Ok(())
}

pub async fn get_app_builds(
    pool: &Pool<MySql>, 
    app_id: i64,
    limit: i64,
    offset: i64
) -> anyhow::Result<Vec<Build>> {
    let builds = sqlx::query_as::<_, Build>(
        r#"SELECT * FROM builds 
        WHERE app_id = ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?"#
    )
    .bind(app_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .context("Failed to fetch app builds")?;

    Ok(builds)
}

// Deployment Operations
pub async fn list_deployments(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<Vec<Deployment>> {
    let deployments = sqlx::query_as::<_, Deployment>(
        "SELECT * FROM deployments WHERE app_id = ? ORDER BY created_at DESC"
    )
    .bind(app_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch deployments")?;

    Ok(deployments)
}

pub async fn get_deployment_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Deployment> {
    let deployment = sqlx::query_as::<_, Deployment>(
        "SELECT * FROM deployments WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch deployment")?;

    Ok(deployment)
}

pub async fn create_deployment(
    pool: &Pool<MySql>,
    app_id: i64,
    build_id: i64,
) -> anyhow::Result<Deployment> {
    let mut tx = pool.begin().await?;

    let deployment = sqlx::query_as::<_, Deployment>(
        "INSERT INTO deployments (app_id, build_id, status) VALUES (?, ?, 'pending')"
    )
    .bind(app_id)
    .bind(build_id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create deployment")?;

    tx.commit().await?;
    Ok(deployment)
}

pub async fn update_deployment_status(
    pool: &Pool<MySql>,
    id: i64,
    status: &str,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
) -> anyhow::Result<Deployment> {
    let mut tx = pool.begin().await?;

    let deployment = sqlx::query_as::<_, Deployment>(
        r#"UPDATE deployments 
        SET status = ?, started_at = ?, completed_at = ?
        WHERE id = ?"#
    )
    .bind(status)
    .bind(started_at)
    .bind(completed_at)
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to update deployment status")?;

    tx.commit().await?;
    Ok(deployment)
}

pub async fn get_latest_deployment(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<Deployment> {
    let deployment = sqlx::query_as::<_, Deployment>(
        r#"SELECT * FROM deployments 
        WHERE app_id = ? 
        ORDER BY created_at DESC 
        LIMIT 1"#
    )
    .bind(app_id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch latest deployment")?;

    Ok(deployment)
}

pub async fn get_successful_deployments(
    pool: &Pool<MySql>,
    app_id: i64,
    limit: i64,
) -> anyhow::Result<Vec<Deployment>> {
    let deployments = sqlx::query_as::<_, Deployment>(
        r#"SELECT * FROM deployments 
        WHERE app_id = ? AND status = 'deployed'
        ORDER BY created_at DESC 
        LIMIT ?"#
    )
    .bind(app_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("Failed to fetch successful deployments")?;

    Ok(deployments)
}

pub async fn delete_deployment(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM deployments WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete deployment")?;

    tx.commit().await?;
    Ok(())
}

pub async fn get_app_deployments(
    pool: &Pool<MySql>,
    app_id: i64,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Deployment>> {
    let deployments = sqlx::query_as::<_, Deployment>(
        r#"SELECT * FROM deployments 
        WHERE app_id = ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?"#
    )
    .bind(app_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .context("Failed to fetch app deployments")?;

    Ok(deployments)
}

pub async fn get_deployment_with_build(
    pool: &Pool<MySql>,
    deployment_id: i64,
) -> anyhow::Result<(Deployment, Build)> {
    let row = sqlx::query(
        r#"SELECT d.*, b.* FROM deployments d
        JOIN builds b ON d.build_id = b.id
        WHERE d.id = ?"#)
        .bind(deployment_id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch deployment with build")?;

    let deployment = Deployment::from_row(&row)?;
    let build = Build::from_row(&row)?;

    Ok((deployment, build))
}