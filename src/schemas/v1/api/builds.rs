use std::sync::Arc;
use crate::DatabaseManager;
use crate::models::build::Build;
use super::super::db::queries as db;
use rocket::serde::json::{self, json, Json, Value};
use rocket::{delete, get, http::{ContentType, Status}, post, put, Data, State};

/// List all builds with pagination support.
#[get("/platform/<platform_id>/builds?<page>&<per_page>")]
pub async fn list_builds(
    platform_id: i64,
    page: Option<u32>,
    per_page: Option<u32>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    // Get platform information
    let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(platform) => platform,
        Err(_) => {
            return Err((
                Status::NotFound,
                Json(json!({
                    "error": "Platform not found",
                    "message": format!("Platform with ID {} does not exist", platform_id)
                }))
            ));
        }
    };

    // Get platform-specific database pool
    let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
        Ok(pool) => pool,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to connect to platform database"
                }))
            ));
        }
    };

    let page: i64 = page.unwrap_or(0).into();
    let per_page: i64 = per_page.unwrap_or(10).into();
    let offset = page * per_page;
    
    let builds = match db::build::list_builds_paginated(&pool, per_page, offset).await {
        Ok(builds) => builds,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to list builds"
                }))
            ));
        }
    };
    
    let total_count = match db::build::get_total_build_count(&pool).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count builds"
                }))
            ));
        }
    };
    
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
    
    Ok(Json(response))
}

/// List builds for a specific application with pagination support.
#[get("/platform/<platform_id>/apps/<app_id>/builds?<page>&<per_page>")]
pub async fn list_builds_for_app(
    platform_id: i64,
    app_id: i64,
    page: Option<u32>,
    per_page: Option<u32>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Vec<Build>>, (Status, Json<Value>)> {
    // Get platform information
    let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(platform) => platform,
        Err(_) => {
            return Err((
                Status::NotFound,
                Json(json!({
                    "error": "Platform not found",
                    "message": format!("Platform with ID {} does not exist", platform_id)
                }))
            ));
        }
    };

    // Get platform-specific database pool
    let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
        Ok(pool) => pool,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to connect to platform database"
                }))
            ));
        }
    };

    let page: i64 = page.unwrap_or(1).into();
    let per_page: i64 = per_page.unwrap_or(10).into();
    let offset = (page - 1) * per_page;
    
    match db::build::list_builds_for_app_paginated(&pool, app_id, per_page, offset).await {
        Ok(builds) => Ok(Json(builds)),
        Err(_) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Database error",
                "message": "Failed to list builds for application"
            }))
        )),
    }
}

/// Get a specific build by ID.
#[get("/platform/<platform_id>/builds/<build_id>")]
pub async fn get_build(
    platform_id: i64,
    build_id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Build>, (Status, Json<Value>)> {
    // Get platform information
    let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(platform) => platform,
        Err(_) => {
            return Err((
                Status::NotFound,
                Json(json!({
                    "error": "Platform not found",
                    "message": format!("Platform with ID {} does not exist", platform_id)
                }))
            ));
        }
    };

    // Get platform-specific database pool
    let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
        Ok(pool) => pool,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to connect to platform database"
                }))
            ));
        }
    };

    match db::build::get_build_by_id(&pool, build_id).await {
        Ok(build) => Ok(Json(build)),
        Err(_) => Err((
            Status::NotFound,
            Json(json!({
                "error": "Build not found",
                "message": format!("Build with ID {} could not be found", build_id)
            }))
        )),
    }
}