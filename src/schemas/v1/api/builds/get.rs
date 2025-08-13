use std::sync::Arc;
use crate::DatabaseManager;
use super::super::super::db::queries as db;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, http::Status, State};

use libomni::types::db::v1 as types;
use types::build::Build;

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