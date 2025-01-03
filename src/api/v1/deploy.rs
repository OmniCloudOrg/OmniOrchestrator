use rocket::{get, post, put, delete, State, http::ContentType, Data};
use rocket::http::Status;
use rocket::serde::json::{Json, Value, json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Debug,Serialize,Deserialize)]
pub struct DeployPermissions {
    max_file_count: u64
}
impl Default for DeployPermissions {
    fn default() -> Self {
        Self { max_file_count: 45000000 }
    }
}
#[get("/deploy/permissions")]
pub fn deploy_permissions() -> Result<rocket::serde::json::Json<DeployPermissions>,Status> {

    Ok(rocket::serde::json::Json(DeployPermissions::default()))
}