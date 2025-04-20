use super::super::tables::{Alert};
use anyhow::Context;
use sqlx::{FromRow, MySql, Pool};

/// Retrieves alerts by application ID, optionally filtering by organization ID and region ID.
/// 
/// # Arguments
/// * `pool` - A reference to the database connection pool.
/// * `app_id` - An optional application ID to filter alerts.
/// 
/// # Returns
/// * A vector of `AlertWithApp` containing the alerts associated with the specified application ID.
pub async fn get_alerts_by_app_id(pool: &Pool<MySql>, app_id: Option<i64>) -> anyhow::Result<Vec<Alert>> {
    let query = if let Some(app_id) = app_id {
        sqlx::query_as::<_, Alert>(
            r#"SELECT id, app_id, alert_name, alert_value, labels, timestamp FROM alerts WHERE app_id = ? "#
        )
        .bind(app_id)
    } else {
        sqlx::query_as::<_, Alert>(
            r#"SELECT id, app_id, alert_name, alert_value, labels, timestamp FROM alerts WHERE app_id IS NULL "#
        )
    };

    query.fetch_all(pool).await.map_err(Into::into)
}

/// Lists all alerts in the system with pagination support.
///
/// # Arguments
/// * `pool` - A reference to the database connection pool.
/// * `page` - The page number for pagination.
/// * `per_page` - The number of alerts per page.
/// 
/// # Returns
/// * A vector of `Alert` containing the alerts for the specified page.
pub async fn list_alerts(pool: &Pool<MySql>, page: i64, per_page: i64) -> anyhow::Result<Vec<Alert>> {
    let offset = page * per_page;
    let query = sqlx::query_as::<_, Alert>(
        r#"SELECT * FROM alerts LIMIT ? OFFSET ?"#
    )
    .bind(per_page)
    .bind(offset);

    query.fetch_all(pool).await.map_err(Into::into)
}