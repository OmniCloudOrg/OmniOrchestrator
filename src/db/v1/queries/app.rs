use super::super::tables::{App, AppWithInstanceCount};
use anyhow::Context;
use sqlx::{MySql, Pool};

/// Retrieves a paginated list of applications from the database.
///
/// This function fetches a subset of applications based on pagination parameters,
/// ordering them by their ID in ascending order. Pagination helps manage large
/// datasets by retrieving only a specific "page" of results.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `page` - Zero-based page number (e.g., 0 for first page, 1 for second page)
/// * `per_page` - Number of records to fetch per page
///
/// # Returns
///
/// * `Ok(Vec<App>)` - Successfully retrieved list of applications
/// * `Err(anyhow::Error)` - Failed to fetch applications, with context
///
/// # Examples
///
/// ```
/// let apps = list_apps(&pool, 0, 10).await?; // Get first 10 apps
/// ```
pub async fn list_apps(pool: &Pool<MySql>, page: i64, per_page: i64) -> anyhow::Result<Vec<AppWithInstanceCount>> {
    println!("Attempting to fetch apps with instance counts from database...");

    let result = sqlx::query_as::<_, AppWithInstanceCount>(
        r#"
        SELECT 
            apps.*, 
            COUNT(instances.id) AS instance_count
        FROM 
            apps
        LEFT JOIN 
            instances ON instances.app_id = apps.id
        GROUP BY 
            apps.id
        ORDER BY 
            apps.id ASC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(per_page)
    .bind(page * per_page)
    .fetch_all(pool)
    .await;

    match result {
        Ok(apps) => {
            println!("Successfully fetched {} apps with instance counts", apps.len());
            Ok(apps)
        }
        Err(e) => {
            eprintln!("Error fetching apps with instance counts: {:#?}", e);
            Err(anyhow::Error::new(e).context("Failed to fetch apps with instance counts"))
        }
    }
}

/// Counts the total number of applications in the database.
///
/// This function retrieves the total count of applications, which can be useful
/// for pagination or reporting purposes.
pub async fn count_apps(pool: &Pool<MySql>) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM apps")
        .fetch_one(pool)
        .await
        .context("Failed to count apps")?;

    Ok(count)
}

/// Retrieves a specific application by its unique identifier.
///
/// This function fetches a single application record matching the provided ID.
/// It's typically used for retrieving detailed information about a specific application.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the application to retrieve
///
/// # Returns
///
/// * `Ok(App)` - Successfully retrieved application
/// * `Err(anyhow::Error)` - Failed to fetch application (including if not found)
///
/// # Error Handling
///
/// Returns an error if no application with the given ID exists or if a database
/// error occurs during the query execution.
pub async fn get_app_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<App> {
    let app = sqlx::query_as::<_, App>("SELECT * FROM apps WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch app")?;

    Ok(app)
}

/// Retrieves all applications belonging to a specific organization.
///
/// This function fetches all applications associated with the provided organization ID,
/// ordered by creation date in descending order (newest first). It's typically used
/// to display all applications owned by an organization.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `org_id` - Organization identifier to filter applications by
///
/// # Returns
///
/// * `Ok(Vec<App>)` - Successfully retrieved list of applications for the organization
/// * `Err(anyhow::Error)` - Failed to fetch applications
///
/// # Note
///
/// This function will return an empty vector if the organization exists but has no applications.
pub async fn get_apps_by_org(pool: &Pool<MySql>, org_id: i64) -> anyhow::Result<Vec<App>> {
    let apps =
        sqlx::query_as::<_, App>("SELECT * FROM apps WHERE org_id = ? ORDER BY created_at DESC")
            .bind(org_id)
            .fetch_all(pool)
            .await
            .context("Failed to fetch org apps")?;

    Ok(apps)
}

/// Creates a new application in the database.
///
/// This function inserts a new application record with the provided parameters.
/// It handles both required fields (name, organization ID) and optional fields.
/// The application is created with maintenance mode disabled by default.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `name` - Name of the application
/// * `org_id` - Organization ID that the application belongs to
/// * `git_repo` - Optional URL of the Git repository for the application
/// * `git_branch` - Optional branch name in the Git repository
/// * `container_image_url` - Optional URL for a container image
/// * `region_id` - Optional ID of the deployment region
///
/// # Returns
///
/// * `Ok(App)` - Successfully created application, including database-assigned fields
/// * `Err(anyhow::Error)` - Failed to create application
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
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

/// Updates an existing application in the database.
///
/// This function modifies an application record with the provided parameters.
/// It uses a dynamic SQL query that only updates fields for which values are provided,
/// leaving other fields unchanged. The updated_at timestamp is always updated
/// to reflect the modification time.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the application to update
/// * `name` - Optional new name for the application
/// * `git_repo` - Optional new Git repository URL
/// * `git_branch` - Optional new Git branch
/// * `container_image_url` - Optional new container image URL
/// * `region_id` - Optional new region ID
/// * `maintenance_mode` - Optional new maintenance mode status
///
/// # Returns
///
/// * `Ok(App)` - Successfully updated application with all current values
/// * `Err(anyhow::Error)` - Failed to update application
///
/// # Dynamic Query Building
///
/// The function dynamically constructs the UPDATE SQL statement based on which
/// parameters have values. This ensures the query only updates the fields that
/// need to change, improving efficiency and reducing the risk of unintended changes.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
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

/// Deletes an application from the database.
///
/// This function permanently removes an application record with the specified ID.
/// The operation is performed within a transaction to ensure data consistency.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the application to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the application
/// * `Err(anyhow::Error)` - Failed to delete the application
///
/// # Warning
///
/// This operation is irreversible. Once an application is deleted, all associated
/// data that depends on the application's existence may become invalid.
///
/// # Note
///
/// This function does not verify if the application exists before attempting deletion.
/// If the application does not exist, the operation will still succeed (as far as SQL is concerned),
/// but no rows will be affected.
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

/// Sets the maintenance mode status for an application.
///
/// This function updates only the maintenance_mode field of an application,
/// making it a more efficient alternative to update_app when only this field 
/// needs to change. When an application is in maintenance mode, it typically
/// displays a maintenance page to users instead of normal operation.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the application to update
/// * `maintenance_mode` - Whether maintenance mode should be enabled (true) or disabled (false)
///
/// # Returns
///
/// * `Ok(App)` - Successfully updated application with the new maintenance mode status
/// * `Err(anyhow::Error)` - Failed to update maintenance mode
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
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