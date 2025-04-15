use crate::db::v1::queries as db;
use crate::db::v1::tables;
use rocket::{delete, get, post, serde::json::Json, State};
use sqlx::MySql;

#[get("/metrics/<instance_id>")]
pub async fn get_metrics_by_app_id(
    pool: &State<sqlx::Pool<MySql>>,
    instance_id: Option<i64>,
) -> Json<Vec<tables::Metric>> {
    let instance_id = instance_id.or(Some(0)); // Set to 0 (or null equivalent) if blank
    let metrics = db::metrics::get_metrics_by_app_id(pool, instance_id)
        .await
        .unwrap();

    Json(metrics)
}

#[get("/metrics")]
pub async fn get_metrics(pool: &State<sqlx::Pool<MySql>>) -> Json<Vec<tables::Metric>> {
    let metrics = db::metrics::get_metrics_by_app_id(pool, None)
        .await
        .unwrap();

    Json(metrics)
}