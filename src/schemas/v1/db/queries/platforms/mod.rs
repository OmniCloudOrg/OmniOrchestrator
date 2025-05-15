use crate::models::platform::{Platform};
use crate::models::instance::Instance;
use anyhow::Context;
use serde::Serialize;
use sqlx::{MySql, Pool};

// Get all platforms
pub async fn get_all_platforms(pool: &Pool<MySql>) -> Result<Vec<Platform>, sqlx::Error> {
    let platforms = sqlx::query_as::<_, Platform>("SELECT * FROM platforms")
        .fetch_all(pool)
        .await?;
    Ok(platforms)
}

// Get platform by ID
pub async fn get_platform_by_id(pool: &Pool<MySql>, platform_id: i64) -> Result<Platform, sqlx::Error> {
    let platform = sqlx::query_as::<_, Platform>("SELECT * FROM platforms WHERE id = ?")
        .bind(platform_id)
        .fetch_one(pool)
        .await?;
    Ok(platform)
}

// Check if platform exists
pub async fn check_platform_exists(pool: &Pool<MySql>, platform_id: i64) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM platforms WHERE id = ?)")
        .bind(platform_id)
        .fetch_one(pool)
        .await?;
    Ok(exists)
}

// Get platform name by ID
pub async fn get_platform_name(pool: &Pool<MySql>, platform_id: i64) -> Result<String, sqlx::Error> {
    let name = sqlx::query_scalar::<_, String>("SELECT name FROM platforms WHERE id = ?")
        .bind(platform_id)
        .fetch_one(pool)
        .await?;
    Ok(name)
}

// Create a new platform
pub async fn create_platform(
    pool: &Pool<MySql>, 
    name: &str, 
    description: Option<&str>
) -> Result<Platform, sqlx::Error> {
    // First check if a platform with this name already exists
    let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM platforms WHERE name = ?)")
        .bind(name)
        .fetch_one(pool)
        .await?;
    
    if exists {
        return Err(sqlx::Error::RowNotFound); // Using this error type as a simple way to indicate the row already exists
    }
    
    sqlx::query(
        "INSERT INTO platforms (name, description) VALUES (?, ?)",
    )
    .bind(name)
    .bind(description)
    .execute(pool)
    .await?;
    
    // Retrieve the newly created platform
    let platform = sqlx::query_as::<_, Platform>("SELECT * FROM platforms WHERE name = ?")
        .bind(name)
        .fetch_one(pool)
        .await?;
    
    Ok(platform)
}

// Update platform
pub async fn update_platform(
    pool: &Pool<MySql>, 
    platform_id: i64, 
    name: Option<&str>, 
    description: Option<&str>
) -> Result<Platform, sqlx::Error> {
    // Build update query dynamically based on provided fields
    let mut query_parts = Vec::new();
    let mut query = String::from("UPDATE platforms SET ");
    
    if let Some(name_val) = name {
        query_parts.push("name = ?");
    }
    
    if let Some(desc_val) = description {
        query_parts.push("description = ?");
    }
    
    // If no fields to update, return the current platform
    if query_parts.is_empty() {
        return get_platform_by_id(pool, platform_id).await;
    }
    
    query.push_str(&query_parts.join(", "));
    query.push_str(" WHERE id = ?");
    
    let mut db_query = sqlx::query(&query);
    
    // Bind parameters in the order they appear in the query
    if let Some(name_val) = name {
        db_query = db_query.bind(name_val);
    }
    
    if let Some(desc_val) = description {
        db_query = db_query.bind(desc_val);
    }
    
    // Bind the WHERE clause parameter
    db_query = db_query.bind(platform_id);
    
    // Execute the update
    db_query.execute(pool).await?;
    
    // Return the updated platform
    get_platform_by_id(pool, platform_id).await
}

// Delete platform
pub async fn delete_platform(pool: &Pool<MySql>, platform_id: i64) -> Result<(), sqlx::Error> {
    // Check if platform exists first
    let exists = check_platform_exists(pool, platform_id).await?;
    
    if !exists {
        return Err(sqlx::Error::RowNotFound);
    }
    
    sqlx::query("DELETE FROM platforms WHERE id = ?")
        .bind(platform_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

// Get count of platforms
pub async fn count_platforms(pool: &Pool<MySql>) -> Result<i64, sqlx::Error> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM platforms")
        .fetch_one(pool)
        .await?;
    Ok(count)
}

// List platforms with pagination
pub async fn list_platforms(
    pool: &Pool<MySql>, 
    page: i64, 
    per_page: i64
) -> Result<Vec<Platform>, sqlx::Error> {
    let offset = (page - 1) * per_page;
    
    let platforms = sqlx::query_as::<_, Platform>(
        "SELECT * FROM platforms ORDER BY id LIMIT ? OFFSET ?"
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    
    Ok(platforms)
}

// Search platforms by name
pub async fn search_platforms_by_name(
    pool: &Pool<MySql>,
    name_pattern: &str
) -> Result<Vec<Platform>, sqlx::Error> {
    let search_pattern = format!("%{}%", name_pattern);
    
    let platforms = sqlx::query_as::<_, Platform>(
        "SELECT * FROM platforms WHERE name LIKE ? ORDER BY name"
    )
    .bind(search_pattern)
    .fetch_all(pool)
    .await?;
    
    Ok(platforms)
}