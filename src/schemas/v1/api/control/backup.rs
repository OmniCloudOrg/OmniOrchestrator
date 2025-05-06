use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post, delete};
use sqlx::{MySql, Pool};
//use super::super::super::db::tables::Backup;
use super::super::super::db::queries as db;

/// List all backups
#[get("/backups?<page>&<per_page>")]
pub async fn list_backups(pool: &State<sqlx::Pool<MySql>>, page: Option<i64>, per_page: Option<i64>) -> Json<Vec<Backup>> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let backups = db::backup::list_backups_paginated(pool, page, per_page).await.unwrap_or_else(|_| vec![]);

    Json(backups)
}

/// List backups by app_id
#[get("/apps/<app_id>/backups?<page>&<page_size>")]
pub async fn list_backups_by_app_id(pool: &State<sqlx::Pool<MySql>>, app_id: &str, page: Option<i64>, page_size: Option<i64>) -> Json<Vec<Backup>> {
    let app_id = app_id.parse::<i64>().unwrap_or(0);
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let backups = db::backup::list_backups_by_app_id(pool, app_id, page, page_size).await.unwrap_or_else(|_| vec![]);

    Json(backups)
}

/// Get a backup by ID
#[get("/backups/<backup_id>")]
pub async fn get_backup(pool: &State<sqlx::Pool<MySql>>, backup_id: &str) -> Json<Option<Backup>> {
    let backup_id = backup_id.parse::<i64>().unwrap_or(0);
    let backup = db::backup::get_backup_by_id(pool, backup_id).await.unwrap_or(None);

    Json(backup)
}

/// Create a new backup
#[post("/backups", format = "json", data = "<new_backup>")]
pub async fn create_backup(
    pool: &State<sqlx::Pool<MySql>>,
    new_backup: Json<Backup>,
) -> Result<Json<Backup>, rocket::http::Status> {
    let mut backup = Backup::new(); // Create a new instance of Backup to ensure defaults are set

    let metadata = new_backup.0.metadata.clone().unwrap_or_default();
    backup.update_metadata(metadata); // Update the metadata with provided or default value
    match db::backup::create_backup(pool, &backup).await {
        Ok(inserted_backup) => Ok(Json(inserted_backup)),
        Err(_) => {
            // Return a 500 Internal Server Error if the database operation fails
            Err(rocket::http::Status::InternalServerError)
        }
    }
}