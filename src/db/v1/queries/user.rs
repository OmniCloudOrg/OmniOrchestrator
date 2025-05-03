use super::super::tables::User;
use crate::db::v1::tables::{UserMeta, UserPii, UserSession};
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
pub async fn list_users(
    pool: &Pool<MySql>,
    page: i64,
    per_page: i64,
) -> anyhow::Result<Vec<User>> {
    let offset = (page * per_page) as i64;
    let limit = per_page as i64;

    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE deleted_at IS NULL ORDER BY created_at DESC LIMIT ? OFFSET ?"
    )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .context("Failed to fetch users")?;

    Ok(users)
}

/// Counts the total number of users in the system.
pub async fn count_users(pool: &Pool<MySql>) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE deleted_at IS NULL")
        .fetch_one(pool)
        .await
        .context("Failed to count users")?;

    Ok(count)
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
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ? AND deleted_at IS NULL")
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
pub async fn get_user_by_email(pool: &Pool<MySql>, email: &str) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ? AND deleted_at IS NULL")
        .bind(email)
        .fetch_one(pool)
        .await
        .context("Failed to fetch user by email")?;

    Ok(user)
}

/// Creates a new user in the system.
///
/// This function inserts a new user record with the provided information.
/// New users are created with the 'pending' status by default.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `email` - User's email address (must be unique)
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
    password: &str, // Should be pre-hashed
    salt: &str,
) -> anyhow::Result<User> {
    let mut tx = pool.begin().await?;

    // Insert into the users table first
    let query = r#"INSERT INTO users (
        email, password, salt, email_verified, active, status, 
        two_factor_enabled, two_factor_verified, login_attempts
    ) VALUES (?, ?, ?, 0, 1, 'pending', 0, 0, 0)"#;

    let user_id = sqlx::query(query)
        .bind(email)
        .bind(password)
        .bind(salt)
        .execute(&mut *tx)
        .await
        .context("Failed to create user")?
        .last_insert_id();

    println!("User created with ID: {}", user_id);

    // Get the newly created user
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to fetch created user")?;

    // Create default user_meta record
    sqlx::query(r#"INSERT INTO user_meta (user_id, timezone, language, theme, onboarding_completed) 
                  VALUES (?, 'UTC', 'en', 'light', 0)"#)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .context("Failed to create user metadata")?;

    // Create empty user_pii record
    sqlx::query(r#"INSERT INTO user_pii (user_id, identity_verified) 
                  VALUES (?, 0)"#)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .context("Failed to create user PII record")?;

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
/// * `email` - Optional new email address for the user
/// * `active` - Optional new active status for the user
/// * `status` - Optional new status (active, deactivated, suspended, pending)
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
    email: Option<&str>,
    active: Option<bool>,
    status: Option<&str>,
) -> anyhow::Result<User> {
    let mut tx = pool.begin().await?;

    let mut query = String::from("UPDATE users SET updated_at = CURRENT_TIMESTAMP");

    if let Some(email) = email {
        query.push_str(", email = ?");
    }
    if let Some(active) = active {
        query.push_str(", active = ?");
    }
    if let Some(status) = status {
        query.push_str(", status = ?");
    }

    query.push_str(" WHERE id = ?");

    let mut db_query = sqlx::query(&query);

    if let Some(email) = email {
        db_query = db_query.bind(email);
    }
    if let Some(active) = active {
        db_query = db_query.bind(active);
    }
    if let Some(status) = status {
        db_query = db_query.bind(status);
    }

    db_query = db_query.bind(id);

    db_query
        .execute(&mut *tx)
        .await
        .context("Failed to update user")?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to fetch updated user")?;

    tx.commit().await?;
    Ok(user)
}

/// Updates a user's personal information.
///
/// This function updates the user's personal information in the user_pii table.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - Unique identifier of the user
/// * `first_name` - Optional new first name
/// * `last_name` - Optional new last name
/// * `full_name` - Optional new full name (can be generated if first and last are provided)
///
/// # Returns
///
/// * `Ok(())` - Successfully updated user PII
/// * `Err(anyhow::Error)` - Failed to update user PII
pub async fn update_user_pii(
    pool: &Pool<MySql>,
    user_id: i64,
    first_name: Option<&str>,
    last_name: Option<&str>,
    full_name: Option<&str>,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    let mut query = String::from("UPDATE user_pii SET updated_at = CURRENT_TIMESTAMP");

    if let Some(first_name) = first_name {
        query.push_str(", first_name = ?");
    }
    if let Some(last_name) = last_name {
        query.push_str(", last_name = ?");
    }
    if let Some(full_name) = full_name {
        query.push_str(", full_name = ?");
    } else if first_name.is_some() && last_name.is_some() {
        query.push_str(", full_name = CONCAT(first_name, ' ', last_name)");
    }

    query.push_str(" WHERE user_id = ?");

    let mut db_query = sqlx::query(&query);

    if let Some(first_name) = first_name {
        db_query = db_query.bind(first_name);
    }
    if let Some(last_name) = last_name {
        db_query = db_query.bind(last_name);
    }
    if let Some(full_name) = full_name {
        db_query = db_query.bind(full_name);
    }

    db_query = db_query.bind(user_id);

    db_query
        .execute(&mut *tx)
        .await
        .context("Failed to update user PII")?;

    tx.commit().await?;
    Ok(())
}

/// Updates a user's preferences and metadata.
///
/// This function updates the user's preferences in the user_meta table.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - Unique identifier of the user
/// * `timezone` - Optional new timezone preference
/// * `language` - Optional new language preference
/// * `theme` - Optional new theme preference
/// * `onboarding_completed` - Optional flag indicating if onboarding is completed
///
/// # Returns
///
/// * `Ok(())` - Successfully updated user metadata
/// * `Err(anyhow::Error)` - Failed to update user metadata
pub async fn update_user_meta(
    pool: &Pool<MySql>,
    user_id: i64,
    timezone: Option<&str>,
    language: Option<&str>,
    theme: Option<&str>,
    onboarding_completed: Option<bool>,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    let mut query = String::from("UPDATE user_meta SET updated_at = CURRENT_TIMESTAMP");

    if let Some(timezone) = timezone {
        query.push_str(", timezone = ?");
    }
    if let Some(language) = language {
        query.push_str(", language = ?");
    }
    if let Some(theme) = theme {
        query.push_str(", theme = ?");
    }
    if let Some(onboarding_completed) = onboarding_completed {
        query.push_str(", onboarding_completed = ?");
    }

    query.push_str(" WHERE user_id = ?");

    let mut db_query = sqlx::query(&query);

    if let Some(timezone) = timezone {
        db_query = db_query.bind(timezone);
    }
    if let Some(language) = language {
        db_query = db_query.bind(language);
    }
    if let Some(theme) = theme {
        db_query = db_query.bind(theme);
    }
    if let Some(onboarding_completed) = onboarding_completed {
        db_query = db_query.bind(onboarding_completed);
    }

    db_query = db_query.bind(user_id);

    db_query
        .execute(&mut *tx)
        .await
        .context("Failed to update user metadata")?;

    tx.commit().await?;
    Ok(())
}

/// Updates a user's security settings.
///
/// This function updates security-related fields in the users table.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the user
/// * `password` - Optional new password (should be pre-hashed)
/// * `salt` - Optional new salt (should be provided if password is)
/// * `two_factor_enabled` - Optional flag to enable/disable two-factor authentication
/// * `two_factor_verified` - Optional flag indicating 2FA verification status
///
/// # Returns
///
/// * `Ok(User)` - Successfully updated user security settings
/// * `Err(anyhow::Error)` - Failed to update user security settings
pub async fn update_user_security(
    pool: &Pool<MySql>,
    id: i64,
    password: Option<&str>, 
    salt: Option<&str>,
    two_factor_enabled: Option<bool>,
    two_factor_verified: Option<bool>,
) -> anyhow::Result<User> {
    let mut tx = pool.begin().await?;

    let mut query = String::from("UPDATE users SET updated_at = CURRENT_TIMESTAMP");

    if let Some(password) = password {
        query.push_str(", password = ?, password_changed_at = CURRENT_TIMESTAMP");
    }
    if let Some(salt) = salt {
        query.push_str(", salt = ?");
    }
    if let Some(two_factor_enabled) = two_factor_enabled {
        query.push_str(", two_factor_enabled = ?");
    }
    if let Some(two_factor_verified) = two_factor_verified {
        query.push_str(", two_factor_verified = ?");
    }

    query.push_str(" WHERE id = ?");

    let mut db_query = sqlx::query(&query);

    if let Some(password) = password {
        db_query = db_query.bind(password);
    }
    if let Some(salt) = salt {
        db_query = db_query.bind(salt);
    }
    if let Some(two_factor_enabled) = two_factor_enabled {
        db_query = db_query.bind(two_factor_enabled);
    }
    if let Some(two_factor_verified) = two_factor_verified {
        db_query = db_query.bind(two_factor_verified);
    }

    db_query = db_query.bind(id);

    db_query
        .execute(&mut *tx)
        .await
        .context("Failed to update user security settings")?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to fetch updated user")?;

    tx.commit().await?;
    Ok(user)
}

/// Soft deletes a user from the system.
///
/// This function marks a user as deleted by setting the deleted_at timestamp,
/// but does not actually remove the record from the database.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - Unique identifier of the user to delete
///
/// # Returns
///
/// * `Ok(())` - Successfully marked the user as deleted
/// * `Err(anyhow::Error)` - Failed to mark the user as deleted
///
/// # Notes
///
/// This operation is reversible by clearing the deleted_at field. The function
/// preserves user data while making it inactive in the system.
pub async fn soft_delete_user(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("UPDATE users SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to soft-delete user")?;

    tx.commit().await?;
    Ok(())
}

/// Hard deletes a user from the system.
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
/// their 'active' flag to false or using soft_delete_user rather than deleting them completely.
///
/// # Note
///
/// Due to CASCADE DELETE constraints in the database, this will automatically delete
/// all related records in user_meta, user_pii, user_sessions, and other tables with
/// foreign key relationships to the user.
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

/// Records a user login attempt.
///
/// This function updates login-related fields like last_login_at and login_attempts.
/// It's used during authentication to track login activity and manage security features
/// like account lockouts after too many failed attempts.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `id` - User ID
/// * `successful` - Whether the login attempt was successful
///
/// # Returns
///
/// * `Ok(User)` - Updated user record
/// * `Err(anyhow::Error)` - Failed to update login information
pub async fn record_login_attempt(
    pool: &Pool<MySql>, 
    id: i64, 
    successful: bool
) -> anyhow::Result<User> {
    let mut tx = pool.begin().await?;

    if successful {
        // Reset login attempts and update last login time
        sqlx::query(
            "UPDATE users SET last_login_at = CURRENT_TIMESTAMP, login_attempts = 0, 
             locked_until = NULL WHERE id = ?"
        )
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to record successful login")?;
    } else {
        // Increment login attempts and possibly lock the account
        sqlx::query(
            "UPDATE users SET login_attempts = login_attempts + 1,
             locked_until = CASE 
                WHEN login_attempts >= 5 THEN DATE_ADD(CURRENT_TIMESTAMP, INTERVAL 30 MINUTE)
                ELSE locked_until
             END
             WHERE id = ?"
        )
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to record failed login")?;
    }

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to fetch updated user")?;

    tx.commit().await?;
    Ok(user)
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
///
/// # Account Lockout
///
/// This function checks if the account is locked before attempting authentication.
pub async fn login_user(
    pool: &Pool<MySql>,
    email: &str,
    password: &str, // Should be pre-hashed
) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users 
         WHERE email = ? 
         AND password = ? 
         AND deleted_at IS NULL 
         AND active = 1
         AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)"
    )
        .bind(email)
        .bind(password)
        .fetch_one(pool)
        .await
        .context("Failed to login user")?;

    Ok(user)
}

