use super::super::tables::{Permission, Role};
use anyhow::Context;
use sqlx::{MySql, Pool};

//=============================================================================
// Role Operations
//=============================================================================

/// Retrieves all roles in the system, ordered by creation time.
///
/// This function fetches all role records from the database, with
/// the most recently created roles first. It provides a complete view
/// of all roles defined in the system.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(Vec<Role>)` - Successfully retrieved list of roles
/// * `Err(anyhow::Error)` - Failed to fetch roles
///
/// # Use Cases
///
/// Common use cases include:
/// - Administrative interfaces for role management
/// - Role selection dropdowns in user management interfaces
/// - System audit and compliance reporting
pub async fn list_roles(pool: &Pool<MySql>) -> anyhow::Result<Vec<Role>> {
    let roles = sqlx::query_as::<_, Role>("SELECT * FROM roles ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .context("Failed to fetch roles")?;

    Ok(roles)
}

/// Retrieves a specific role by its unique identifier.
///
/// This function fetches detailed information about a single role record.
/// It's typically used when specific role details are needed, such as
/// for displaying role information or editing role properties.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the role to retrieve
///
/// # Returns
///
/// * `Ok(Role)` - Successfully retrieved role information
/// * `Err(anyhow::Error)` - Failed to fetch role (including if not found)
///
/// # Error Handling
///
/// Returns an error if no role with the given ID exists or if a database
/// error occurs during the query execution.
pub async fn get_role_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Role> {
    let role = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch role")?;

    Ok(role)
}

/// Creates a new role in the system.
///
/// This function inserts a new role record with the provided name and description.
/// Roles are a fundamental component of the role-based access control (RBAC) system,
/// used to group related permissions and assign them to users.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `name` - Name of the new role (should be unique and descriptive)
/// * `description` - Optional description explaining the role's purpose
///
/// # Returns
///
/// * `Ok(Role)` - Successfully created role record
/// * `Err(anyhow::Error)` - Failed to create role record
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Note
///
/// Creating a role doesn't automatically assign any permissions to it.
/// Use `assign_permission_to_role` to associate permissions with the newly created role.
pub async fn create_role(
    pool: &Pool<MySql>,
    name: &str,
    description: Option<&str>,
) -> anyhow::Result<Role> {
    let mut tx = pool.begin().await?;

    let role = sqlx::query_as::<_, Role>("INSERT INTO roles (name, description) VALUES (?, ?)")
        .bind(name)
        .bind(description)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to create role")?;

    tx.commit().await?;
    Ok(role)
}

/// Updates an existing role's information.
///
/// This function modifies a role record with the provided name and/or description.
/// It supports partial updates, allowing you to update only the fields that need changing.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the role to update
/// * `name` - Optional new name for the role
/// * `description` - Optional new description for the role
///
/// # Returns
///
/// * `Ok(Role)` - Successfully updated role record
/// * `Err(anyhow::Error)` - Failed to update role
///
/// # Dynamic Query Building
///
/// This function dynamically builds an SQL query based on which parameters are provided.
/// Only the fields specified with Some values will be updated, while None values are ignored.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn update_role(
    pool: &Pool<MySql>,
    id: i64,
    name: Option<&str>,
    description: Option<&str>,
) -> anyhow::Result<Role> {
    let mut tx = pool.begin().await?;

    // Start with base query
    let mut query = String::from("UPDATE roles SET id = id");

    // Add clauses for provided fields
    if let Some(name) = name {
        query.push_str(", name = ?");
    }
    if let Some(description) = description {
        query.push_str(", description = ?");
    }

    // Add WHERE clause
    query.push_str(" WHERE id = ?");

    // Prepare the query
    let mut db_query = sqlx::query_as::<_, Role>(&query);

    // Bind parameters for provided fields
    if let Some(name) = name {
        db_query = db_query.bind(name);
    }
    if let Some(description) = description {
        db_query = db_query.bind(description);
    }

    // Bind the ID parameter
    db_query = db_query.bind(id);

    // Execute the query
    let role = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update role")?;

    tx.commit().await?;
    Ok(role)
}

/// Deletes a role from the system.
///
/// This function permanently removes a role record from the database.
/// It should be used with caution, as it affects user permissions and
/// may impact system access for users with this role.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the role to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the role
/// * `Err(anyhow::Error)` - Failed to delete the role
///
/// # Warning
///
/// This operation is irreversible. Before deleting a role, consider:
/// - Users with this role will lose the associated permissions
/// - Relationships in role_user and permissions_role tables may need to be cleaned up
/// - System functionality may be affected if critical roles are removed
///
/// # Important
///
/// Depending on the database schema, this operation may:
/// - Fail if foreign key constraints are enforced and the role is in use
/// - Leave orphaned records if cascading deletes are not configured
/// - Remove user access to system features
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn delete_role(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM roles WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete role")?;

    tx.commit().await?;
    Ok(())
}

