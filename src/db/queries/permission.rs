use sqlx::{MySql, Pool};
use anyhow::Context;
use crate::models::{Role, Permission};

// Role Operations
pub async fn list_roles(pool: &Pool<MySql>) -> anyhow::Result<Vec<Role>> {
    let roles = sqlx::query_as::<_, Role>(
        "SELECT * FROM roles ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch roles")?;

    Ok(roles)
}

pub async fn get_role_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Role> {
    let role = sqlx::query_as::<_, Role>(
        "SELECT * FROM roles WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch role")?;

    Ok(role)
}

pub async fn create_role(
    pool: &Pool<MySql>,
    name: &str,
    description: Option<&str>,
) -> anyhow::Result<Role> {
    let mut tx = pool.begin().await?;

    let role = sqlx::query_as::<_, Role>(
        "INSERT INTO roles (name, description) VALUES (?, ?)"
    )
    .bind(name)
    .bind(description)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create role")?;

    tx.commit().await?;
    Ok(role)
}

pub async fn update_role(
    pool: &Pool<MySql>,
    id: i64,
    name: Option<&str>,
    description: Option<&str>,
) -> anyhow::Result<Role> {
    let mut tx = pool.begin().await?;

    let mut query = String::from("UPDATE roles SET id = id");
    
    if let Some(name) = name {
        query.push_str(", name = ?");
    }
    if let Some(description) = description {
        query.push_str(", description = ?");
    }
    
    query.push_str(" WHERE id = ?");

    let mut db_query = sqlx::query_as::<_, Role>(&query);
    
    if let Some(name) = name {
        db_query = db_query.bind(name);
    }
    if let Some(description) = description {
        db_query = db_query.bind(description);
    }
    
    db_query = db_query.bind(id);

    let role = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update role")?;

    tx.commit().await?;
    Ok(role)
}

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

// Permission Operations
pub async fn list_permissions(pool: &Pool<MySql>) -> anyhow::Result<Vec<Permission>> {
    let permissions = sqlx::query_as::<_, Permission>(
        "SELECT * FROM permissions ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch permissions")?;

    Ok(permissions)
}

pub async fn get_permission_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Permission> {
    let permission = sqlx::query_as::<_, Permission>(
        "SELECT * FROM permissions WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to fetch permission")?;

    Ok(permission)
}

pub async fn create_permission(
    pool: &Pool<MySql>,
    name: &str,
    description: Option<&str>,
    resource_type: Option<&str>,
) -> anyhow::Result<Permission> {
    let mut tx = pool.begin().await?;

    let permission = sqlx::query_as::<_, Permission>(
        "INSERT INTO permissions (name, description, resource_type) VALUES (?, ?, ?)"
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

pub async fn update_permission(
    pool: &Pool<MySql>,
    id: i64,
    name: Option<&str>,
    description: Option<&str>,
    resource_type: Option<&str>,
) -> anyhow::Result<Permission> {
    let mut tx = pool.begin().await?;

    let mut query = String::from("UPDATE permissions SET id = id");
    
    if let Some(name) = name {
        query.push_str(", name = ?");
    }
    if let Some(description) = description {
        query.push_str(", description = ?");
    }
    if let Some(resource_type) = resource_type {
        query.push_str(", resource_type = ?");
    }
    
    query.push_str(" WHERE id = ?");

    let mut db_query = sqlx::query_as::<_, Permission>(&query);
    
    if let Some(name) = name {
        db_query = db_query.bind(name);
    }
    if let Some(description) = description {
        db_query = db_query.bind(description);
    }
    if let Some(resource_type) = resource_type {
        db_query = db_query.bind(resource_type);
    }
    
    db_query = db_query.bind(id);

    let permission = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update permission")?;

    tx.commit().await?;
    Ok(permission)
}

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

// Role-Permission Operations
pub async fn assign_permission_to_role(
    pool: &Pool<MySql>,
    permission_id: i64,
    role_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO permissions_role (permissions_id, role_id) VALUES (?, ?)"
    )
    .bind(permission_id)
    .bind(role_id)
    .execute(&mut *tx)
    .await
    .context("Failed to assign permission to role")?;

    tx.commit().await?;
    Ok(())
}

pub async fn remove_permission_from_role(
    pool: &Pool<MySql>,
    permission_id: i64,
    role_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "DELETE FROM permissions_role WHERE permissions_id = ? AND role_id = ?"
    )
    .bind(permission_id)
    .bind(role_id)
    .execute(&mut *tx)
    .await
    .context("Failed to remove permission from role")?;

    tx.commit().await?;
    Ok(())
}

pub async fn get_role_permissions(
    pool: &Pool<MySql>,
    role_id: i64,
) -> anyhow::Result<Vec<Permission>> {
    let permissions = sqlx::query_as::<_, Permission>(
        r#"SELECT p.* FROM permissions p
        JOIN permissions_role pr ON p.id = pr.permissions_id
        WHERE pr.role_id = ?
        ORDER BY p.created_at DESC"#
    )
    .bind(role_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch role permissions")?;

    Ok(permissions)
}

// User-Role Operations
pub async fn assign_role_to_user(
    pool: &Pool<MySql>,
    user_id: i64,
    role_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO role_user (user_id, role_id) VALUES (?, ?)"
    )
    .bind(user_id)
    .bind(role_id)
    .execute(&mut *tx)
    .await
    .context("Failed to assign role to user")?;

    tx.commit().await?;
    Ok(())
}

pub async fn remove_role_from_user(
    pool: &Pool<MySql>,
    user_id: i64,
    role_id: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "DELETE FROM role_user WHERE user_id = ? AND role_id = ?"
    )
    .bind(user_id)
    .bind(role_id)
    .execute(&mut *tx)
    .await
    .context("Failed to remove role from user")?;

    tx.commit().await?;
    Ok(())
}

pub async fn get_user_roles(
    pool: &Pool<MySql>,
    user_id: i64,
) -> anyhow::Result<Vec<Role>> {
    let roles = sqlx::query_as::<_, Role>(
        r#"SELECT r.* FROM roles r
        JOIN role_user ru ON r.id = ru.role_id
        WHERE ru.user_id = ?
        ORDER BY r.created_at DESC"#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch user roles")?;

    Ok(roles)
}

pub async fn get_user_permissions(
    pool: &Pool<MySql>,
    user_id: i64,
) -> anyhow::Result<Vec<Permission>> {
    let permissions = sqlx::query_as::<_, Permission>(
        r#"SELECT DISTINCT p.* FROM permissions p
        JOIN permissions_role pr ON p.id = pr.permissions_id
        JOIN role_user ru ON pr.role_id = ru.role_id
        WHERE ru.user_id = ?
        ORDER BY p.created_at DESC"#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch user permissions")?;

    Ok(permissions)
}