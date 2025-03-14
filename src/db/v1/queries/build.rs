use sqlx::{FromRow, MySql, Pool};
use anyhow::Context;
use super::super::tables::Build;

// List builds (Paginated)
pub async fn list_builds_paginated(pool: &Pool<MySql>, per_page: i64, page: i64) -> anyhow::Result<Vec<Build>> {
    let builds = sqlx::query_as::<_, Build>(
        "SELECT * FROM builds ORDER BY id ASC LIMIT ? OFFSET ?"
    )
    .bind(per_page)
    .bind(page)
    .fetch_all(pool)
    .await
    .context("Failed to fetch builds")?;

    Ok(builds)
}

// List builds for a specific app (paginated)
pub async fn list_builds_for_app_paginated(pool: &Pool<MySql>, app_id: i64, per_page: i64, offset: i64) -> anyhow::Result<Vec<Build>> {
    let builds = sqlx::query_as::<_, Build>(
        "SELECT * FROM builds WHERE app_id = ? ORDER BY id ASC LIMIT ? OFFSET ?"
    )
    .bind(app_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await
    .context("Failed to fetch builds")?;

    Ok(builds)
}

// Get a build by ID
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

// Create a build
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

// Update a build
pub async fn update_build(
    pool: &Pool<MySql>,
    id: i64,
    status: &str,
    build_log: &str,
) -> anyhow::Result<Build> {
    let build = sqlx::query_as::<_, Build>(
        "UPDATE builds SET status = ?, build_log = ? WHERE id = ?"
    )
    .bind(status)
    .bind(build_log)
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to update build")?;

    Ok(build)
}

// Delete a build
pub async fn delete_build(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM builds WHERE id = ?")
    .bind(id)
    .execute(pool)
    .await
    .context("Failed to delete build")?;

    Ok(())
}

// Delete all builds for an app
pub async fn delete_builds_for_app(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM builds WHERE app_id = ?")
    .bind(app_id)
    .execute(pool)
    .await
    .context("Failed to delete builds for app")?;

    Ok(())
}