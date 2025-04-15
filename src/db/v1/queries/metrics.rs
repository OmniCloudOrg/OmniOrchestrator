use sqlx::{FromRow, MySql, Pool};
use crate::db::tables::Metric;

pub async fn get_metrics_by_app_id(pool: &Pool<MySql>, instance_id: Option<i64>) -> anyhow::Result<Vec<Metric>> {
    let query = if let Some(app_id) = instance_id {
        sqlx::query_as::<_, Metric>(
            r#"SELECT id, instance_id, metric_name, metric_value, labels, timestamp FROM metrics WHERE instance_id = ? "#
        )
        .bind(app_id)
    } else {
        sqlx::query_as::<_, Metric>(
            r#"SELECT id, instance_id, metric_name, metric_value, labels, timestamp FROM metrics WHERE instance_id IS NULL "#
        )
    };

    query.fetch_all(pool).await.map_err(Into::into)
}
