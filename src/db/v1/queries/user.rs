use super::super::tables::User;
use anyhow::Context;
use sqlx::{MySql, Pool};

pub async fn list_users(pool: &Pool<MySql>) -> anyhow::Result<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .context("Failed to fetch users")?;

    Ok(users)
}

pub async fn get_user_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch user")?;

    Ok(user)
}

pub async fn get_user_by_email(pool: &Pool<MySql>, email: &str) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await
        .context("Failed to fetch user by email")?;

    Ok(user)
}

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
