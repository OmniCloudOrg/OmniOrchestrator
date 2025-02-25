use rocket::http::Status;
use serde::{Deserialize, Serialize};
use rocket::get;

#[derive(Debug,Serialize,Deserialize)]
pub struct DeployPermissions {
    max_file_count: u64
}
impl Default for DeployPermissions {
    fn default() -> Self {
        Self { max_file_count: 45000000 }
    }
}
//TODO: replace with proxy
#[get("/deploy/permissions")]
pub fn deploy_permissions() -> Result<rocket::serde::json::Json<DeployPermissions>,Status> {

    Ok(rocket::serde::json::Json(DeployPermissions::default()))
}
 