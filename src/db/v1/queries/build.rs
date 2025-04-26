use super::super::tables::Build;
use anyhow::Context;
use sqlx::{FromRow, MySql, Pool};

/// Retrieves a paginated list of all builds in the system.
///
/// This function fetches builds with pagination support, ordering them by ID
/// in ascending order (oldest first). It provides a way to browse through
/// potentially large numbers of build records without loading them all at once.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `per_page` - Number of builds to return per page
/// * `page` - Zero-based page number (e.g., 0 for first page, 1 for second page)
///
/// # Returns
///
/// * `Ok(Vec<Build>)` - Successfully retrieved list of builds for the requested page
/// * `Err(anyhow::Error)` - Failed to fetch builds
///
/// # Pagination
///
/// The function calculates the appropriate OFFSET as `page * per_page`.
/// For example, with per_page = 10:
/// - page 0 → entries 0-9
/// - page 1 → entries 10-19
/// - page 2 → entries 20-29
pub async fn list_builds_paginated(
    pool: &Pool<MySql>,
    per_page: i64,
    page: i64,
) -> anyhow::Result<Vec<Build>> {
    let builds =
        sqlx::query_as::<_, Build>("SELECT * FROM builds ORDER BY id ASC LIMIT ? OFFSET ?")
            .bind(per_page)
            .bind(page)
            .fetch_all(pool)
            .await
            .context("Failed to fetch builds")?;

    Ok(builds)
}

/// Retrieves the total number of builds in the system.
pub async fn get_total_build_count(pool: &Pool<MySql>) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM builds")
        .fetch_one(pool)
        .await
        .context("Failed to fetch build count")?;

    Ok(count)
}


/// Retrieves a paginated list of builds for a specific application.
///
/// This function fetches builds associated with a particular application,
/// with pagination support. Results are ordered by build ID in ascending order
/// (oldest first). This is useful for viewing the build history of a specific app.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application whose builds to retrieve
/// * `per_page` - Number of builds to return per page
/// * `offset` - Number of builds to skip (for pagination)
///
/// # Returns
///
/// * `Ok(Vec<Build>)` - Successfully retrieved list of builds for the application
/// * `Err(anyhow::Error)` - Failed to fetch builds
///
/// # Note
///
/// Unlike `list_builds_paginated`, this function uses a direct offset value
/// rather than calculating it from a page number. The caller must calculate
/// the appropriate offset based on their pagination scheme (typically `page * per_page`).
pub async fn list_builds_for_app_paginated(
    pool: &Pool<MySql>,
    app_id: i64,
    per_page: i64,
    offset: i64,
) -> anyhow::Result<Vec<Build>> {
    let builds = sqlx::query_as::<_, Build>(
        "SELECT * FROM builds WHERE app_id = ? ORDER BY id ASC LIMIT ? OFFSET ?",
    )
    .bind(app_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await
    .context("Failed to fetch builds")?;

    Ok(builds)
}

/// Retrieves a specific build by its unique identifier.
///
/// This function fetches detailed information about a single build record.
/// It's typically used when specific build details are needed, such as
/// viewing build logs or checking build status.
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

/// Creates a new build record in the database.
///
/// This function inserts a new build entry with the provided parameters.
/// It's typically called when a new build process is initiated for an application.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Identifier of the application this build belongs to
/// * `git_commit` - Git commit hash or identifier for this build
/// * `git_branch` - Git branch name used for this build
/// * `git_repo` - Git repository URL or identifier
/// * `status` - Initial status of the build (e.g., "pending", "in_progress")
/// * `build_log` - Initial build log content (may be empty or contain setup information)
///
/// # Returns
///
/// * `Ok(Vec<Build>)` - Successfully created build record(s)
/// * `Err(anyhow::Error)` - Failed to create build record
///
/// # Note
///
/// The function returns a vector of builds, which is unusual for a creation operation
/// that typically returns a single record. This may be due to specific implementation
/// requirements or to accommodate batch creation scenarios.
///
/// # Important
///
/// This function doesn't take a transaction parameter, so it commits changes
/// immediately. For operations that need to be part of a larger transaction,
/// consider enhancing this function to accept a transaction parameter.
pub async fn create_build(
    pool: &Pool<MySql>,
    app_id: i64,
    git_commit: &str,
    git_branch: &str,
    git_repo: &str,
    status: &str,
    build_log: &str,
) -> anyhow::Result<Vec<Build>> {
    let builds = sqlx::query_as::<_, Build>(
        "INSERT INTO builds (app_id, git_commit, git_branch, git_repo, status, build_log) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(app_id)
    .bind(git_commit)
    .bind(git_branch)
    .bind(git_repo)
    .bind(status)
    .bind(build_log)
    .fetch_all(pool)
    .await
    .context("Failed to update build")?;

    Ok(builds)
}

/// Updates an existing build record with new status and log information.
///
/// This function modifies a build record to reflect the current state of the
/// build process. It's typically called during or after a build process to
/// update its status and append to the build log.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the build to update
/// * `status` - New status of the build (e.g., "success", "failed", "in_progress")
/// * `build_log` - Updated build log content
///
/// # Returns
///
/// * `Ok(Build)` - Successfully updated build record
/// * `Err(anyhow::Error)` - Failed to update build
///
/// # Use Cases
///
/// Common use cases for this function include:
/// - Updating build status as it progresses through different stages
/// - Appending build output to the log as it becomes available
/// - Marking a build as complete with its final status
///
/// # Note
///
/// This function replaces the entire build log content rather than appending to it.
/// If incremental updates are needed, the caller should fetch the current log,
/// append to it, and then pass the complete updated log to this function.
pub async fn update_build(
    pool: &Pool<MySql>,
    id: i64,
    status: &str,
    build_log: &str,
) -> anyhow::Result<Build> {
    let build =
        sqlx::query_as::<_, Build>("UPDATE builds SET status = ?, build_log = ? WHERE id = ?")
            .bind(status)
            .bind(build_log)
            .bind(id)
            .fetch_one(pool)
            .await
            .context("Failed to update build")?;

    Ok(build)
}

/// Deletes a specific build record from the database.
///
/// This function permanently removes a build record identified by its ID.
/// It's typically used for cleanup operations or when a build was created erroneously.
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
/// This operation is irreversible. Once a build is deleted, all associated
/// information including build logs and status history is permanently lost.
///
/// # Note
///
/// This function does not verify if the build exists before attempting deletion.
/// If the build does not exist, the operation will still succeed (as far as SQL is concerned),
/// but no rows will be affected.
pub async fn delete_build(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM builds WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .context("Failed to delete build")?;

    Ok(())
}

/// Deletes all build records associated with a specific application.
///
/// This function permanently removes all build records for a given application.
/// It's typically used when an application is being deleted, or when a complete
/// build history reset is desired.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application whose builds should be deleted
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the builds for the application
/// * `Err(anyhow::Error)` - Failed to delete the builds
///
/// # Warning
///
/// This operation is irreversible and bulk in nature. It will delete all build
/// records for the specified application without any additional confirmation.
/// Use with caution, especially in production environments.
///
/// # Use Cases
///
/// Common scenarios for using this function include:
/// - Application deletion (cleanup of associated data)
/// - Build history purging for storage optimization
/// - Resetting an application's build history before migration or major changes
///
/// # Note
///
/// If the application has no builds, this operation will succeed but affect zero rows.
pub async fn delete_builds_for_app(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM builds WHERE app_id = ?")
        .bind(app_id)
        .execute(pool)
        .await
        .context("Failed to delete builds for app")?;

    Ok(())
}