//=============================================================================
// Permission Operations
//=============================================================================

/// Retrieves all permissions in the system, ordered by ID.
///
/// This function fetches all permission records from the database, providing
/// a complete view of all defined permissions in the system. The results are
/// ordered by ID in ascending order, which typically reflects the order in
/// which permissions were created.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(Vec<Permission>)` - Successfully retrieved list of permissions
/// * `Err(anyhow::Error)` - Failed to fetch permissions
///
/// # Use Cases
///
/// Common use cases include:
/// - Administrative interfaces for permission management
/// - Permission assignment interfaces when configuring roles
/// - System audit and compliance reporting
pub async fn list_permissions(pool: &Pool<MySql>) -> anyhow::Result<Vec<Permission>> {
    let permissions = sqlx::query_as::<_, Permission>("SELECT * FROM permissions ORDER BY id ASC")
        .fetch_all(pool)
        .await
        .context("Failed to fetch permissions")?;

    Ok(permissions)
}

/// Retrieves a specific permission by its unique identifier.
///
/// This function fetches detailed information about a single permission record.
/// It's typically used when specific permission details are needed, such as
/// for displaying permission information or checking permission properties.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the permission to retrieve
///
/// # Returns
///
/// * `Ok(Permission)` - Successfully retrieved permission information
/// * `Err(anyhow::Error)` - Failed to fetch permission (including if not found)
///
/// # Error Handling
///
/// Returns an error if no permission with the given ID exists or if a database
/// error occurs during the query execution.
pub async fn get_permission_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Permission> {
    let permission = sqlx::query_as::<_, Permission>("SELECT * FROM permissions WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch permission")?;

    Ok(permission)
}

/// Creates a new permission in the system.
///
/// This function inserts a new permission record with the provided name,
/// description, and resource type. Permissions are a fundamental component
/// of the role-based access control (RBAC) system, representing specific
/// actions that can be performed on system resources.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `name` - Name of the new permission (should be unique and descriptive)
/// * `description` - Optional description explaining the permission's purpose
/// * `resource_type` - Type of resource this permission applies to (e.g., "app", "user", "deployment")
///
/// # Returns
///
/// * `Ok(Permission)` - Successfully created permission record
/// * `Err(anyhow::Error)` - Failed to create permission record
///
/// # Permission Naming
///
/// Permission names are typically formatted as verb-noun pairs describing an action
/// on a resource type, such as "create-app", "read-user", or "deploy-application".
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Note
///
/// Creating a permission doesn't automatically assign it to any roles.
/// Use `assign_permission_to_role` to associate the permission with roles.
pub async fn create_permission(
    pool: &Pool<MySql>,
    name: &str,
    description: Option<String>,
    resource_type: String,
) -> anyhow::Result<Permission> {
    let mut tx = pool.begin().await?;

    let permission = sqlx::query_as::<_, Permission>(
        "INSERT INTO permissions (name, description, resource_type) VALUES (?, ?, ?)",
    )
    .bind(name)
    .bind(description)
    .bind(resource_type)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create permission")?;

    tx.commit().await?;
    Ok(permission)
}

/// Updates an existing permission's information.
///
/// This function modifies a permission record with the provided name,
/// description, and/or resource type. It supports partial updates,
/// allowing you to update only the fields that need changing.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the permission to update
/// * `name` - Optional new name for the permission
/// * `description` - Optional new description for the permission
/// * `resource_type` - Optional new resource type for the permission
///
/// # Returns
///
/// * `Ok(Permission)` - Successfully updated permission record
/// * `Err(anyhow::Error)` - Failed to update permission
///
/// # Dynamic Query Building
///
/// This function dynamically builds an SQL query based on which parameters
/// are provided. Only the fields specified with Some values will be updated,
/// while None values are ignored.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Important
///
/// Changing a permission's properties may affect the behavior of the RBAC system.
/// Consider the impact on existing roles and users before making changes,
/// especially to the name or resource_type fields.
pub async fn update_permission(
    pool: &Pool<MySql>,
    id: i64,
    name: Option<&str>,
    description: Option<&str>,
    resource_type: Option<&str>,
) -> anyhow::Result<Permission> {
    // Define which fields are being updated
    let update_fields = [
        (name.is_some(), "name = ?"),
        (description.is_some(), "description = ?"),
        (resource_type.is_some(), "resource_type = ?"),
    ];

    // Build update query with only the fields that have values
    let field_clauses = update_fields
        .iter()
        .filter(|(has_value, _)| *has_value)
        .enumerate()
        .map(|(i, (_, field))| {
            if i == 0 {
                format!(" SET {}", field)
            } else {
                format!(", {}", field)
            }
        })
        .collect::<String>();

    let query = format!("UPDATE permissions{} WHERE id = ?", field_clauses);

    // Start binding parameters
    let mut db_query = sqlx::query_as::<_, Permission>(&query);

    // Bind parameters
    if let Some(val) = name {
        db_query = db_query.bind(val);
    }
    if let Some(val) = description {
        db_query = db_query.bind(val);
    }
    if let Some(val) = resource_type {
        db_query = db_query.bind(val);
    }

    // Bind the ID parameter
    db_query = db_query.bind(id);

    // Execute the query in a transaction
    let mut tx = pool.begin().await?;
    let permission = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update permission")?;

    tx.commit().await?;
    Ok(permission)
}

