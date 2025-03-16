use super::super::tables::{Build, Deployment};
use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, MySql, Pool};

//=============================================================================
// Build Operations
//=============================================================================

/// Retrieves all builds for a specific application, ordered by creation time.
///
/// This function fetches the complete build history for an application, with
/// the most recent builds first. It's useful for displaying the build timeline
/// and history of an application.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application whose builds to retrieve
///
/// # Returns
///
/// * `Ok(Vec<Build>)` - Successfully retrieved list of builds for the application
/// * `Err(anyhow::Error)` - Failed to fetch builds
///
/// # Note
///
/// This function returns all builds without pagination. For applications with
/// a large number of builds, consider using `get_app_builds` which supports
/// pagination.
pub async fn list_builds(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<Vec<Build>> {
    let builds = sqlx::query_as::<_, Build>(
        "SELECT * FROM builds WHERE app_id = ? ORDER BY created_at DESC",
    )
    .bind(app_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch builds")?;

    Ok(builds)
}

/// Retrieves a specific build by its unique identifier.
///
/// This function fetches detailed information about a single build record.
/// It's typically used when detailed build information is needed, such as
/// for displaying build details or for triggering a deployment based on a build.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the build to retrieve
///
/// # Returns
///
/// * `Ok(Build)` - Successfully retrieved build information
/// * `Err(anyhow::Error)` - Failed to fetch build (including if not found)
///
/// # Error Handling
///
/// Returns an error if no build with the given ID exists or if a database
/// error occurs during the query execution.
pub async fn get_build_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Build> {
    let build = sqlx::query_as::<_, Build>("SELECT * FROM builds WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch build")?;

    Ok(build)
}

/// Creates a new build record for an application.
///
/// This function initiates a build process by creating a build record in the database.
/// The build starts in a 'pending' status and can be updated as the build process progresses.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Identifier of the application this build belongs to
/// * `source_version` - Optional source version identifier (e.g., git commit hash or tag)
///
/// # Returns
///
/// * `Ok(Build)` - Successfully created build record
/// * `Err(anyhow::Error)` - Failed to create build record
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn create_build(
    pool: &Pool<MySql>,
    app_id: i64,
    source_version: Option<&str>,
) -> anyhow::Result<Build> {
    let mut tx = pool.begin().await?;

    let build = sqlx::query_as::<_, Build>(
        "INSERT INTO builds (app_id, source_version, status) VALUES (?, ?, 'pending')",
    )
    .bind(app_id)
    .bind(source_version)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create build")?;

    tx.commit().await?;
    Ok(build)
}

/// Updates the status and timing information of an existing build.
///
/// This function modifies a build record to reflect its current state and progress.
/// It's typically called during the build lifecycle to update status from 'pending'
/// to 'in_progress' to 'succeeded' or 'failed', along with appropriate timestamps.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the build to update
/// * `status` - New status of the build (e.g., 'pending', 'in_progress', 'succeeded', 'failed')
/// * `started_at` - Optional timestamp when the build process started
/// * `completed_at` - Optional timestamp when the build process completed
///
/// # Returns
///
/// * `Ok(Build)` - Successfully updated build record
/// * `Err(anyhow::Error)` - Failed to update build
///
/// # Status Lifecycle
///
/// A typical build status lifecycle might be:
/// 1. 'pending' - Build is queued but not yet started
/// 2. 'in_progress' - Build process has begun (set started_at)
/// 3. 'succeeded' or 'failed' - Build has completed (set completed_at)
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
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
        WHERE id = ?"#,
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

/// Retrieves the most recent successful build for an application.
///
/// This function finds the latest build with a 'succeeded' status for a given application.
/// It's commonly used for initiating deployments or for reporting the latest successful build.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application
///
/// # Returns
///
/// * `Ok(Build)` - Successfully retrieved the latest successful build
/// * `Err(anyhow::Error)` - Failed to fetch build or no successful build exists
///
/// # Error Handling
///
/// Returns an error if no successful builds exist for the application or
/// if a database error occurs during the query execution.
///
/// # Use Cases
///
/// Common use cases include:
/// - Determining what to deploy when a user requests "deploy latest build"
/// - Showing the latest valid build in the application dashboard
/// - Computing time since last successful build for metrics
pub async fn get_latest_successful_build(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<Build> {
    let build = sqlx::query_as::<_, Build>(
        r#"SELECT * FROM builds 
        WHERE app_id = ? AND status = 'succeeded'
        ORDER BY created_at DESC LIMIT 1"#,
    )
    .bind(app_id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch latest successful build")?;

    Ok(build)
}

/// Deletes a specific build from the database.
///
/// This function permanently removes a build record and should be used with caution.
/// It's typically used for cleanup operations or removing invalid builds.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the build to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the build
/// * `Err(anyhow::Error)` - Failed to delete the build
///
/// # Warning
///
/// This operation is irreversible. Consider the implications before deleting builds,
/// especially if they're referenced by deployments or other records.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
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

/// Retrieves a paginated list of builds for a specific application.
///
/// This function fetches builds associated with a particular application,
/// with pagination support. Results are ordered by creation time in descending order
/// (newest first).
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application whose builds to retrieve
/// * `limit` - Maximum number of builds to return
/// * `offset` - Number of builds to skip (for pagination)
///
/// # Returns
///
/// * `Ok(Vec<Build>)` - Successfully retrieved paginated list of builds
/// * `Err(anyhow::Error)` - Failed to fetch builds
///
/// # Pagination
///
/// The offset-based pagination works as follows:
/// - First page: offset=0, limit=N
/// - Second page: offset=N, limit=N
/// - Third page: offset=2*N, limit=N
///
/// # Use Cases
///
/// This function is preferred over `list_builds` when dealing with applications
/// that have a large number of builds, as it allows for efficient pagination.
pub async fn get_app_builds(
    pool: &Pool<MySql>,
    app_id: i64,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Build>> {
    let builds = sqlx::query_as::<_, Build>(
        r#"SELECT * FROM builds 
        WHERE app_id = ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?"#,
    )
    .bind(app_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .context("Failed to fetch app builds")?;

    Ok(builds)
}

//=============================================================================
// Deployment Operations
//=============================================================================

/// Retrieves all deployments for a specific application, ordered by creation time.
///
/// This function fetches the complete deployment history for an application,
/// with the most recent deployments first. It's useful for displaying the
/// deployment timeline and history of an application.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application whose deployments to retrieve
///
/// # Returns
///
/// * `Ok(Vec<Deployment>)` - Successfully retrieved list of deployments
/// * `Err(anyhow::Error)` - Failed to fetch deployments
///
/// # Note
///
/// This function returns all deployments without pagination. For applications with
/// a large number of deployments, consider using `get_app_deployments` which
/// supports pagination.
pub async fn list_deployments(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<Vec<Deployment>> {
    let deployments = sqlx::query_as::<_, Deployment>(
        "SELECT * FROM deployments WHERE app_id = ? ORDER BY created_at DESC",
    )
    .bind(app_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch deployments")?;

    Ok(deployments)
}

/// Retrieves a specific deployment by its unique identifier.
///
/// This function fetches detailed information about a single deployment record.
/// It's typically used when detailed deployment information is needed, such as
/// for displaying deployment details or checking deployment status.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the deployment to retrieve
///
/// # Returns
///
/// * `Ok(Deployment)` - Successfully retrieved deployment information
/// * `Err(anyhow::Error)` - Failed to fetch deployment (including if not found)
///
/// # Error Handling
///
/// Returns an error if no deployment with the given ID exists or if a database
/// error occurs during the query execution.
pub async fn get_deployment_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Deployment> {
    let deployment = sqlx::query_as::<_, Deployment>("SELECT * FROM deployments WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch deployment")?;

    Ok(deployment)
}

/// Creates a new deployment record for an application based on a specific build.
///
/// This function initiates a deployment process by creating a deployment record
/// in the database. The deployment starts in a 'pending' status and can be
/// updated as the deployment process progresses.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Identifier of the application this deployment belongs to
/// * `build_id` - Identifier of the build to deploy
///
/// # Returns
///
/// * `Ok(Deployment)` - Successfully created deployment record
/// * `Err(anyhow::Error)` - Failed to create deployment record
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Relationships
///
/// Each deployment is associated with:
/// - An application (app_id)
/// - A specific build (build_id) that is being deployed
///
/// This allows tracking which version of the application is deployed and when.
pub async fn create_deployment(
    pool: &Pool<MySql>,
    app_id: i64,
    build_id: i64,
) -> anyhow::Result<Deployment> {
    let mut tx = pool.begin().await?;

    let deployment = sqlx::query_as::<_, Deployment>(
        "INSERT INTO deployments (app_id, build_id, status) VALUES (?, ?, 'pending')",
    )
    .bind(app_id)
    .bind(build_id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create deployment")?;

    tx.commit().await?;
    Ok(deployment)
}

/// Updates the status and timing information of an existing deployment.
///
/// This function modifies a deployment record to reflect its current state and progress.
/// It's typically called during the deployment lifecycle to update status from 'pending'
/// to 'in_progress' to 'deployed' or 'failed', along with appropriate timestamps.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the deployment to update
/// * `status` - New status of the deployment (e.g., 'pending', 'in_progress', 'deployed', 'failed')
/// * `started_at` - Optional timestamp when the deployment process started
/// * `completed_at` - Optional timestamp when the deployment process completed
///
/// # Returns
///
/// * `Ok(Deployment)` - Successfully updated deployment record
/// * `Err(anyhow::Error)` - Failed to update deployment
///
/// # Status Lifecycle
///
/// A typical deployment status lifecycle might be:
/// 1. 'pending' - Deployment is queued but not yet started
/// 2. 'in_progress' - Deployment process has begun (set started_at)
/// 3. 'deployed' or 'failed' - Deployment has completed (set completed_at)
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
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
        WHERE id = ?"#,
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

/// Retrieves the most recent deployment for an application.
///
/// This function finds the latest deployment for a given application, regardless
/// of its status. It's commonly used for checking the current deployment status
/// or for display in application dashboards.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application
///
/// # Returns
///
/// * `Ok(Deployment)` - Successfully retrieved the latest deployment
/// * `Err(anyhow::Error)` - Failed to fetch deployment or no deployments exist
///
/// # Error Handling
///
/// Returns an error if no deployments exist for the application or
/// if a database error occurs during the query execution.
///
/// # Use Cases
///
/// Common use cases include:
/// - Showing the current deployment status in application dashboards
/// - Determining if a deployment is in progress
/// - Computing time since last deployment for metrics
pub async fn get_latest_deployment(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<Deployment> {
    let deployment = sqlx::query_as::<_, Deployment>(
        r#"SELECT * FROM deployments 
        WHERE app_id = ? 
        ORDER BY created_at DESC 
        LIMIT 1"#,
    )
    .bind(app_id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch latest deployment")?;

    Ok(deployment)
}

/// Retrieves successful deployments for a specific application.
///
/// This function fetches deployments with 'deployed' status for a given application,
/// limited to a specified number and ordered by creation time (newest first).
/// It's useful for displaying deployment history or for rollback operations.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application
/// * `limit` - Maximum number of successful deployments to retrieve
///
/// # Returns
///
/// * `Ok(Vec<Deployment>)` - Successfully retrieved list of deployments
/// * `Err(anyhow::Error)` - Failed to fetch deployments
///
/// # Use Cases
///
/// Common use cases include:
/// - Providing a list of deployments that can be rolled back to
/// - Showing deployment history in application dashboards
/// - Tracking successful deployment frequency for metrics
pub async fn get_successful_deployments(
    pool: &Pool<MySql>,
    app_id: i64,
    limit: i64,
) -> anyhow::Result<Vec<Deployment>> {
    let deployments = sqlx::query_as::<_, Deployment>(
        r#"SELECT * FROM deployments 
        WHERE app_id = ? AND status = 'deployed'
        ORDER BY created_at DESC 
        LIMIT ?"#,
    )
    .bind(app_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("Failed to fetch successful deployments")?;

    Ok(deployments)
}

/// Deletes a specific deployment from the database.
///
/// This function permanently removes a deployment record and should be used with caution.
/// It's typically used for cleanup operations or removing invalid deployments.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the deployment to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the deployment
/// * `Err(anyhow::Error)` - Failed to delete the deployment
///
/// # Warning
///
/// This operation is irreversible. Consider the implications before deleting deployments,
/// especially for applications in production environments.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
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

/// Retrieves a paginated list of deployments for a specific application.
///
/// This function fetches deployments associated with a particular application,
/// with pagination support. Results are ordered by creation time in descending order
/// (newest first).
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application whose deployments to retrieve
/// * `limit` - Maximum number of deployments to return
/// * `offset` - Number of deployments to skip (for pagination)
///
/// # Returns
///
/// * `Ok(Vec<Deployment>)` - Successfully retrieved paginated list of deployments
/// * `Err(anyhow::Error)` - Failed to fetch deployments
///
/// # Pagination
///
/// The offset-based pagination works as follows:
/// - First page: offset=0, limit=N
/// - Second page: offset=N, limit=N
/// - Third page: offset=2*N, limit=N
///
/// # Use Cases
///
/// This function is preferred over `list_deployments` when dealing with applications
/// that have a large number of deployments, as it allows for efficient pagination.
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
        LIMIT ? OFFSET ?"#,
    )
    .bind(app_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .context("Failed to fetch app deployments")?;

    Ok(deployments)
}

/// Retrieves a deployment with its associated build information.
///
/// This function fetches a deployment record together with the build it's
/// associated with, combining the data in a single query. This is more efficient
/// than making separate queries for deployment and build information.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `deployment_id` - Unique identifier of the deployment to retrieve
///
/// # Returns
///
/// * `Ok((Deployment, Build))` - Successfully retrieved deployment and build information
/// * `Err(anyhow::Error)` - Failed to fetch data (including if not found)
///
/// # Use Cases
///
/// Common use cases include:
/// - Displaying detailed deployment information including the build being deployed
/// - Getting complete information for logs or auditing
/// - Providing comprehensive information for deployment status pages
///
/// # Note
///
/// This function performs a JOIN operation to fetch both deployment and build data
/// in a single query, which is more efficient than making two separate database calls.
pub async fn get_deployment_with_build(
    pool: &Pool<MySql>,
    deployment_id: i64,
) -> anyhow::Result<(Deployment, Build)> {
    let row = sqlx::query(
        r#"SELECT d.*, b.* FROM deployments d
        JOIN builds b ON d.build_id = b.id
        WHERE d.id = ?"#,
    )
    .bind(deployment_id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch deployment with build")?;

    let deployment = Deployment::from_row(&row)?;
    let build = Build::from_row(&row)?;

    Ok((deployment, build))
}