// db/queries/deployment.rs
use crate::models::deployment::Deployment;
use anyhow::Context;
use sqlx::{MySql, Pool};

/// Retrieves a paginated list of deployments from the database.
pub async fn list_deployments(pool: &Pool<MySql>, page: i64, per_page: i64) -> anyhow::Result<Vec<Deployment>> {
    println!("Attempting to fetch deployments from database...");

    let result = sqlx::query_as::<_, Deployment>(
        r#"
        SELECT *
        FROM deployments
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(per_page)
    .bind(page * per_page)
    .fetch_all(pool)
    .await;

    match result {
        Ok(deployments) => {
            println!("Successfully fetched {} deployments", deployments.len());
            Ok(deployments)
        }
        Err(e) => {
            eprintln!("Error fetching deployments: {:#?}", e);
            Err(anyhow::Error::new(e).context("Failed to fetch deployments"))
        }
    }
}

/// Counts the total number of deployments in the database.
pub async fn count_deployments(pool: &Pool<MySql>) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM deployments")
        .fetch_one(pool)
        .await
        .context("Failed to count deployments")?;

    Ok(count)
}

/// Retrieves a specific deployment by its unique identifier.
pub async fn get_deployment_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Deployment> {
    let deployment = sqlx::query_as::<_, Deployment>("SELECT * FROM deployments WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch deployment")?;

    Ok(deployment)
}

/// Retrieves all deployments for a specific application with pagination.
pub async fn list_deployments_by_app(
    pool: &Pool<MySql>, 
    app_id: i64, 
    page: i64, 
    per_page: i64
) -> anyhow::Result<Vec<Deployment>> {
    let deployments = sqlx::query_as::<_, Deployment>(
        "SELECT * FROM deployments WHERE app_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
    )
        .bind(app_id)
        .bind(per_page)
        .bind(page * per_page)
        .fetch_all(pool)
        .await
        .context("Failed to fetch app deployments")?;

    Ok(deployments)
}

/// Counts the number of deployments for a specific application.
pub async fn count_deployments_by_app(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM deployments WHERE app_id = ?"
    )
    .bind(app_id)
    .fetch_one(pool)
    .await
    .context("Failed to count deployments by app_id")?;

    Ok(count)
}

/// Creates a new deployment in the database.
pub async fn create_deployment(
    pool: &Pool<MySql>,
    app_id: i64,
    build_id: i64,
    version: &str,
    deployment_strategy: &str,
    previous_deployment_id: Option<i64>,
    canary_percentage: Option<i64>,
    environment_variables: Option<serde_json::Value>,
    annotations: Option<serde_json::Value>,
    labels: Option<serde_json::Value>,
    created_by: Option<i64>,
) -> anyhow::Result<Deployment> {
    // Begin transaction
    let mut tx = pool.begin().await?;

    // Insert new deployment
    let deployment = sqlx::query_as::<_, Deployment>(
        r#"INSERT INTO deployments (
            app_id, build_id, version, status, deployment_strategy, 
            previous_deployment_id, canary_percentage, environment_variables,
            annotations, labels, created_at, created_by
        ) VALUES (?, ?, ?, 'pending', ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, ?)"#,
    )
    .bind(app_id)
    .bind(build_id)
    .bind(version)
    .bind(deployment_strategy)
    .bind(previous_deployment_id)
    .bind(canary_percentage)
    .bind(environment_variables)
    .bind(annotations)
    .bind(labels)
    .bind(created_by)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create deployment")?;

    // Commit transaction
    tx.commit().await?;

    // Return newly created deployment
    Ok(deployment)
}

/// Updates the status of an existing deployment.
pub async fn update_deployment_status(
    pool: &Pool<MySql>,
    id: i64,
    status: &str,
    error_message: Option<&str>,
) -> anyhow::Result<Deployment> {
    let mut tx = pool.begin().await?;

    // Update fields based on the new status
    let deployment = match status {
        "in_progress" => {
            sqlx::query_as::<_, Deployment>(
                "UPDATE deployments SET status = ?, started_at = CURRENT_TIMESTAMP WHERE id = ?"
            )
            .bind(status)
            .bind(id)
            .fetch_one(&mut *tx)
            .await
        },
        "deployed" | "failed" | "canceled" => {
            sqlx::query_as::<_, Deployment>(
                "UPDATE deployments SET status = ?, completed_at = CURRENT_TIMESTAMP, 
                deployment_duration = TIMESTAMPDIFF(SECOND, started_at, CURRENT_TIMESTAMP),
                error_message = ? WHERE id = ?"
            )
            .bind(status)
            .bind(error_message)
            .bind(id)
            .fetch_one(&mut *tx)
            .await
        },
        _ => {
            sqlx::query_as::<_, Deployment>(
                "UPDATE deployments SET status = ? WHERE id = ?"
            )
            .bind(status)
            .bind(id)
            .fetch_one(&mut *tx)
            .await
        }
    }.context("Failed to update deployment status")?;

    tx.commit().await?;
    Ok(deployment)
}

/// Deletes a deployment from the database.
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