/// Deletes a permission from the system.
///
/// This function permanently removes a permission record from the database.
/// It should be used with caution, as it affects role capabilities and
/// may impact system access control.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the permission to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the permission
/// * `Err(anyhow::Error)` - Failed to delete the permission
///
/// # Warning
///
/// This operation is irreversible. Before deleting a permission, consider:
/// - Roles with this permission will lose the associated capability
/// - Relationships in permissions_role table may need to be cleaned up
/// - System functionality may be affected if critical permissions are removed
///
/// # Important
///
/// Depending on the database schema, this operation may:
/// - Fail if foreign key constraints are enforced and the permission is in use
/// - Leave orphaned records if cascading deletes are not configured
/// - Affect user access to system features
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn delete_permission(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM permissions WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete permission")?;

    tx.commit().await?;
    Ok(())
}

//=============================================================================
// Role-Permission Operations
//=============================================================================

/// Assigns a permission to a role.
///
/// This function creates an association between a permission and a role,
/// granting the capability represented by the permission to users who have
/// the specified role. This is a core operation in the RBAC system for
/// building role capabilities.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `permission_id` - Unique identifier of the permission to assign
/// * `role_id` - Unique identifier of the role to receive the permission
///
/// # Returns
///
/// * `Ok(())` - Successfully assigned the permission to the role
/// * `Err(anyhow::Error)` - Failed to assign the permission
///
/// # Uniqueness
///
/// This function assumes that the combination of `permission_id` and `role_id`
/// must be unique in the permissions_role table. If this association already
/// exists, the operation will fail with a unique constraint violation.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Note
///
/// After this operation, all users who have the specified role will effectively
/// gain the assigned permission.
pub async fn assign_permission_to_role(
    pool: &Pool<MySql>,
    permission_id: i64,
    role_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("INSERT INTO permissions_role (permissions_id, role_id) VALUES (?, ?)")
        .bind(permission_id)
        .bind(role_id)
        .execute(&mut *tx)
        .await
        .context("Failed to assign permission to role")?;

    tx.commit().await?;
    Ok(())
}

/// Removes a permission from a role.
///
/// This function deletes the association between a permission and a role,
/// revoking the capability represented by the permission from users who have
/// the specified role. This is used to adjust role capabilities in the RBAC system.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `permission_id` - Unique identifier of the permission to remove
/// * `role_id` - Unique identifier of the role from which to remove the permission
///
/// # Returns
///
/// * `Ok(())` - Successfully removed the permission from the role
/// * `Err(anyhow::Error)` - Failed to remove the permission
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Note
///
/// After this operation, users who have the specified role will no longer have the
/// capability granted by this permission, unless they have another role that includes it.
pub async fn remove_permission_from_role(
    pool: &Pool<MySql>,
    permission_id: i64,
    role_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM permissions_role WHERE permissions_id = ? AND role_id = ?")
        .bind(permission_id)
        .bind(role_id)
        .execute(&mut *tx)
        .await
        .context("Failed to remove permission from role")?;

    tx.commit().await?;
    Ok(())
}

