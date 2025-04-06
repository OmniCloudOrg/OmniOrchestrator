use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post, delete};
use sqlx::{MySql, Pool};
use crate::db::v1::tables::Backup;
use crate::db::v1::queries::{self as db};

/// List all backups
#[get("/apps/<app_id>/backups?<page>&<page_size>")]
pub async fn list_backups(pool: &State<sqlx::Pool<MySql>>, app_id: &str, page: Option<i64>, page_size: Option<i64>) -> Json<Vec<Backup>> {
    let app_id = app_id.parse::<i64>().unwrap_or(0);
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let backups = db::backup::list_backups_paginated(pool, app_id, page, page_size).await.unwrap_or_else(|_| vec![]);

    Json(backups)
}

