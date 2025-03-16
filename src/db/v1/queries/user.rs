use super::super::tables::User;
use anyhow::Context;
use sqlx::{MySql, Pool};

/// Retrieves all users in the system, ordered by creation time.
///
/// This function fetches all user records from the database, with
/// the most recently created users first. It provides a complete view
/// of all users registered in the system.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
///
/// # Returns
///
/// * `Ok(Vec<User>)` - Successfully retrieved list of users
/// * `Err(anyhow::Error)` - Failed to fetch users
///
/// # Use Cases
///
/// Common use cases include:
/// - Administrative dashboards showing all system users
/// - User management interfaces
/// - User activity reports and analytics
pub async fn list_users(pool: &Pool<MySql>) -> anyhow::Result<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .context("Failed to fetch users")?;

    Ok(users)
}

/// Retrieves a specific user by their unique identifier.
///
/// This function fetches detailed information about a single user record.
/// It's typically used when specific user details are needed, such as
/// for displaying user profiles or performing user-specific operations.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the user to retrieve
///
/// # Returns
///
/// * `Ok(User)` - Successfully retrieved user information
/// * `Err(anyhow::Error)` - Failed to fetch user (including if not found)
///
/// # Error Handling
///
/// Returns an error if no user with the given ID exists or if a database
/// error occurs during the query execution.
pub async fn get_user_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch user")?;

    Ok(user)
}

/// Retrieves a user by their email address.
///
/// This function fetches a user record based on their email address, which
/// serves as a unique identifier in many authentication and user management
/// scenarios. It's commonly used during login processes and for email verification.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `email` - Email address of the user to retrieve
///
/// # Returns
///
/// * `Ok(User)` - Successfully retrieved user information
/// * `Err(anyhow::Error)` - Failed to fetch user (including if not found)
///
/// # Error Handling
///
/// Returns an error if no user with the given email exists or if a database
/// error occurs during the query execution.
///
/// # Note
///
/// This function assumes email addresses are unique in the system. If your
/// database schema does not enforce this constraint, it's possible to retrieve
/// multiple users with the same email, in which case this function would return
/// the first match.
pub async fn get_user_by_email(pool: &Pool<MySql>, email: &str) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await
        .context("Failed to fetch user by email")?;

    Ok(user)
}

/// Creates a new user in the system.
///
/// This function inserts a new user record with the provided information.
/// New users are created with the 'active' flag set to true by default.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `email` - User's email address (must be unique)
/// * `name` - User's display name
/// * `password` - User's password (should be pre-hashed for security)
/// * `salt` - Cryptographic salt used in the password hashing process
///
/// # Returns
///
/// * `Ok(User)` - Successfully created user record
/// * `Err(anyhow::Error)` - Failed to create user record
///
/// # Security Considerations
///
/// This function expects the password to be pre-hashed before being passed in.
/// It does not perform any password hashing itself, as this is typically handled
/// by a higher-level security service. Never pass plain text passwords to this function.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn create_user(
    pool: &Pool<MySql>,
    email: &str,
    name: &str,
    password: &str, // Should be pre-hashed
    salt: &str,
) -> anyhow::Result<User> {
    let mut tx = pool.begin().await?;

    let user = sqlx::query_as::<_, User>(
        r#"INSERT INTO users (
            email, name, password, salt, active
        ) VALUES (?, ?, ?, ?, true)"#,
    )
    .bind(email)
    .bind(name)
    .bind(password)
    .bind(salt)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create user")?;

    tx.commit().await?;
    Ok(user)
}

/// Updates an existing user's information.
///
/// This function modifies a user record with the provided information.
/// It supports partial updates, allowing you to update only the fields that need changing.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the user to update
/// * `name` - Optional new name for the user
/// * `email` - Optional new email address for the user
/// * `active` - Optional new active status for the user
///
/// # Returns
///
/// * `Ok(User)` - Successfully updated user record
/// * `Err(anyhow::Error)` - Failed to update user
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
///
/// # Note
///
/// This function does not support updating passwords. Password updates should be
/// handled by a dedicated function with appropriate security measures.
pub async fn update_user(
    pool: &Pool<MySql>,
    id: i64,
    name: Option<&str>,
    email: Option<&str>,
    active: Option<bool>,
) -> anyhow::Result<User> {
    let mut tx = pool.begin().await?;

    let mut query = String::from("UPDATE users SET updated_at = CURRENT_TIMESTAMP");

    if let Some(name) = name {
        query.push_str(", name = ?");
    }
    if let Some(email) = email {
        query.push_str(", email = ?");
    }
    if let Some(active) = active {
        query.push_str(", active = ?");
    }

    query.push_str(" WHERE id = ?");

    let mut db_query = sqlx::query_as::<_, User>(&query);

    if let Some(name) = name {
        db_query = db_query.bind(name);
    }
    if let Some(email) = email {
        db_query = db_query.bind(email);
    }
    if let Some(active) = active {
        db_query = db_query.bind(active);
    }

    db_query = db_query.bind(id);

    let user = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update user")?;

    tx.commit().await?;
    Ok(user)
}

/// Deletes a user from the system.
///
/// This function permanently removes a user record from the database.
/// It should be used with caution, as it typically has significant implications
/// for data integrity and user experience.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the user to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully deleted the user
/// * `Err(anyhow::Error)` - Failed to delete the user
///
/// # Warning
///
/// This operation is irreversible. Consider the implications before deleting
/// users, especially those with existing data or relationships in the system.
/// For many applications, it may be preferable to deactivate users by setting
/// their 'active' flag to false rather than deleting them completely.
///
/// # Important
///
/// This function only deletes the user record itself. It does not cascade
/// delete related records such as user roles, permissions, or content created
/// by the user. Consider implementing cascading delete logic or foreign key
/// constraints if needed.
///
/// # Transaction Handling
///
/// This function uses a database transaction to ensure atomicity of the operation.
/// If any part of the operation fails, the entire operation is rolled back.
pub async fn delete_user(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete user")?;

    tx.commit().await?;
    Ok(())
}

/// Authenticates a user using email and password.
///
/// This function attempts to retrieve a user record with the provided email
/// and hashed password combination. It's used during login processes to verify 
/// user credentials.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `email` - Email address entered by the user
/// * `password` - Password entered by the user (should be pre-hashed)
///
/// # Returns
///
/// * `Ok(User)` - Successfully authenticated user
/// * `Err(anyhow::Error)` - Authentication failed or user doesn't exist
///
/// # Security Considerations
///
/// This function expects the password to be pre-hashed before being passed in.
/// It does not perform any password hashing itself, as this is typically handled
/// by a higher-level security service that:
///
/// 1. Retrieves the user and their salt using `get_user_by_email`
/// 2. Uses the salt to hash the provided password
/// 3. Calls this function with the properly hashed password
///
/// # Error Handling
///
/// For security reasons, this function provides a generic error message
/// regardless of whether the email wasn't found or the password was incorrect.
/// This prevents information leakage about existing email addresses.
pub async fn login_user(
    pool: &Pool<MySql>,
    email: &str,
    password: &str, // Should be pre-hashed
) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ? AND password = ?")
        .bind(email)
        .bind(password)
        .fetch_one(pool)
        .await
        .context("Failed to login user")?;

    Ok(user)
}