/// Retrieves all permissions associated with a specific role.
///
/// This function fetches all permissions that have been assigned to a given role.
/// It's useful for displaying role capabilities or checking the full set of
/// permissions granted by a particular role.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `role_id` - Unique identifier of the role whose permissions to retrieve
///
/// # Returns
///
/// * `Ok(Vec<Permission>)` - Successfully retrieved list of permissions for the role
/// * `Err(anyhow::Error)` - Failed to fetch role permissions
///
/// # Query Details
///
/// This function performs a JOIN operation between the permissions and permissions_role
/// tables to find all permissions associated with the specified role.
pub async fn get_role_permissions(
    pool: &Pool<MySql>,
    role_id: i64,
) -> anyhow::Result<Vec<Permission>> {
    let permissions = sqlx::query_as::<_, Permission>(
        r#"SELECT p.* FROM permissions p
        JOIN permissions_role pr ON p.id = pr.permissions_id
        WHERE pr.role_id = ?
        ORDER BY p.created_at DESC"#,
    )
    .bind(role_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch role permissions")?;

    Ok(permissions)
}

//=============================================================================
// User-Role Operations
//=============================================================================

/// Assigns a role to a user.
///
/// This function creates an association between a user and a role,
/// granting the user all permissions associated with that role.
/// This is a core operation in the RBAC system for controlling user access.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - Unique identifier of the user to receive the role
/// * `role_id` - Unique identifier of the role to assign
///
/// # Returns
///
/// * `Ok(())` - Successfully assigned the role to the user
/// * `Err(anyhow::Error)` - Failed to assign the role
///
/// # Uniqueness
///
/// This function assumes that the combination of `user_id` and `role_id`
/// must be unique in the role_user table. If this association already exists,
/// the operation will fail with a unique constraint violation.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Note
///
/// After this operation, the user will have all permissions associated with
/// the assigned role, as determined by the permissions_role table.
pub async fn assign_role_to_user(
    pool: &Pool<MySql>,
    user_id: i64,
    role_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("INSERT INTO role_user (user_id, role_id) VALUES (?, ?)")
        .bind(user_id)
        .bind(role_id)
        .execute(&mut *tx)
        .await
        .context("Failed to assign role to user")?;

    tx.commit().await?;
    Ok(())
}

/// Removes a role from a user.
///
/// This function deletes the association between a user and a role,
/// revoking all permissions granted by that role from the user.
/// This is used to adjust user access in the RBAC system.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - Unique identifier of the user from whom to remove the role
/// * `role_id` - Unique identifier of the role to remove
///
/// # Returns
///
/// * `Ok(())` - Successfully removed the role from the user
/// * `Err(anyhow::Error)` - Failed to remove the role
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
///
/// # Note
///
/// After this operation, the user will no longer have the permissions granted
/// by this role, unless they have other roles that include the same permissions.
pub async fn remove_role_from_user(
    pool: &Pool<MySql>,
    user_id: i64,
    role_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM role_user WHERE user_id = ? AND role_id = ?")
        .bind(user_id)
        .bind(role_id)
        .execute(&mut *tx)
        .await
        .context("Failed to remove role from user")?;

    tx.commit().await?;
    Ok(())
}

/// Retrieves all roles assigned to a specific user.
///
/// This function fetches all roles that have been assigned to a given user.
/// It's useful for displaying user roles or checking role-based access control.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - Unique identifier of the user whose roles to retrieve
///
/// # Returns
///
/// * `Ok(Vec<Role>)` - Successfully retrieved list of roles for the user
/// * `Err(anyhow::Error)` - Failed to fetch user roles
///
/// # Query Details
///
/// This function performs a JOIN operation between the roles and role_user
/// tables to find all roles associated with the specified user.
pub async fn get_user_roles(pool: &Pool<MySql>, user_id: i64) -> anyhow::Result<Vec<Role>> {
    let roles = sqlx::query_as::<_, Role>(
        r#"SELECT r.* FROM roles r
        JOIN role_user ru ON r.id = ru.role_id
        WHERE ru.user_id = ?
        ORDER BY r.created_at DESC"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch user roles")?;

    Ok(roles)
}

/// Retrieves all permissions effectively granted to a specific user.
///
/// This function computes the complete set of permissions a user has based on
/// all their assigned roles. It eliminates duplicate permissions when a user
/// has multiple roles that grant the same permission.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - Unique identifier of the user whose permissions to retrieve
///
/// # Returns
///
/// * `Ok(Vec<Permission>)` - Successfully retrieved list of user permissions
/// * `Err(anyhow::Error)` - Failed to fetch user permissions
///
/// # Query Details
///
/// This function performs multiple JOIN operations across the permissions,
/// permissions_role, and role_user tables to calculate the effective permissions
/// for a user based on all their assigned roles.
///
/// # Performance Considerations
///
/// This query can be relatively expensive in systems with many roles and permissions.
/// Consider caching the results when appropriate, especially for frequently accessed users.
pub async fn get_user_permissions(
    pool: &Pool<MySql>,
    user_id: i64,
) -> anyhow::Result<Vec<Permission>> {
    let permissions = sqlx::query_as::<_, Permission>(
        r#"SELECT DISTINCT p.* FROM permissions p
        JOIN permissions_role pr ON p.id = pr.permissions_id
        JOIN role_user ru ON pr.role_id = ru.role_id
        WHERE ru.user_id = ?
        ORDER BY p.created_at DESC"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch user permissions")?;

    Ok(permissions)
}