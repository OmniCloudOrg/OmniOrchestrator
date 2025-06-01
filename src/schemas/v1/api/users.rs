use super::super::db::queries::user::{
    create_user, get_user_by_email, record_login_attempt, create_session,
    invalidate_session, update_user_security, update_user_pii, update_user_meta,
    get_user_sessions, get_user_meta, get_user_pii
};

use crate::schemas::v1::db::queries;
use super::super::db::queries::user::invalidate_all_user_sessions;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use log;
use rand::rngs::OsRng;
use rand::{RngCore, TryRngCore};
use rocket::{http::Status, http::Cookie, http::CookieJar};
use rocket::serde::json::json;
use rocket::{get, post, put};
use rocket::response::status::Custom;
use rocket::State;
use sha2::{Digest, Sha256};
use sqlx::mysql::MySqlPool as Pool;
use uuid::Uuid;

use libomni::types::db::v1 as types;
use libomni::types::db::auth::{AuthConfig, Claims};
use types::user::User;

/// Register a new user
#[post("/auth/register", data = "<data>")]
pub async fn handle_register(
    pool: &State<Pool>, 
    data: String,
    cookies: &CookieJar<'_>,
    auth_config: &State<AuthConfig>
) -> Result<rocket::serde::json::Value, Custom<String>> {
    let data = match serde_json5::from_str::<serde_json::Value>(&data) {
        Ok(d) => d,
        Err(_) => return Err(Custom(Status::BadRequest, String::from("Not a valid JSON object"))),
    };

    let email = match data.get("email").and_then(|e| e.as_str()) {
        Some(e) => e,
        None => {
            return Err(Custom(
                Status::BadRequest,
                String::from("Email is required and must be a string"),
            ))
        }
    };

    let password = match data.get("password").and_then(|p| p.as_str()) {
        Some(p) => p,
        None => {
            return Err(Custom(
                Status::BadRequest,
                String::from("Password is required and must be a string"),
            ))
        }
    };

    let name = match data.get("name").and_then(|n| n.as_str()) {
        Some(n) => n,
        None => {
            return Err(Custom(
                Status::BadRequest,
                String::from("Name is required and must be a string"),
            ))
        }
    };

    // Check if user already exists
    if let Ok(_) = get_user_by_email(pool, email).await {
        return Err(Custom(
            Status::Conflict,
            String::from("User with this email already exists"),
        ));
    }

    // Generate a random salt and hash the password
    let mut rng = OsRng;
    let mut salt = [0u8; 16];
    rng.try_fill_bytes(&mut salt);
    let salt_hex = hex::encode(salt);
    let salted = format!("{}{}", password, salt_hex);
    let mut hasher = Sha256::new();
    hasher.update(salted.as_bytes());
    let password_hash = hex::encode(hasher.finalize());

    // Create the user
    let user = match create_user(pool, email, &password_hash, &salt_hex).await {
        Ok(user) => user,
        Err(e) => {
            log::error!("Error creating user: {}", e);
            return Err(Custom(
                Status::InternalServerError,
                String::from("Error creating user"),
            ));
        }
    };

    // Create a JWT token for the new user
    let (token, session_id) = create_auth_token_and_session(
        pool,
        &user,
        auth_config,
    ).await?;

    // Set session cookie if using cookies
    let mut cookie = Cookie::new("session_id", session_id.to_string());
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_same_site(rocket::http::SameSite::Strict);
    cookies.add(cookie);

    Ok(json!({
        "token": token,
        "user": {
            "id": user.id,
            "email": user.email,
            "created_at": user.created_at,
            "active": user.active
        }
    }))
}

