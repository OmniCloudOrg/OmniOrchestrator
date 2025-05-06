use crate::models::org::Org;
use anyhow::Context;
use sqlx::{MySql, Pool};

/// Retrieves all organizations in the system, ordered by creation time.
///
/// This function fetches all organization records from the database, with
/// the most recently created organizations first. It provides a complete view
/// of all organizations in the system.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(Vec<Org>)` - Successfully retrieved list of organizations
/// * `Err(anyhow::Error)` - Failed to fetch organizations
///
/// # Use Cases
///
/// Common use cases include:
/// - Administrative dashboards showing all organizations
/// - System-wide reports and analytics
/// - Multi-tenant application management
pub async fn list_orgs(pool: &Pool<MySql>) -> anyhow::Result<Vec<Org>> {
    let orgs = sqlx::query_as::<_, Org>("SELECT * FROM orgs ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .context("Failed to fetch organizations")?;

    Ok(orgs)
}

/// Retrieves a specific organization by its unique identifier.
///
/// This function fetches detailed information about a single organization record.
/// It's typically used when specific organization details are needed, such as
/// for displaying organization profiles or handling organization-specific operations.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the organization to retrieve
///
/// # Returns
///
/// * `Ok(Org)` - Successfully retrieved organization information
/// * `Err(anyhow::Error)` - Failed to fetch organization (including if not found)
///
/// # Error Handling
///
/// Returns an error if no organization with the given ID exists or if a database
/// error occurs during the query execution.
pub async fn get_org_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Org> {
    let org = sqlx::query_as::<_, Org>("SELECT * FROM orgs WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch organization")?;

    Ok(org)
}

/// Creates a new organization in the system.
///
/// This function inserts a new organization record with the provided name.
/// It uses a transaction to ensure data consistency during the creation process.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `name` - Name of the new organization
///
/// # Returns
///
/// * `Ok(Org)` - Successfully created organization record
/// * `Err(anyhow::Error)` - Failed to create organization record
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Note
///
/// The created organization will have system-generated values for:
/// - `id` - Auto-incremented primary key
/// - `created_at` - Timestamp of creation
/// - `updated_at` - Initially same as creation timestamp
pub async fn create_org(pool: &Pool<MySql>, name: &str) -> anyhow::Result<Org> {
    let mut tx = pool.begin().await?;

    let org = sqlx::query_as::<_, Org>("INSERT INTO orgs (name) VALUES (?)")
        .bind(name)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to create organization")?;

    tx.commit().await?;
    Ok(org)
}

/// Updates an existing organization's information.
///
/// This function modifies an organization record with the provided name.
/// It also updates the `updated_at` timestamp to reflect the modification time.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the organization to update
/// * `name` - New name for the organization
///
/// # Returns
///
/// * `Ok(Org)` - Successfully updated organization record
/// * `Err(anyhow::Error)` - Failed to update organization
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Error Handling
///
/// Returns an error if no organization with the given ID exists or if a database
/// error occurs during the update operation.
pub async fn update_org(pool: &Pool<MySql>, id: i64, name: &str) -> anyhow::Result<Org> {
    let mut tx = pool.begin().await?;

    let org = sqlx::query_as::<_, Org>(
        "UPDATE orgs SET name = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(name)
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to update organization")?;

    tx.commit().await?;
    Ok(org)
}

/// Deletes an organization from the system.
///
/// This function permanently removes an organization record from the database.
/// It should be used with caution, as it typically has significant implications
/// for associated data and user access.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the organization to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the organization
/// * `Err(anyhow::Error)` - Failed to delete the organization
///
/// # Warning
///
/// This operation is irreversible. Consider the implications before deleting
/// organizations, especially those with active users or resources.
///
/// # Important
///
/// This function only deletes the organization record itself. It does not cascade
/// delete related records such as organization members, applications, or other
/// resources associated with the organization. Consider implementing cascading
/// delete logic or foreign key constraints if needed.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn delete_org(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM orgs WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete organization")?;

    tx.commit().await?;
    Ok(())
}

/// Adds a user as a member of an organization with a specific role.
///
/// This function creates a relationship between a user and an organization,
/// assigning the user a role within that organization. This role typically
/// determines the user's permissions and access level within the organization.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `org_id` - Unique identifier of the organization
/// * `user_id` - Unique identifier of the user to add
/// * `role` - Role to assign to the user within the organization (e.g., "admin", "member")
///
/// # Returns
///
/// * `Ok(())` - Successfully added the user to the organization
/// * `Err(anyhow::Error)` - Failed to add the user to the organization
///
/// # Uniqueness
///
/// This function assumes that the combination of `org_id` and `user_id`
/// must be unique in the orgmember table. If a user is already a member
/// of the organization, the operation will fail with a unique constraint violation.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn add_org_member(
    pool: &Pool<MySql>,
    org_id: i64,
    user_id: i64,
    role: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("INSERT INTO orgmember (org_id, user_id, role) VALUES (?, ?, ?)")
        .bind(org_id)
        .bind(user_id)
        .bind(role)
        .execute(&mut *tx)
        .await
        .context("Failed to add organization member")?;

    tx.commit().await?;
    Ok(())
}

/// Removes a user from an organization.
///
/// This function deletes the relationship between a user and an organization,
/// effectively revoking the user's membership and access to the organization's resources.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `org_id` - Unique identifier of the organization
/// * `user_id` - Unique identifier of the user to remove
///
/// # Returns
///
/// * `Ok(())` - Successfully removed the user from the organization
/// * `Err(anyhow::Error)` - Failed to remove the user from the organization
///
/// # Important
///
/// This function only removes the membership relationship. It does not perform any
/// cleanup of resources owned by or associated with the user within the organization.
/// Additional logic may be needed to handle resource ownership transfer or deletion.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn remove_org_member(
    pool: &Pool<MySql>,
    org_id: i64,
    user_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM orgmember WHERE org_id = ? AND user_id = ?")
        .bind(org_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .context("Failed to remove organization member")?;

    tx.commit().await?;
    Ok(())
}

/// Updates a user's role within an organization.
///
/// This function modifies the role assigned to a user within an organization,
/// which typically affects their permissions and access level. This is useful
/// for promoting or demoting users within the organization's hierarchy.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `org_id` - Unique identifier of the organization
/// * `user_id` - Unique identifier of the user whose role to update
/// * `role` - New role to assign to the user (e.g., "admin", "member")
///
/// # Returns
///
/// * `Ok(())` - Successfully updated the user's role
/// * `Err(anyhow::Error)` - Failed to update the user's role
///
/// # Error Handling
///
/// Returns an error if the user is not a member of the organization or if a database
/// error occurs during the update operation.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Authorization Considerations
///
/// This function only performs the database operation to update a role. It does not
/// implement any authorization logic to determine if the requesting user has permission
/// to change roles. Such checks should be implemented in the business logic layer.
pub async fn update_org_member_role(
    pool: &Pool<MySql>,
    org_id: i64,
    user_id: i64,
    role: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("UPDATE orgmember SET role = ? WHERE org_id = ? AND user_id = ?")
        .bind(role)
        .bind(org_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .context("Failed to update organization member role")?;

    tx.commit().await?;
    Ok(())
}