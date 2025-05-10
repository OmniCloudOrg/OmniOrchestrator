use crate::models::instance::Instance;
use anyhow::Context;
use sqlx::{MySql, Pool};

/// List instances by `region_id` and `app_id` paginated by `page` and `per_page` using a where clause.
pub async fn list_instances_by_region(
    pool: &Pool<MySql>,
    region_id: i64,
    app_id: i64,
    page: i64,
    per_page: i64,
) -> anyhow::Result<Vec<Instance>> {
    let instances = sqlx::query_as::<_, Instance>(
        "SELECT * FROM instances WHERE region_id = ? AND app_id = ? LIMIT ?, ?",
    )
    .bind(region_id)
    .bind(app_id)
    .bind((page - 1) * per_page)
    .bind(per_page)
    .fetch_all(pool)
    .await
    .context("Failed to fetch instances")?;

    Ok(instances)
}

/// Counts the total number of instances across all applications.
/// 
/// This function returns the total count of instances in the database.
/// It's useful for monitoring overall resource allocation and usage.
pub async fn count_instances(pool: &Pool<MySql>) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM instances")
        .fetch_one(pool)
        .await
        .context("Failed to count instances")?;

    Ok(count)
}

/// Retrieves a specific instance by its unique identifier.
///
/// This function fetches detailed information about a single compute instance.
/// It's typically used when specific instance details are needed, such as
/// for status monitoring or management operations.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the instance to retrieve
///
/// # Returns
///
/// * `Ok(Instance)` - Successfully retrieved instance information
/// * `Err(anyhow::Error)` - Failed to fetch instance (including if not found)
///
/// # Error Handling
///
/// Returns an error if no instance with the given ID exists or if a database
/// error occurs during the query execution.
pub async fn get_instance_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Instance> {
    let instance = sqlx::query_as::<_, Instance>("SELECT * FROM instances WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch instance")?;

    Ok(instance)
}

/// Creates a new compute instance for an application.
///
/// This function provisions a new compute instance for an application with the
/// specified instance type. The instance is initially created with a 'provisioning'
/// status and 'running' instance_status.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Identifier of the application this instance belongs to
/// * `instance_type` - Type of instance to create (e.g., 'small', 'medium', 'large')
///
/// # Returns
///
/// * `Ok(Instance)` - Successfully created instance record
/// * `Err(anyhow::Error)` - Failed to create instance record
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # State Model
///
/// The instance is created with:
/// - `status`: 'provisioning' - Indicates the instance is being set up
/// - `instance_status`: 'running' - Indicates the intended operational state
///
/// These states will be updated as the instance progresses through its lifecycle.
pub async fn create_instance(
    pool: &Pool<MySql>,
    app_id: i64,
    instance_type: &str,
) -> anyhow::Result<Instance> {
    let mut tx = pool.begin().await?;

    let instance = sqlx::query_as::<_, Instance>(
        r#"INSERT INTO instances (
            app_id, instance_type, status, instance_status
        ) VALUES (?, ?, 'provisioning', 'running')"#,
    )
    .bind(app_id)
    .bind(instance_type)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create instance")?;

    tx.commit().await?;
    Ok(instance)
}

/// Updates the status and details of an existing instance.
///
/// This function modifies an instance record to reflect its current state and
/// associated runtime information. It's typically called during the instance
/// lifecycle as it changes state or is assigned to specific infrastructure.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the instance to update
/// * `status` - New provisioning status (e.g., 'provisioning', 'running', 'failed')
/// * `instance_status` - New operational status (e.g., 'running', 'stopped', 'terminated')
/// * `container_id` - Optional identifier of the container running the instance
/// * `node_name` - Optional name of the node hosting the instance
///
/// # Returns
///
/// * `Ok(Instance)` - Successfully updated instance record
/// * `Err(anyhow::Error)` - Failed to update instance
///
/// # Status Model
///
/// The instance has two status fields:
/// - `status`: Represents the provisioning lifecycle (provisioning, running, failed)
/// - `instance_status`: Represents the operational state (running, stopped, terminated)
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn update_instance_status(
    pool: &Pool<MySql>,
    id: i64,
    status: &str,
    instance_status: &str,
    container_id: Option<&str>,
    node_name: Option<&str>,
) -> anyhow::Result<Instance> {
    let mut tx = pool.begin().await?;

    let instance = sqlx::query_as::<_, Instance>(
        r#"UPDATE instances 
        SET status = ?, instance_status = ?, container_id = ?, node_name = ?, 
            updated_at = CURRENT_TIMESTAMP 
        WHERE id = ?"#,
    )
    .bind(status)
    .bind(instance_status)
    .bind(container_id)
    .bind(node_name)
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to update instance status")?;

    tx.commit().await?;
    Ok(instance)
}