/// Login a user
#[post("/auth/login", data = "<data>")]
pub async fn handle_login(
    pool: &State<Pool>, 
    auth_config: &State<AuthConfig>,
    data: String,
    cookies: &CookieJar<'_>
) -> Result<rocket::serde::json::Value, Custom<String>> {
    let data = match serde_json5::from_str::<serde_json::Value>(&data) {
        Ok(d) => d,
        Err(_) => return Err(Custom(Status::BadRequest, String::from("Not a valid JSON object"))),
    };

    let email = match data.get("email").and_then(|e| e.as_str()) {
        Some(e) => e,
        None => {
            return Err(Custom(
                Status::BadRequest,
                String::from("Email is required and must be a string"),
            ))
        }
    };

    let password = match data.get("password").and_then(|p| p.as_str()) {
        Some(p) => p,
        None => {
            return Err(Custom(
                Status::BadRequest,
                String::from("Password is required and must be a string"),
            ))
        }
    };

    // Get user from database
    let user = match get_user_by_email(pool, email).await {
        Ok(user) => {
            // Check if account is active
            if !user.active {
                return Err(Custom(
                    Status::Forbidden,
                    String::from("Account is inactive"),
                ));
            }
            
            // Check password
            let salted = format!("{}{}", password, user.salt);
            let mut hasher = Sha256::new();
            hasher.update(salted.as_bytes());
            let hashed_password = hex::encode(hasher.finalize());
            
            if hashed_password != user.password {
                // Record the failed attempt
                let _ = record_login_attempt(pool, user.id, false).await;
                return Err(Custom(
                    Status::Unauthorized,
                    String::from("Invalid credentials"),
                ));
            }
            
            // Record successful login
            match record_login_attempt(pool, user.id, true).await {
                Ok(updated_user) => updated_user,
                Err(e) => {
                    log::error!("Error recording login attempt: {}", e);
                    user
                }
            }
        },
        Err(_) => {
            return Err(Custom(
                Status::Unauthorized,
                String::from("Invalid credentials"),
            ));
        }
    };

    // Create JWT token and session
    let (token, session_id) = create_auth_token_and_session(
        pool,
        &user,
        auth_config,
    ).await?;

    // Set session cookie
    let mut cookie = Cookie::new("session_id", session_id.to_string());
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_same_site(rocket::http::SameSite::Strict);
    cookies.add(cookie);

    Ok(json!({
        "token": token,
        "user": {
            "id": user.id,
            "email": user.email,
            "created_at": user.created_at,
            "active": user.active
        }
    }))
}

/// Get the current user's profile
#[get("/auth/me")]
pub async fn get_current_user(
    user: User,
) -> Result<rocket::serde::json::Value, Custom<String>> {
    // Just return the basic user info
    Ok(json!({
        "id": user.id,
        "email": user.email,
        "created_at": user.created_at,
        "updated_at": user.updated_at,
        "active": user.active,
        "last_login_at": user.last_login_at,
    }))
}

/// Update user profile information
#[put("/users/profile", data = "<data>")]
pub async fn update_profile(
    user: User,
    pool: &State<Pool>,
    data: String,
) -> Result<rocket::serde::json::Value, Custom<String>> {
    let data = match serde_json5::from_str::<serde_json::Value>(&data) {
        Ok(d) => d,
        Err(_) => return Err(Custom(Status::BadRequest, String::from("Not a valid JSON object"))),
    };

    // Extract optional profile fields
    let first_name = data.get("first_name").and_then(|f| f.as_str());
    let last_name = data.get("last_name").and_then(|l| l.as_str());
    let full_name = data.get("full_name").and_then(|f| f.as_str());

    // Update PII info if provided
    if first_name.is_some() || last_name.is_some() || full_name.is_some() {
        if let Err(e) = update_user_pii(pool, user.id, first_name, last_name, full_name).await {
            log::error!("Error updating user PII: {}", e);
            return Err(Custom(
                Status::InternalServerError,
                String::from("Error updating profile information"),
            ));
        }
    }

    // Extract user preferences
    let timezone = data.get("timezone").and_then(|t| t.as_str());
    let language = data.get("language").and_then(|l| l.as_str());
    let theme = data.get("theme").and_then(|t| t.as_str());
    let onboarding_completed = data.get("onboarding_completed").and_then(|o| o.as_bool());

    // Update meta info if provided
    if timezone.is_some() || language.is_some() || theme.is_some() || onboarding_completed.is_some() {
        if let Err(e) = update_user_meta(
            pool, 
            user.id, 
            timezone, 
            language, 
            theme, 
            onboarding_completed
        ).await {
            log::error!("Error updating user preferences: {}", e);
            return Err(Custom(
                Status::InternalServerError,
                String::from("Error updating profile preferences"),
            ));
        }
    }

    Ok(json!({
        "message": "Profile updated successfully"
    }))
}

