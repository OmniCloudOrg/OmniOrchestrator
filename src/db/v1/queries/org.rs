use super::super::tables::Org;
use anyhow::Context;
use sqlx::{MySql, Pool};

pub async fn list_orgs(pool: &Pool<MySql>) -> anyhow::Result<Vec<Org>> {
    let orgs = sqlx::query_as::<_, Org>("SELECT * FROM orgs ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .context("Failed to fetch organizations")?;

    Ok(orgs)
}

pub async fn get_org_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Org> {
    let org = sqlx::query_as::<_, Org>("SELECT * FROM orgs WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch organization")?;

    Ok(org)
}

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
