use crate::schemas::v1::models::platform::Platform;
use crate::db_manager::{self, DatabaseManager};
use rocket::{get, post, delete};
use rocket::serde::json::Json;
use rocket::http::Status;
use std::sync::Arc;
use rocket::State;
use log::{info, error}; // Add logging

#[get("/platforms")]
pub async fn list_platforms(db_manager: &State<Arc<DatabaseManager>>) -> Json<Vec<Platform>> {
    info!("Listing all platforms");
    let platforms = db_manager.get_all_platforms().await.unwrap_or_default();
    Json(platforms)
}

#[post("/platforms", data = "<platform_data>")]
pub async fn add_platform(
    platform_data: Json<Platform>,
    db_manager: &State<Arc<DatabaseManager>>
) -> Status {
    info!("Adding new platform: {:?}", platform_data.name);

    // Make a mutable copy to modify fields
    let mut platform = platform_data.into_inner();
    if platform.table_name.is_none() {
        platform.table_name = Some(format!("omni_p_{}", platform.name));
    }
    if platform.subdomain.is_none() {
        platform.subdomain = Some(platform.name.clone());
    }

    info!("Platform Data: {:?}", platform);

    match db_manager.create_platform(
        db_manager,
        platform.clone()
    ).await {
        Ok(platform_id) => {
            info!("Platform created with id: {}", platform_id);
            if let Err(_e) = db_manager.get_platform_pool(&platform.name, platform_id).await {
                error!("Failed to initialize connection pool for platform: {}", platform.name);
                return Status::InternalServerError;
            }
            Status::Created
        }
        Err(e) => {
            error!("Failed to create platform: {:?}", e);
            Status::InternalServerError
        }
    }
}

#[delete("/platforms/<platform_id>")]
pub async fn remove_platform(
    platform_id: i64,
    db_manager: &State<Arc<DatabaseManager>>
) -> Status {
    info!("Removing platform with id: {}", platform_id);
    match db_manager.delete_platform(platform_id).await {
        Ok(_) => {
            info!("Platform {} removed successfully", platform_id);
            Status::NoContent
        },
        Err(e) => {
            error!("Failed to remove platform {}: {:?}", platform_id, e);
            Status::InternalServerError
        }
    }
}