/// Change user password
#[put("/auth/change-password", data = "<data>")]
pub async fn change_password(
    user: User,
    pool: &State<Pool>,
    auth_config: &State<AuthConfig>,
    data: String,
) -> Result<rocket::serde::json::Value, Custom<String>> {
    // Parse the incoming JSON data
    let data = match serde_json5::from_str::<serde_json::Value>(&data) {
        Ok(d) => d,
        Err(_) => return Err(Custom(
            Status::BadRequest, 
            String::from("Invalid JSON request")
        )),
    };

    // Validate current password
    let current_password = match data.get("current_password").and_then(|p| p.as_str()) {
        Some(p) if !p.is_empty() => p,
        _ => return Err(Custom(
            Status::BadRequest,
            String::from("Current password is required")
        )),
    };

    // Validate new password
    let new_password = match data.get("new_password").and_then(|p| p.as_str()) {
        Some(p) if is_password_valid(p) => p,
        _ => return Err(Custom(
            Status::BadRequest,
            String::from("Invalid new password. Must be 12+ characters with mix of uppercase, lowercase, numbers, and symbols")
        )),
    };

    // Prevent password reuse
    if current_password == new_password {
        return Err(Custom(
            Status::BadRequest,
            String::from("New password cannot be the same as current password")
        ));
    }

    // Verify current password
    let salted_current = format!("{}{}", current_password, user.salt);
    let mut current_hasher = Sha256::new();
    current_hasher.update(salted_current.as_bytes());
    let current_hashed_password = hex::encode(current_hasher.finalize());
    
    log::info!("DB Salt: {}", user.salt);
    log::info!("DB Password Hash: {}", user.password);
    log::info!("Current Password: {}", current_password);
    log::info!("Salted Current: {}", salted_current);
    log::info!("Computed Hash: {}", current_hashed_password);

    // Constant-time comparison to prevent timing attacks
    if current_hashed_password != user.password {
        return Err(Custom(
            Status::Unauthorized,
            String::from("Current password is incorrect")
        ));
    }

    // Generate a new salt for the new password
    let mut rng = OsRng;
    let mut new_salt = [0u8; 16];
    rng.try_fill_bytes(&mut new_salt);
    let new_salt_hex = hex::encode(new_salt);
    
    // Hash the new password
    let new_salted = format!("{}{}", new_password, new_salt_hex);
    let mut new_hasher = Sha256::new();
    new_hasher.update(new_salted.as_bytes());
    let new_password_hash = hex::encode(new_hasher.finalize());

    // Update password with additional security settings
    match update_user_security(
        pool,
        user.id,
        Some(&new_password_hash),
        Some(&new_salt_hex),
        None,
        None,
    ).await {
        Ok(_) => {
            // Invalidate all existing sessions after password change
            match invalidate_all_user_sessions(pool, user.id).await {
                Ok(_) => log::info!("All sessions invalidated for user {}", user.id),
                Err(e) => log::warn!("Failed to invalidate sessions: {}", e),
            }

            Ok(json!({
                "message": "Password changed successfully",
                "action": "All existing sessions have been terminated"
            }))
        },
        Err(e) => {
            log::error!("Password update failed: {}", e);
            Err(Custom(
                Status::InternalServerError,
                String::from("Failed to update password")
            ))
        }
    }
}

/// Validate password strength
fn is_password_valid(password: &str) -> bool {
    // Comprehensive password strength check
    password.len() >= 12 && 
    password.chars().any(|c| c.is_uppercase()) &&
    password.chars().any(|c| c.is_lowercase()) &&
    password.chars().any(|c| c.is_numeric()) &&
    password.chars().any(|c| !c.is_alphanumeric())
}