/// Creates a new user session.
///
/// This function creates a new session for a user after successful authentication.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - ID of the authenticated user
/// * `session_token` - Generated session token
/// * `refresh_token` - Optional refresh token
/// * `ip_address` - Client IP address
/// * `user_agent` - Client user agent string
/// * `expires_at` - Session expiration time
///
/// # Returns
///
/// * `Ok(i64)` - ID of the created session
/// * `Err(anyhow::Error)` - Failed to create session
pub async fn create_session(
    pool: &Pool<MySql>,
    user_id: i64,
    session_token: &str,
    refresh_token: Option<&str>,
    ip_address: &str,
    user_agent: &str,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> anyhow::Result<i64> {
    let result = sqlx::query(
        "INSERT INTO user_sessions (
            user_id, session_token, refresh_token, ip_address, user_agent,
            is_active, last_activity, expires_at
        ) VALUES (?, ?, ?, ?, ?, 1, CURRENT_TIMESTAMP, ?)"
    )
        .bind(user_id)
        .bind(session_token)
        .bind(refresh_token)
        .bind(ip_address)
        .bind(user_agent)
        .bind(expires_at.naive_utc())
        .execute(pool)
        .await
        .context("Failed to create session")?;

    Ok(result.last_insert_id() as i64)
}

/// Invalidates a user session.
///
/// This function marks a session as inactive, effectively logging the user out.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `session_token` - The session token to invalidate
///
/// # Returns
///
/// * `Ok(())` - Successfully invalidated the session
/// * `Err(anyhow::Error)` - Failed to invalidate the session
pub async fn invalidate_session(
    pool: &Pool<MySql>,
    session_token: &str,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE user_sessions SET is_active = 0 WHERE session_token = ?")
        .bind(session_token)
        .execute(pool)
        .await
        .context("Failed to invalidate session")?;

    Ok(())
}

