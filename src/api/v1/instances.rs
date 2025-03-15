use crate::db::v1::tables::Instance;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::MySql;

use crate::db::v1::queries::{self as db};

// List all applications
#[get("/apps/<app_id>/instances")]
pub async fn list_instances(pool: &State<sqlx::Pool<MySql>>, app_id: i64) -> Json<Vec<Instance>> {
    let instances = db::instance::list_instances(pool, app_id).await.unwrap();
    println!("Found {} instances", instances.len());
    let instances_vec: Vec<Instance> = instances.into_iter().collect();
    println!("Returning {} apps", instances_vec.len());
    Json(instances_vec)
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