/// Constant-time comparison to prevent timing attacks
fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    a.bytes().zip(b.bytes()).fold(0, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Logout the current user
#[post("/auth/logout")]
pub async fn logout(
    cookies: &CookieJar<'_>,
    _user: User, // Renamed to _user since we don't use it directly
    pool: &State<Pool>,
) -> Result<rocket::serde::json::Value, Custom<String>> {
    // Get session token from cookie
    if let Some(session_cookie) = cookies.get("session_id") {
        // Invalidate the session
        if let Err(e) = invalidate_session(pool, session_cookie.value()).await {
            log::error!("Error invalidating session: {}", e);
            return Err(Custom(
                Status::InternalServerError,
                String::from("Error logging out"),
            ));
        }

        // Remove session cookie
        cookies.remove(Cookie::named("session_id"));
    }

    Ok(json!({
        "message": "Logged out successfully"
    }))
}

/// Helper function to create a JWT token and session
async fn create_auth_token_and_session(
    pool: &State<Pool>,
    user: &User,
    auth_config: &State<AuthConfig>,
) -> Result<(String, i64), Custom<String>> {
    // Create JWT token
    let now = Utc::now();
    let exp = (now + Duration::hours(auth_config.token_expiry_hours)).timestamp() as usize;
    
    // Generate a unique session token
    let session_token = Uuid::new_v4().to_string();
    
    let claims = Claims {
        sub: user.id.to_string(),
        exp,
        iat: now.timestamp() as usize,
        user_data: user.clone(),
    };
    
    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                String::from("Error creating authentication token"),
            ));
        }
    };
    
    // Create a new session
    let ip = "unknown".to_string();
    let ua = "unknown".to_string();
    let expires_at = now + Duration::hours(auth_config.token_expiry_hours);
    
    let session_id = match create_session(
        pool,
        user.id,
        &session_token,
        None,
        &ip,
        &ua,
        expires_at,
    ).await {
        Ok(id) => id,
        Err(e) => {
            log::error!("Error creating session: {}", e);
            return Err(Custom(
                Status::InternalServerError,
                String::from("Error creating user session"),
            ));
        }
    };
    
    Ok((token, session_id))
}

///list all sessions for a user
#[get("/auth/sessions")]
pub async fn list_user_sessions(
    user: User,
    pool: &State<Pool>,
) -> Result<rocket::serde::json::Value, Custom<String>> {
    // Fetch user sessions
    match get_user_sessions(pool, user.id).await {
        Ok(sessions) => Ok(json!({
            "sessions": sessions
        })),
        Err(e) => {
            log::error!("Error fetching user sessions: {}", e);
            Err(Custom(
                Status::InternalServerError,
                String::from("Error fetching user sessions"),
            ))
        }
    }
}

/// Invalidate a specific session
#[delete("/auth/sessions/<session_id>")]
pub async fn invalidate_user_session(
    user: User,
    session_id: String,
    pool: &State<Pool>,
) -> Result<rocket::serde::json::Value, Custom<String>> {
    // Invalidate the session
    match invalidate_session(pool, &session_id).await {
        Ok(_) => Ok(json!({
            "message": "Session invalidated successfully"
        })),
        Err(e) => {
            log::error!("Error invalidating session: {}", e);
            Err(Custom(
                Status::InternalServerError,
                String::from("Error invalidating session"),
            ))
        }
    }
}

/// Get the current user's complete profile including meta and PII data
#[get("/users/profile")]
pub async fn get_user_profile(
    user: User,
    pool: &State<Pool>,
) -> Result<rocket::serde::json::Value, Custom<String>> {
    // Fetch user meta data
    let user_meta = match get_user_meta(pool, user.id).await {
        Ok(meta) => meta,
        Err(e) => {
            log::error!("Error fetching user meta: {}", e);
            return Err(Custom(
                Status::InternalServerError,
                String::from("Error fetching user preferences"),
            ));
        }
    };
    
    // Fetch user PII data
    let user_pii = match get_user_pii(pool, user.id).await {
        Ok(pii) => pii,
        Err(e) => {
            log::error!("Error fetching user PII: {}", e);
            return Err(Custom(
                Status::InternalServerError,
                String::from("Error fetching user personal information"),
            ));
        }
    };
    
    // Combine all data into a single response
    Ok(json!({
        // Basic user information
        "id": user.id,
        "email": user.email,
        "email_verified": user.email_verified > 0,
        "active": user.active,
        "status": user.status,
        "created_at": user.created_at,
        "updated_at": user.updated_at,
        "last_login_at": user.last_login_at,
        
        // User meta information
        "timezone": user_meta.timezone,
        "language": user_meta.language,
        "theme": user_meta.theme,
        "notification_preferences": user_meta.notification_preferences,
        "profile_image": user_meta.profile_image,
        "dashboard_layout": user_meta.dashboard_layout,
        "onboarding_completed": user_meta.onboarding_completed > 0,
        
        // User PII information
        "first_name": user_pii.first_name,
        "last_name": user_pii.last_name,
        "full_name": user_pii.full_name,
        "identity_verified": user_pii.identity_verified > 0
    }))
}

