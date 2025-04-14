use super::super::tables::{Region, ProviderRegion};
use anyhow::Context;
use sqlx::{MySql, Pool};

/// Retrieves a paginated list of deployment regions.
///
/// This function fetches regions from the database with optional pagination support.
/// Results are ordered by creation time with the most recently created regions first.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `limit` - Optional maximum number of regions to return (defaults to 100 if not specified)
/// * `offset` - Optional number of regions to skip (for pagination)
///
/// # Returns
///
/// * `Ok(Vec<Region>)` - Successfully retrieved list of regions
/// * `Err(anyhow::Error)` - Failed to fetch regions
///
/// # Dynamic Query Building
///
/// This function uses SQLx's QueryBuilder to dynamically construct a SQL query
/// based on whether pagination parameters are provided. This approach is more
/// efficient than building strings manually and protects against SQL injection.
///
/// # Pagination
///
/// When both `limit` and `offset` are provided, standard SQL pagination is applied.
/// If only `limit` is provided, just the first N records are returned.
/// If neither is provided, all regions are returned (with a safety limit of 100).
pub async fn list_regions(
    pool: &Pool<MySql>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> anyhow::Result<Vec<Region>> {
    let regions = sqlx::query_as::<_, Region>(
        "SELECT * FROM regions ORDER BY created_at DESC LIMIT ? OFFSET ?",
    )
    .bind(limit.unwrap_or(100))
    .bind(offset.unwrap_or(0))
    .fetch_all(pool)
    .await
    .context("Failed to fetch regions")?;

    Ok(regions)
}

pub async fn list_provider_regions(
    pool: &Pool<MySql>,
) -> anyhow::Result<Vec<ProviderRegion>> {
    let regions = sqlx::query_as::<_, ProviderRegion>(
        "SELECT regions.*, providers.name AS provider_name, providers_regions.status AS binding_status 
         FROM regions 
         JOIN providers ON regions.provider = providers.id 
         JOIN providers_regions ON regions.id = providers_regions.region_id AND providers.id = providers_regions.provider_id",
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch provider regions")?;

    Ok(regions)
}

/// Retrieves a specific region by its unique identifier.
///
/// This function fetches detailed information about a single region record.
/// It's typically used when specific region details are needed, such as
/// for displaying region information or resource allocation.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the region to retrieve
///
/// # Returns
///
/// * `Ok(Region)` - Successfully retrieved region information
/// * `Err(anyhow::Error)` - Failed to fetch region (including if not found)
///
/// # Error Handling
///
/// Returns an error if no region with the given ID exists or if a database
/// error occurs during the query execution.
pub async fn get_region_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Region> {
    let region = sqlx::query_as::<_, Region>("SELECT * FROM regions WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch region")?;

    Ok(region)
}

/// Creates a new deployment region in the system.
///
/// This function registers a new region for application deployments.
/// Regions typically represent geographical deployment locations or
/// distinct cloud provider environments.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `name` - Name of the region (typically a geographical identifier like "us-east", "eu-west")
/// * `provider` - Cloud or infrastructure provider (e.g., "aws", "gcp", "azure", "on-prem")
/// * `status` - Initial status of the region (e.g., "provisioning", "active", "maintenance")
///
/// # Returns
///
/// * `Ok(Region)` - Successfully created region record
/// * `Err(anyhow::Error)` - Failed to create region record
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Common Region States
///
/// Typical region status values include:
/// - "provisioning" - Region is being set up and not yet ready for deployments
/// - "active" - Region is fully operational and can accept deployments
/// - "maintenance" - Region is temporarily unavailable for new deployments
/// - "deprecated" - Region is being phased out, no new deployments accepted
/// - "unavailable" - Region is not currently operational
pub async fn create_region(
    pool: &Pool<MySql>,
    name: &str,
    provider: &str,
    status: &str,
) -> anyhow::Result<Region> {
    let mut tx = pool.begin().await?;

    let region = sqlx::query_as::<_, Region>(
        "INSERT INTO regions (name, provider, status) VALUES (?, ?, ?)",
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

/// Updates the status of an existing deployment region.
///
/// This function changes the operational status of a region, which affects
/// whether new deployments can be directed to it. Status changes are critical
/// operations that can affect application availability and deployment strategies.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the region to update
/// * `status` - New status for the region (e.g., "active", "maintenance", "unavailable")
///
/// # Returns
///
/// * `Ok(Region)` - Successfully updated region record
/// * `Err(anyhow::Error)` - Failed to update region
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Operational Impact
///
/// Changing a region's status may have significant operational impacts:
/// - Setting to "maintenance" or "unavailable" prevents new deployments
/// - Status changes should be coordinated with deployment schedules
/// - Monitoring systems may need to be updated based on region status
/// - Load balancers may need reconfiguration after status changes
pub async fn update_region_status(
    pool: &Pool<MySql>,
    id: i64,
    status: &str,
) -> anyhow::Result<Region> {
    let mut tx = pool.begin().await?;

    let region = sqlx::query_as::<_, Region>("UPDATE regions SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update region status")?;

    tx.commit().await?;
    Ok(region)
}

/// Deletes a deployment region from the system.
///
/// This function permanently removes a region record from the database.
/// It should be used with extreme caution, as it may affect deployed applications
/// and infrastructure allocation.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the region to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the region
/// * `Err(anyhow::Error)` - Failed to delete the region
///
/// # Warning
///
/// This operation is irreversible and potentially dangerous. Instead of deleting
/// regions, consider changing their status to "deprecated" or "unavailable" first,
/// and ensure no active deployments exist in the region before deletion.
///
/// # Cascading Effects
///
/// Depending on the database schema and application logic:
/// - Deployed applications in this region may lose their region reference
/// - Foreign key constraints may prevent deletion if the region is in use
/// - Monitoring, billing, and operational systems may be affected
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
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

/// Retrieves all active deployment regions.
///
/// This function fetches all regions with a status of "active", indicating
/// they are available for new deployments. It's typically used for deployment
/// target selection and region availability displays.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(Vec<Region>)` - Successfully retrieved list of active regions
/// * `Err(anyhow::Error)` - Failed to fetch active regions
///
/// # Use Cases
///
/// Common use cases include:
/// - Populating region selection dropdowns in deployment interfaces
/// - Determining valid deployment targets for automated processes
/// - Calculating resource availability across active regions
/// - Health status dashboards showing operational deployment locations
///
/// # Query Details
///
/// Results are filtered by status="active" and ordered by creation time,
/// with the most recently created regions appearing first in the list.
pub async fn get_active_regions(pool: &Pool<MySql>) -> anyhow::Result<Vec<Region>> {
    let regions = sqlx::query_as::<_, Region>(
        "SELECT * FROM regions WHERE status = 'active' ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch active regions")?;

    Ok(regions)
}