/// Invalidates all sessions for a user.
///
/// This function marks all of a user's sessions as inactive, effectively logging them out
/// of all devices.
///
/// # Arguments
///
/// * `pool` - Database connection pool for executing the query
/// * `user_id` - The ID of the user whose sessions should be invalidated
///
/// # Returns
///
/// * `Ok(())` - Successfully invalidated all sessions
/// * `Err(anyhow::Error)` - Failed to invalidate sessions
pub async fn invalidate_all_user_sessions(
    pool: &Pool<MySql>,
    user_id: i64,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE user_sessions SET is_active = 0 WHERE user_id = ?")
        .bind(user_id)
        .execute(pool)
        .await
        .context("Failed to invalidate user sessions")?;

    Ok(())
}

/// Retrieves a list of all active sessions for a user.
pub async fn get_user_sessions(
    pool: &Pool<MySql>,
    user_id: i64,
) -> anyhow::Result<Vec<UserSession>> {
    let sessions = sqlx::query_as::<_, UserSession>(
        "SELECT * FROM user_sessions WHERE user_id = ? AND is_active = 1"
    )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .context("Failed to fetch user sessions")?;

    Ok(sessions)
}

/// Is session valid?
pub async fn is_session_valid(
    pool: &Pool<MySql>,
    session_token: &str,
) -> anyhow::Result<bool> {
    let session = sqlx::query_as::<_, UserSession>(
        "SELECT * FROM user_sessions WHERE session_token = ? AND is_active = 1"
    )
        .bind(session_token)
        .fetch_optional(pool)
        .await
        .context("Failed to check session validity")?;

    Ok(session.is_some())
}

