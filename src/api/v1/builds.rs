use rocket::{get, post, put, delete, State, http::ContentType, Data};
use crate::db::tables::Build;
use crate::db::v1::queries as db;
use rocket::http::Status;
use rocket::serde::json::Json;
use sqlx::MySql;

// List all builds (Paginated)
#[get("/builds?<page>&<per_page>")]
pub async fn list_builds(
    pool: &State<sqlx::Pool<MySql>>,
    page: Option<u32>,
    per_page: Option<u32>,
) -> Json<Vec<Build>> {
    let page: i64 = page.unwrap_or(1).into();
    let per_page: i64 = per_page.unwrap_or(10).into();
    
    let builds = db::build::list_builds_paginated(pool, per_page, page)
        .await
        .unwrap();

    Json(builds)
}

// List builds for a specific app (paginated)
#[get("/apps/<app_id>/builds?<page>&<per_page>")]
pub async fn list_builds_for_app(
    pool: &State<sqlx::Pool<MySql>>,
    app_id: i64,
    page: Option<u32>,
    per_page: Option<u32>,
) -> Json<Vec<Build>> {
    let page: i64 = page.unwrap_or(1).into();
    let per_page: i64 = per_page.unwrap_or(10).into();
    let offset = (page - 1) * per_page;
    
    let builds = db::build::list_builds_for_app_paginated(pool, app_id, per_page, offset)
        .await
        .unwrap();

    Json(builds)
}

// Get a build by ID
#[get("/builds/<build_id>")]
pub async fn get_build(
    pool: &State<sqlx::Pool<MySql>>,
    build_id: i64,
) -> Json<Build> {
    let build = db::build::get_build_by_id(pool, build_id)
        .await
        .unwrap();

    Json(build)
}