/// Update user profile information - handles both PII and meta updates in a single endpoint
#[put("/users/profile", data = "<data>")]
pub async fn update_user_profile(
    user: User,
    pool: &State<Pool>,
    data: String,
) -> Result<rocket::serde::json::Value, Custom<String>> {
    // Parse the incoming JSON data
    let data = match serde_json5::from_str::<serde_json::Value>(&data) {
        Ok(d) => d,
        Err(_) => return Err(Custom(
            Status::BadRequest, 
            String::from("Invalid JSON request")
        )),
    };
    
    // Extract optional profile fields (PII)
    let first_name = data.get("first_name").and_then(|f| f.as_str());
    let last_name = data.get("last_name").and_then(|l| l.as_str());
    let full_name = data.get("full_name").and_then(|f| f.as_str());

    // Update PII info if provided
    if first_name.is_some() || last_name.is_some() || full_name.is_some() {
        if let Err(e) = update_user_pii(pool, user.id, first_name, last_name, full_name).await {
            log::error!("Error updating user PII: {}", e);
            return Err(Custom(
                Status::InternalServerError,
                String::from("Error updating profile information"),
            ));
        }
    }

    // Extract user preferences
    let timezone = data.get("timezone").and_then(|t| t.as_str());
    let language = data.get("language").and_then(|l| l.as_str());
    let theme = data.get("theme").and_then(|t| t.as_str());
    let onboarding_completed = data.get("onboarding_completed").and_then(|o| o.as_bool());

    // Update meta info if provided
    if timezone.is_some() || language.is_some() || theme.is_some() || onboarding_completed.is_some() {
        if let Err(e) = update_user_meta(
            pool, 
            user.id, 
            timezone, 
            language, 
            theme, 
            onboarding_completed
        ).await {
            log::error!("Error updating user preferences: {}", e);
            return Err(Custom(
                Status::InternalServerError,
                String::from("Error updating profile preferences"),
            ));
        }
    }

    Ok(json!({
        "message": "Profile updated successfully",
        "updated_fields": {
            "pii": {
                "first_name": first_name.is_some(),
                "last_name": last_name.is_some(),
                "full_name": full_name.is_some(),
            },
            "meta": {
                "timezone": timezone.is_some(),
                "language": language.is_some(),
                "theme": theme.is_some(),
                "onboarding_completed": onboarding_completed.is_some(),
            }
        }
    }))
}

/// List  all users
#[get("/users?<page>&<per_page>")]
pub async fn list_users(
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &State<Pool>,
) -> Result<rocket::serde::json::Value, Custom<String>> {
    match (page, per_page) {
        (Some(page), Some(per_page)) => {
            // Fetch paginated users and total count
            let users = match queries::user::list_users(pool, page, per_page).await {
                Ok(u) => u,
                Err(e) => {
                    log::error!("Error fetching users: {}", e);
                    return Err(Custom(
                        Status::InternalServerError,
                        String::from("Error fetching users"),
                    ));
                }
            };
            let total_count = match queries::user::count_users(pool).await {
                Ok(c) => c,
                Err(e) => {
                    log::error!("Error counting users: {}", e);
                    return Err(Custom(
                        Status::InternalServerError,
                        String::from("Error counting users"),
                    ));
                }
            };
            let total_pages = ((total_count as f64) / (per_page as f64)).ceil() as i64;

            Ok(json!({
                "users": users,
                "pagination": {
                    "page": page,
                    "per_page": per_page,
                    "total_count": total_count,
                    "total_pages": total_pages
                }
            }))
        }
        _ => Err(Custom(
            Status::BadRequest,
            String::from("Missing pagination parameters: please provide both 'page' and 'per_page'")
        ))
    }
}