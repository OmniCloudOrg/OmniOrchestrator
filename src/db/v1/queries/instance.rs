use super::super::tables::Instance;
use anyhow::Context;
use sqlx::{MySql, Pool};

pub async fn list_instances(pool: &Pool<MySql>, app_id: i64) -> anyhow::Result<Vec<Instance>> {
    let instances = sqlx::query_as::<_, Instance>(
        "SELECT * FROM instances WHERE app_id = ? ORDER BY created_at DESC",
    )
    .bind(app_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch instances")?;

    Ok(instances)
}

pub async fn get_instance_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Instance> {
    let instance = sqlx::query_as::<_, Instance>("SELECT * FROM instances WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch instance")?;

    Ok(instance)
}

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
