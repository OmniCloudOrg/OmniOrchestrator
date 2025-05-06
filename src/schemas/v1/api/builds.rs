//! Build management module for handling application builds.
//!
//! This module provides a REST API for managing builds, including:
//! - Listing all builds with pagination
//! - Listing builds for a specific application
//! - Getting details of a specific build

use crate::models::build::Build;
use super::super::db::queries as db;
use rocket::serde::json::{self, json, Json};
use rocket::{delete, get, http::ContentType, post, put, Data, State};
use sqlx::MySql;

/// List all builds with pagination support.
///
/// This endpoint retrieves a paginated list of all builds in the system,
/// allowing administrators to monitor and track build activities.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `page` - Optional page number for pagination (defaults to 1)
/// * `per_page` - Optional number of items per page (defaults to 10)
///
/// # Returns
///
/// A JSON array of build objects
#[get("/builds?<page>&<per_page>")]
pub async fn list_builds(
    pool: &State<sqlx::Pool<MySql>>,
    page: Option<u32>,
    per_page: Option<u32>,
) -> Json<serde_json::Value> {
    let page: i64 = page.unwrap_or(0).into();
    let per_page: i64 = per_page.unwrap_or(10).into();
    let offset = page * per_page;

    let builds = db::build::list_builds_paginated(pool, per_page, offset)
        .await
        .unwrap();

    let total_count = db::build::get_total_build_count(pool).await.unwrap();
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

    let response = json!({
        "builds": builds,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    });

    Json(response)
}

/// List builds for a specific application with pagination support.
///
/// This endpoint retrieves a paginated list of builds for a specific application,
/// allowing users to track the build history of an application.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `app_id` - ID of the application to list builds for
/// * `page` - Optional page number for pagination (defaults to 1)
/// * `per_page` - Optional number of items per page (defaults to 10)
///
/// # Returns
///
/// A JSON array of build objects for the specified application
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

/// Get a specific build by ID.
///
/// This endpoint retrieves detailed information about a specific build,
/// which can be used to inspect the build status, logs, and other details.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `build_id` - ID of the build to retrieve
///
/// # Returns
///
/// A JSON object containing the build details
#[get("/builds/<build_id>")]
pub async fn get_build(pool: &State<sqlx::Pool<MySql>>, build_id: i64) -> Json<Build> {
    let build = db::build::get_build_by_id(pool, build_id).await.unwrap();

    Json(build)
}