use super::super::tables::App;
use anyhow::Context;
use sqlx::{MySql, Pool};

pub async fn list_apps(pool: &Pool<MySql>, page: i64, per_page: i64) -> anyhow::Result<Vec<App>> {
    println!("Attempting to fetch apps from database...");

    let result = sqlx::query_as::<_, App>("SELECT * FROM apps ORDER BY id ASC LIMIT ? OFFSET ?")
        .bind(per_page)
        .bind(page * per_page)
        .fetch_all(pool)
        .await;

    match result {
        Ok(apps) => {
            println!("Successfully fetched {} apps", apps.len());
            Ok(apps)
        }
        Err(e) => {
            eprintln!("Error fetching apps: {:#?}", e);
            Err(anyhow::Error::new(e).context("Failed to fetch apps"))
        }
    }
}

pub async fn get_app_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<App> {
    let app = sqlx::query_as::<_, App>("SELECT * FROM apps WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch app")?;

    Ok(app)
}

pub async fn get_apps_by_org(pool: &Pool<MySql>, org_id: i64) -> anyhow::Result<Vec<App>> {
    let apps =
        sqlx::query_as::<_, App>("SELECT * FROM apps WHERE org_id = ? ORDER BY created_at DESC")
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
    // Begin transaction
    let mut tx = pool.begin().await?;

    // Define query to insert app with default maintenance_mode set to false
    let app = sqlx::query_as::<_, App>(
        r#"INSERT INTO apps (
            name, org_id, git_repo, git_branch, container_image_url, region_id, maintenance_mode
        ) VALUES (?, ?, ?, ?, ?, ?, false)"#,
    )
    // Bind required parameters
    .bind(name)
    .bind(org_id)
    // Bind optional parameters
    .bind(git_repo)
    .bind(git_branch)
    .bind(container_image_url)
    .bind(region_id)
    // Execute query and handle errors
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create app")?;

    // Commit transaction
    tx.commit().await?;

    // Return newly created app
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
    // Define which fields are being updated
    let update_fields = [
        (name.is_some(), "name = ?"),
        (git_repo.is_some(), "git_repo = ?"),
        (git_branch.is_some(), "git_branch = ?"),
        (container_image_url.is_some(), "container_image_url = ?"),
        (region_id.is_some(), "region_id = ?"),
        (maintenance_mode.is_some(), "maintenance_mode = ?"),
    ];

    // Build update query with only the fields that have values
    let field_clauses = update_fields
        .iter()
        .filter(|(has_value, _)| *has_value)
        .map(|(_, field)| format!(", {}", field))
        .collect::<String>();

    let query = format!(
        "UPDATE apps SET updated_at = CURRENT_TIMESTAMP{} WHERE id = ?",
        field_clauses
    );

    // Start binding parameters
    let mut db_query = sqlx::query_as::<_, App>(&query);

    // Bind string parameters
    if let Some(val) = name {
        db_query = db_query.bind(val);
    }
    if let Some(val) = git_repo {
        db_query = db_query.bind(val);
    }
    if let Some(val) = git_branch {
        db_query = db_query.bind(val);
    }
    if let Some(val) = container_image_url {
        db_query = db_query.bind(val);
    }

    // Bind numeric/boolean parameters
    if let Some(val) = region_id {
        db_query = db_query.bind(val);
    }
    if let Some(val) = maintenance_mode {
        db_query = db_query.bind(val);
    }

    // Bind the ID parameter
    db_query = db_query.bind(id);

    // Execute the query in a transaction
    let mut tx = pool.begin().await?;
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
        "UPDATE apps SET maintenance_mode = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(maintenance_mode)
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .context(format!("Failed to update app {} maintenance mode", id))?;

    tx.commit().await?;
    Ok(app)
}
