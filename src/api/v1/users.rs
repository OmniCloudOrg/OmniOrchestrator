use super::super::super::db::v1::queries::user::{create_user, login_user};
use crate::db::v1::tables::User;
use log;
use rand::rngs::OsRng;
use rand::Rng;
use rocket::{http::Status, serde::json};
use rocket::serde::json::json;
use rocket::post;
use rocket::response::status::Custom;
use rocket::State;
use sha2::{Digest, Sha256};
use sqlx::mysql::MySqlPool as Pool;

#[post("/users/create", data = "<data>")]
pub async fn handle_create_user(pool: &State<Pool>, data: String) -> Custom<String> {
    let data = match serde_json5::from_str::<serde_json::Value>(&data) {
        Ok(d) => d,
        Err(_) => return Custom(Status::BadRequest, String::from("Not a valid JSON object")),
    };

    let username = match data.get("username").and_then(|u| u.as_str()) {
        Some(u) => u,
        None => {
            return Custom(
                Status::BadRequest,
                String::from("Username is required and must be a string"),
            )
        }
    };

    let password = match data.get("password").and_then(|p| p.as_str()) {
        Some(p) => p,
        None => {
            return Custom(
                Status::BadRequest,
                String::from("Password is required and must be a string"),
            )
        }
    };

    let email = match data.get("email").and_then(|e| e.as_str()) {
        Some(e) => e,
        None => {
            return Custom(
                Status::BadRequest,
                String::from("Email is required and must be a string"),
            )
        }
    };

    let active = match data.get("active").and_then(|a| a.as_u64()) {
        Some(a) => a as i32,
        None => {
            return Custom(
                Status::BadRequest,
                String::from("Active status is required and must be a number"),
            )
        }
    };

    let mut rng = OsRng;
    let salt: [u8; 16] = rng.gen();
    let salted = format!("{}{}", password, hex::encode(salt));
    let mut hasher = Sha256::new();
    hasher.update(salted.as_bytes());
    let password_hash = hex::encode(hasher.finalize());

    match create_user(pool, username, &password_hash, email, &active.to_string()).await {
        Ok(_) => Custom(Status::Ok, String::from("User created successfully")),
        Err(e) => {
            log::error!("Error creating user: {}", e);
            Custom(
                Status::InternalServerError,
                String::from("Error creating user"),
            )
        }
    }
}

#[post("/users/login", data = "<data>")]
pub async fn handle_login(pool: &State<Pool>, data: String) -> Custom<String> {
    let data = match serde_json5::from_str::<serde_json::Value>(&data) {
        Ok(d) => d,
        Err(_) => return Custom(Status::BadRequest, String::from("Not a valid JSON object")),
    };

    let email = match data.get("email").and_then(|e| e.as_str()) {
        Some(e) => e,
        None => {
            return Custom(
                Status::BadRequest,
                String::from("Email is required and must be a string"),
            )
        }
    };

    let password = match data.get("password").and_then(|p| p.as_str()) {
        Some(p) => p,
        None => {
            return Custom(
                Status::BadRequest,
                String::from("Password is required and must be a string"),
            )
        }
    };

    // Note: You'll need to implement the actual login_user function in your queries module
    match login_user(pool, email, password).await {
        Ok(user) => Custom(Status::Ok, format!("Login successful: {:?}", user)),
        Err(e) => {
            log::error!("Error logging in user: {}", e);
            Custom(
                Status::InternalServerError,
                String::from("Error logging in user"),
            )
        }
    }
}

#[get("/me")]
pub async fn get_current_user(user: User) -> Result<rocket::serde::json::Value, Status> {
    Ok(json!(user))
}
