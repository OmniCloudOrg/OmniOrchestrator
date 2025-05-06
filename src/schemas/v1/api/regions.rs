use crate::models::region::Region;
use crate::models::provider::ProviderRegion;
use crate::schemas::v1::db::queries::{self as db};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, http::ContentType, post, put, Data, State};
use serde::{Deserialize, Serialize};
use sqlx::MySql;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

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
#[get("/regions?<page>&<per_page>")]
pub async fn list_regions(
    pool: &State<sqlx::Pool<MySql>>,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Json<Vec<Region>> {
    let regions = db::region::list_regions(pool, page, per_page)
        .await
        .expect("Failed to list regions");
    println!("Found {} regions", regions.len());
    let regions_vec: Vec<Region> = regions.into_iter().collect();
    println!("Returning {} regions", regions_vec.len());
    Json(regions_vec)
}

#[get("/provider_regions")]
pub async fn list_provider_regions(
    pool: &State<sqlx::Pool<MySql>>,
) -> Json<Vec<ProviderRegion>> {
    let regions = db::region::list_provider_regions(pool)
        .await
        .expect("Failed to list provider regions");
    println!("Found {} provider regions", regions.len());
    let regions_vec: Vec<ProviderRegion> = regions.into_iter().collect();
    println!("Returning {} provider regions", regions_vec.len());
    Json(regions_vec)
}
// TODO: (@tristanpoland) Fix these API endpoints to accept the correct data and pass it through to the query engine

// // Get a single region by ID
// #[get("/regions/<id>")]
// pub async fn get_region(id: String, pool: &State<sqlx::
// Pool<MySql>>) -> Result<Json<Region>, Status> {
//     let region = db::region::get_region_by_id(pool, &id).await.unwrap();
//     match region {
//         Some(region) => Ok(Json(region)),
//         None => Err(Status::NotFound),
//     }
// }
//
// // Create a new region
// #[post("/regions", format = "json", data = "<region_request>")]
// pub async fn create_region(
//     region_request: Json<CreateRegionRequest>,
//     pool: &State<sqlx::Pool<MySql>>
// ) -> Result<Json<Region>, Status> {
//     let region = db::region::create_region(
//         pool,
//         &region_request.name,
//         &region_request.description,
//         &region_request.url,
//         &region_request.org_id
//     ).await;
//     match region {
//         Ok(region) => Ok(Json(region)),
//         Err(_) => Err(Status::InternalServerError),
//     }
// }
//
// // Update an existing region
// #[put("/regions/<id>", format = "json", data = "<region_request>")]
// pub async fn update_region(
//     id: String,
//     region_request: Json<UpdateRegionRequest>,
//     pool: &State<sqlx::Pool<MySql>>
// )-> Result<Json<Region>, Status> {
//     let region = db::region::update_region(
//         pool,
//         &id,
//         &region_request.name,
//         &region_request.description,
//         &region_request.url,
//         &region_request.org_id
//     ).await;
//     match region {
//         Ok(region) => Ok(Json(region)),
//         Err(_) => Err(Status::InternalServerError),
//     }
// }
//
// // Delete a region
// #[delete("/regions/<id>")]
// pub async fn delete_region(id: String, pool: &State<sqlx::
// Pool<MySql>>) -> Result<Status, Status> {
//     let result = db::region::delete_region(pool, &id).await;
//     match result {
//         Ok(_) => Ok(Status::NoContent),
//         Err(_) => Err(Status::InternalServerError),
//     }
// }
