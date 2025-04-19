use sqlx::{FromRow, MySql, Pool};
use crate::db::tables::Metric;

pub async fn get_metrics_by_app_id(pool: &Pool<MySql>, app_id: Option<i64>) -> anyhow::Result<Vec<Metric>> {
    let query = if let Some(app_id) = app_id {
        sqlx::query_as::<_, Metric>(
            r#"SELECT id, app_id, metric_name, metric_value, labels, timestamp FROM metrics WHERE app_id = ? "#
        )
        .bind(app_id)
    } else {
        sqlx::query_as::<_, Metric>(
            r#"SELECT id, app_id, metric_name, metric_value, labels, timestamp FROM metrics WHERE app_id IS NULL "#
        )
    };

    query.fetch_all(pool).await.map_err(Into::into)
}
