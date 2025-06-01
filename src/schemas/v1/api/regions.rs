use crate::DatabaseManager;
use crate::schemas::v1::db::queries::{self as db};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, http::ContentType, post, put, Data, State};
use serde::{Deserialize, Serialize};
use sqlx::MySql;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use libomni::types::db::v1 as types;
use types::region::Region;
use types::provider::ProviderRegion;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRegionRequest {
    /// The name of the region, e.g. 'us-east-1'
    name: String,
    /// A human-friendly name for the region, e.g. 'US East'
    display_name: String,
    /// What provider is responsible for this region? eg. 'kubernetes', 'docker', 'aws', 'gcp', 'azure', 'detee'
    provider: String,
    /// What does your cloud provicer call this region? e.g. 'us-east-1'
    provider_region: Option<String>,
    /// Country / State / province / City in which the data center is located
    location: Option<String>,
    /// Coordinates for the region, can be used for geolocation or mapping
    coordinates: Option<String>, // Will need conversion to POINT in DB layer
    /// The status of the region, can be used to indicate if the region is usable or not
    status: Option<String>, // ENUM('active', 'maintenance', 'offline', 'deprecated')
    /// Can be used to indicate if the region is visible and usable to all orgs
    is_public: Option<bool>,
    /// If you have multiple types of regions, for certain kinds of compute optimization
    class: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRegionRequest {
    name: String,
    description: String,
    url: String,
    org_id: i64,
}

// List all regions paginated
#[get("/platform/<platform_id>/regions?<page>&<per_page>")]
pub async fn list_regions(
    platform_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Vec<Region>>, (Status, Json<Value>)> {
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

    let regions = match db::region::list_regions(&pool, page, per_page).await {
        Ok(regions) => regions,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to list regions"
                }))
            ));
        }
    };
    
    println!("Found {} regions", regions.len());
    let regions_vec: Vec<Region> = regions.into_iter().collect();
    println!("Returning {} regions", regions_vec.len());
    
    Ok(Json(regions_vec))
}

#[get("/platform/<platform_id>/provider_regions")]
pub async fn list_provider_regions(
    platform_id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Vec<ProviderRegion>>, (Status, Json<Value>)> {
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

    let regions = match db::region::list_provider_regions(&pool).await {
        Ok(regions) => regions,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to list provider regions"
                }))
            ));
        }
    };
    
    println!("Found {} provider regions", regions.len());
    let regions_vec: Vec<ProviderRegion> = regions.into_iter().collect();
    println!("Returning {} provider regions", regions_vec.len());
    
    Ok(Json(regions_vec))
}

// Here are the commented routes updated to be platform-specific:

