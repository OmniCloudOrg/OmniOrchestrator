use crate::models::instance::Instance;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::MySql;

use crate::schemas::v1::db::queries::{self as db};

/// List all instances by `region_id` and `app_id`
#[get("/apps/<app_id>/instances/region/<region_id>?<page>&<per_page>")]
pub async fn list_instances_by_region(
    pool: &State<sqlx::Pool<MySql>>,
    app_id: i64,
    region_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Json<Vec<Instance>> {
    // Default to page 1 and 10 items per page
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let instances = db::instance::list_instances_by_region(pool, region_id, app_id, page, per_page)
        .await
        .unwrap();
    println!("Found {} instances", instances.len());
    let instances_vec: Vec<Instance> = instances.into_iter().collect();
    println!("Returning {} apps", instances_vec.len());
    Json(instances_vec)
}

/// Count all instances across all applications
#[get("/instances/count")]
pub async fn count_instances(pool: &State<sqlx::Pool<MySql>>) -> Json<i64> {
    let count = db::instance::count_instances(pool).await.unwrap();
    println!("Found {} instances", count);
    Json(count)
}

// Get an instance by ID
#[get("/instances/<instance_id>")]
pub async fn get_instance(pool: &State<sqlx::Pool<MySql>>, instance_id: i64) -> Json<Instance> {
    let instance = db::instance::get_instance_by_id(pool, instance_id)
        .await
        .unwrap();
    println!("Found instance: {:?}", instance);
    Json(instance)
}

// Create a new instance
// NOTE: While this technically works we do not enable the endpoint in the API bacause you are meant to use the scaling tools to create instances

//   #[post("/app/<app_id>/instances")]
//   pub async fn create_instance(pool: &State<sqlx::Pool<MySql>>, app_id: i64, data: Json<CreateInstanceRequest>) -> Status {
//       let instance = db::instance::create_instance(pool, app_id, data.name.clone(), data.memory, data.cpu).await.unwrap();
//       println!("Created instance: {:?}", instance);
//       Status::Created
//   }

// Delete an instance
// NOTE: While this technically works we do not enable the endpoint in the API bacause you are meant to use the scaling tools to delete instances

//   #[delete("/instances/<instance_id>")]
//   pub async fn delete_instance(pool: &State<sqlx::Pool<MySql>>, instance_id: i64) -> Status {
//       let instance = db::instance::delete_instance(pool, instance_id).await.unwrap();
//       println!("Deleted instance: {:?}", instance);
//       Status::Ok
//   }
