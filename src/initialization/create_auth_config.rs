use libomni::types::db::auth::AuthConfig;

/// Constructs the authentication configuration from environment variables.
///
/// - Loads the JWT secret and token expiry hours from environment variables.
/// - Panics if `JWT_SECRET` is not set or if `TOKEN_EXPIRY_HOURS` is invalid.
///
/// # Returns
/// Returns an `AuthConfig` struct with the loaded values.
pub fn create_auth_config() -> AuthConfig {
    AuthConfig {
        jwt_secret: std::env::var("JWT_SECRET")
            .expect("Environment variable JWT_SECRET must be set for secure operation."),
        token_expiry_hours: std::env::var("TOKEN_EXPIRY_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .expect("Invalid value for TOKEN_EXPIRY_HOURS"),
    }
}