// // Get a single region by ID
// #[get("/platform/<platform_id>/regions/<id>")]
// pub async fn get_region(
//     platform_id: i64,
//     id: String, 
//     db_manager: &State<Arc<DatabaseManager>>
// ) -> Result<Json<Region>, (Status, Json<Value>)> {
//     // Get platform information
//     let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
//         Ok(platform) => platform,
//         Err(_) => {
//             return Err((
//                 Status::NotFound,
//                 Json(json!({
//                     "error": "Platform not found",
//                     "message": format!("Platform with ID {} does not exist", platform_id)
//                 }))
//             ));
//         }
//     };
//
//     // Get platform-specific database pool
//     let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
//         Ok(pool) => pool,
//         Err(_) => {
//             return Err((
//                 Status::InternalServerError,
//                 Json(json!({
//                     "error": "Database error",
//                     "message": "Failed to connect to platform database"
//                 }))
//             ));
//         }
//     };
//
//     let region = match db::region::get_region_by_id(&pool, &id).await {
//         Ok(Some(region)) => region,
//         Ok(None) => {
//             return Err((
//                 Status::NotFound,
//                 Json(json!({
//                     "error": "Region not found",
//                     "message": format!("Region with ID {} does not exist", id)
//                 }))
//             ));
//         },
//         Err(_) => {
//             return Err((
//                 Status::InternalServerError,
//                 Json(json!({
//                     "error": "Database error",
//                     "message": "Failed to retrieve region"
//                 }))
//             ));
//         }
//     };
//     
//     Ok(Json(region))
// }
//
// // Create a new region
// #[post("/platform/<platform_id>/regions", format = "json", data = "<region_request>")]
// pub async fn create_region(
//     platform_id: i64,
//     region_request: Json<CreateRegionRequest>,
//     db_manager: &State<Arc<DatabaseManager>>
// ) -> Result<Json<Region>, (Status, Json<Value>)> {
//     // Get platform information
//     let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
//         Ok(platform) => platform,
//         Err(_) => {
//             return Err((
//                 Status::NotFound,
//                 Json(json!({
//                     "error": "Platform not found",
//                     "message": format!("Platform with ID {} does not exist", platform_id)
//                 }))
//             ));
//         }
//     };
//
//     // Get platform-specific database pool
//     let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
//         Ok(pool) => pool,
//         Err(_) => {
//             return Err((
//                 Status::InternalServerError,
//                 Json(json!({
//                     "error": "Database error",
//                     "message": "Failed to connect to platform database"
//                 }))
//             ));
//         }
//     };
//
//     let region = match db::region::create_region(
//         &pool,
//         &region_request.name,
//         &region_request.description,
//         &region_request.url,
//         &region_request.org_id
//     ).await {
//         Ok(region) => region,
//         Err(_) => {
//             return Err((
//                 Status::InternalServerError,
//                 Json(json!({
//                     "error": "Database error",
//                     "message": "Failed to create region"
//                 }))
//             ));
//         }
//     };
//     
//     Ok(Json(region))
// }
//
// // Update an existing region
// #[put("/platform/<platform_id>/regions/<id>", format = "json", data = "<region_request>")]
// pub async fn update_region(
//     platform_id: i64,
//     id: String,
//     region_request: Json<UpdateRegionRequest>,
//     db_manager: &State<Arc<DatabaseManager>>
// ) -> Result<Json<Region>, (Status, Json<Value>)> {
//     // Get platform information
//     let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
//         Ok(platform) => platform,
//         Err(_) => {
//             return Err((
//                 Status::NotFound,
//                 Json(json!({
//                     "error": "Platform not found",
//                     "message": format!("Platform with ID {} does not exist", platform_id)
//                 }))
//             ));
//         }
//     };
//
//     // Get platform-specific database pool
//     let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
//         Ok(pool) => pool,
//         Err(_) => {
//             return Err((
//                 Status::InternalServerError,
//                 Json(json!({
//                     "error": "Database error",
//                     "message": "Failed to connect to platform database"
//                 }))
//             ));
//         }
//     };
//
//     let region = match db::region::update_region(
//         &pool,
//         &id,
//         &region_request.name,
//         &region_request.description,
//         &region_request.url,
//         &region_request.org_id
//     ).await {
//         Ok(region) => region,
//         Err(_) => {
//             return Err((
//                 Status::InternalServerError,
//                 Json(json!({
//                     "error": "Database error",
//                     "message": "Failed to update region"
//                 }))
//             ));
//         }
//     };
//     
//     Ok(Json(region))
// }
//
// // Delete a region
// #[delete("/platform/<platform_id>/regions/<id>")]
// pub async fn delete_region(
//     platform_id: i64,
//     id: String, 
//     db_manager: &State<Arc<DatabaseManager>>
// ) -> Result<Status, (Status, Json<Value>)> {
//     // Get platform information
//     let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
//         Ok(platform) => platform,
//         Err(_) => {
//             return Err((
//                 Status::NotFound,
//                 Json(json!({
//                     "error": "Platform not found",
//                     "message": format!("Platform with ID {} does not exist", platform_id)
//                 }))
//             ));
//         }
//     };
//
//     // Get platform-specific database pool
//     let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
//         Ok(pool) => pool,
//         Err(_) => {
//             return Err((
//                 Status::InternalServerError,
//                 Json(json!({
//                     "error": "Database error",
//                     "message": "Failed to connect to platform database"
//                 }))
//             ));
//         }
//     };
//
//     match db::region::delete_region(&pool, &id).await {
//         Ok(_) => Ok(Status::NoContent),
//         Err(_) => {
//             return Err((
//                 Status::InternalServerError,
//                 Json(json!({
//                     "error": "Database error",
//                     "message": "Failed to delete region"
//                 }))
//             ));
//         }
//     }
// }