/// Deletes a specific instance from the database.
///
/// This function permanently removes an instance record from the database.
/// It's typically used for cleanup operations after an instance has been 
/// terminated, or to remove invalid instances.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the instance to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the instance
/// * `Err(anyhow::Error)` - Failed to delete the instance
///
/// # Warning
///
/// This operation is irreversible and should generally only be performed after
/// ensuring that the actual compute resource has been properly terminated.
/// Otherwise, resource leaks may occur.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn delete_instance(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM instances WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete instance")?;

    tx.commit().await?;
    Ok(())
}

/// Retrieves all running instances for a specific application.
///
/// This function fetches all instances in the 'running' state for an application,
/// ordered by creation time with the most recent first. It's useful for monitoring
/// active compute resources and managing application scaling.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application whose running instances to retrieve
///
/// # Returns
///
/// * `Ok(Vec<Instance>)` - Successfully retrieved list of running instances
/// * `Err(anyhow::Error)` - Failed to fetch running instances
///
/// # Use Cases
///
/// Common use cases include:
/// - Monitoring active compute resources
/// - Load balancing traffic across running instances
/// - Determining if auto-scaling is necessary
/// - Checking application health through instance distribution
pub async fn get_running_instances(
    pool: &Pool<MySql>,
    app_id: i64,
) -> anyhow::Result<Vec<Instance>> {
    let instances = sqlx::query_as::<_, Instance>(
        r#"SELECT * FROM instances 
        WHERE app_id = ? AND instance_status = 'running'
        ORDER BY created_at DESC"#,
    )
    .bind(app_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch running instances")?;

    Ok(instances)
}

/// Counts the number of running instances for a specific application.
///
/// This function returns the total count of instances in the 'running' state
/// for an application. It's more efficient than fetching all instances and 
/// counting them, especially when only the count is needed.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application
///
/// # Returns
///
/// * `Ok(i64)` - Successfully retrieved count of running instances
/// * `Err(anyhow::Error)` - Failed to count running instances
///
/// # Use Cases
///
/// Common use cases include:
/// - Auto-scaling decisions based on current instance count
/// - Monitoring application capacity
/// - Enforcing instance limits based on account tier
/// - Billing calculations based on active instance time
pub async fn count_running_instances(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>(
        r#"SELECT COUNT(*) FROM instances 
        WHERE app_id = ? AND instance_status = 'running'"#,
    )
    .bind(app_id)
    .fetch_one(pool)
    .await
    .context("Failed to count running instances")?;

    Ok(count)
}

/// Terminates all running instances for a specific application.
///
/// This function marks all running instances of an application as 'terminated'.
/// It's typically used during application shutdown, maintenance, or redeployment
/// scenarios when all compute resources need to be released.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `app_id` - Unique identifier of the application whose instances to terminate
///
/// # Returns
///
/// * `Ok(())` - Successfully marked instances as terminated
/// * `Err(anyhow::Error)` - Failed to terminate instances
///
/// # Important
///
/// This function only updates the database records to reflect termination.
/// The actual termination of compute resources should be handled by a separate
/// process that reacts to these status changes.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Use Cases
///
/// Common scenarios for using this function include:
/// - Application shutdown or decommissioning
/// - Emergency resource release during cost overruns
/// - Preparing for maintenance or major version upgrades
/// - Responding to security incidents requiring isolation
pub async fn terminate_all_instances(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"UPDATE instances 
        SET status = 'terminated', 
            instance_status = 'terminated',
            updated_at = CURRENT_TIMESTAMP 
        WHERE app_id = ? AND instance_status = 'running'"#,
    )
    .bind(app_id)
    .execute(&mut *tx)
    .await
    .context("Failed to terminate instances")?;

    tx.commit().await?;
    Ok(())
}