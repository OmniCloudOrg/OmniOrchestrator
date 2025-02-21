use sqlx::{MySql, Pool};
use anyhow::Context;
use super::super::tables::App;

pub async fn list_apps(pool: &Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let apps = sqlx::query_as::<_, App>(
        "SELECT * FROM apps ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch apps")?;

    Ok(apps)
}

pub async fn get_app_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<App> {
    let app = sqlx::query_as::<_, App>(
        "SELECT * FROM apps WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch app")?;

    Ok(app)
}

pub async fn get_apps_by_org(pool: &Pool<MySql>, org_id: i64) -> anyhow::Result<Vec<App>> {
    let apps = sqlx::query_as::<_, App>(
        "SELECT * FROM apps WHERE org_id = ? ORDER BY created_at DESC"
    )
    .bind(org_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch org apps")?;

    Ok(apps)
}

pub async fn create_app(
    pool: &Pool<MySql>,
    name: &str,
    org_id: i64,
    git_repo: Option<&str>,
    git_branch: Option<&str>,
    container_image_url: Option<&str>,
    region_id: Option<i64>,
) -> anyhow::Result<App> {
    let mut tx = pool.begin().await?;

    let app = sqlx::query_as::<_, App>(
        r#"INSERT INTO apps (
            name, org_id, git_repo, git_branch, container_image_url, region_id, maintenance_mode
        ) VALUES (?, ?, ?, ?, ?, ?, false)"#
    )
    .bind(name)
    .bind(org_id)
    .bind(git_repo)
    .bind(git_branch)
    .bind(container_image_url)
    .bind(region_id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create app")?;

    tx.commit().await?;
    Ok(app)
}

pub async fn update_app(
    pool: &Pool<MySql>,
    id: i64,
    name: Option<&str>,
    git_repo: Option<&str>,
    git_branch: Option<&str>,
    container_image_url: Option<&str>,
    region_id: Option<i64>,
    maintenance_mode: Option<bool>,
) -> anyhow::Result<App> {
    let mut tx = pool.begin().await?;

    let mut query = String::from("UPDATE apps SET updated_at = CURRENT_TIMESTAMP");
    
    if let Some(name) = name {
        query.push_str(", name = ?");
    }
    if let Some(git_repo) = git_repo {
        query.push_str(", git_repo = ?");
    }
    if let Some(git_branch) = git_branch {
        query.push_str(", git_branch = ?");
    }
    if let Some(container_image_url) = container_image_url {
        query.push_str(", container_image_url = ?");
    }
    if let Some(region_id) = region_id {
        query.push_str(", region_id = ?");
    }
    if let Some(maintenance_mode) = maintenance_mode {
        query.push_str(", maintenance_mode = ?");
    }
    
    query.push_str(" WHERE id = ?");

    let mut db_query = sqlx::query_as::<_, App>(&query);
    
    if let Some(name) = name {
        db_query = db_query.bind(name);
    }
    if let Some(git_repo) = git_repo {
        db_query = db_query.bind(git_repo);
    }
    if let Some(git_branch) = git_branch {
        db_query = db_query.bind(git_branch);
    }
    if let Some(container_image_url) = container_image_url {
        db_query = db_query.bind(container_image_url);
    }
    if let Some(region_id) = region_id {
        db_query = db_query.bind(region_id);
    }
    if let Some(maintenance_mode) = maintenance_mode {
        db_query = db_query.bind(maintenance_mode);
    }
    
    db_query = db_query.bind(id);

    let app = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update app")?;

    tx.commit().await?;
    Ok(app)
}

pub async fn delete_app(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM apps WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete app")?;

    tx.commit().await?;
    Ok(())
}

pub async fn set_maintenance_mode(
    pool: &Pool<MySql>,
    id: i64,
    maintenance_mode: bool,
) -> anyhow::Result<App> {
    let mut tx = pool.begin().await?;

    let app = sqlx::query_as::<_, App>(
        "UPDATE apps SET maintenance_mode = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    )
    .bind(maintenance_mode)
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to update app maintenance mode")?;

    tx.commit().await?;
    Ok(app)
}