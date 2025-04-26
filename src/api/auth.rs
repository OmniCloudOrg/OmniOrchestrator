use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::sync::Arc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use rocket::State;
use chrono::{DateTime, Utc, Duration};
use serde_json::json;
use rocket::serde::json::Json;
use rocket::{post, get, routes};

// User struct that will be used throughout the application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub roles: Vec<String>,
    // Add other fields you need
}

// JWT claims struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,         // Subject (user ID)
    pub exp: usize,          // Expiration time
    pub iat: usize,          // Issued at
    pub user_data: User,     // User data embedded in token
}

// Login request
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Auth config
#[derive(Debug)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry_hours: i64,
}

// Request guard for User
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the auth config from Rocket state
        let auth_config = match request.rocket().state::<AuthConfig>() {
            Some(cfg) => cfg,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };
        
        // Extract the token from the Authorization header
        let token = request
            .headers()
            .get_one("Authorization")
            .and_then(|value| {
                if value.starts_with("Bearer ") {
                    Some(value[7..].to_string())
                } else {
                    None
                }
            });

        match token {
            Some(token) => {
                // Validate the token
                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(token_data) => Outcome::Success(token_data.claims.user_data),
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            }
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