/// Get user meta information
pub async fn get_user_meta(pool: &Pool<MySql>, user_id: i64) -> Result<UserMeta, anyhow::Error> {
    // First try to find existing meta
    let meta = sqlx::query_as::<_, UserMeta>(
        r#"
        SELECT 
            id, user_id, timezone, language, theme, 
            notification_preferences, profile_image, dashboard_layout, 
            onboarding_completed, created_at, updated_at
        FROM user_meta
        WHERE user_id = ?
        "#
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    // If meta exists, return it
    if let Some(meta) = meta {
        return Ok(meta);
    }

    // Otherwise create default meta for the user
    let default_meta = sqlx::query_as::<_, UserMeta>(
        r#"
        INSERT INTO user_meta (
            user_id, timezone, language, theme, 
            onboarding_completed
        ) VALUES (?, 'UTC', 'en', 'light', 0)
        RETURNING id, user_id, timezone, language, theme, 
                  notification_preferences, profile_image, dashboard_layout, 
                  onboarding_completed, created_at, updated_at
        "#
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(default_meta)
}

/// Get user PII (Personally Identifiable Information)
pub async fn get_user_pii(pool: &Pool<MySql>, user_id: i64) -> Result<UserPii, anyhow::Error> {
    // Try to find existing PII
    let pii = sqlx::query_as::<_, UserPii>(
        r#"
        SELECT 
            id, user_id, first_name, last_name, full_name, 
            identity_verified, identity_verification_date, 
            identity_verification_method, created_at, updated_at
        FROM user_pii
        WHERE user_id = ?
        "#
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    // If PII exists, return it
    if let Some(pii) = pii {
        return Ok(pii);
    }

    // Otherwise create empty PII record for the user
    let default_pii = sqlx::query_as::<_, UserPii>(
        r#"
        INSERT INTO user_pii (
            user_id, identity_verified
        ) VALUES (?, 0)
        RETURNING id, user_id, first_name, last_name, full_name, 
                 identity_verified, identity_verification_date, 
                 identity_verification_method, created_at, updated_at
        "#
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(default_pii)
}
