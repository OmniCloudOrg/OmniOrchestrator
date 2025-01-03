use rocket::post;
use super::super::super::db::queries::{user_create, user_login};
use sha2::{Sha256, Digest};
use rand::{thread_rng, Rng};

pub fn test() {
    let path = std::path::Path::new("test");
   
}

#[post("/users/create", data = "<data>")]
pub fn create_user(data: String) -> rocket::response::status::Custom<String> {
    let data = serde_json::from_str::<serde_json::Value>(&data).expect("Not a valid JSON object");

    let username = data.get("username").expect("The username is required").as_str().expect("Username must be a string");
    let password = data.get("password").expect("The password is required").as_str().expect("Password must be a string");
    let mut rng = thread_rng();
    let salt: [u8; 16] = rng.gen();
    let salted = format!("{}{}", password, hex::encode(salt));
    let mut hasher = Sha256::new();
    hasher.update(salted.as_bytes());
    let password_hash = hex::encode(hasher.finalize());
    let email = data.get("email").expect("The email is required").as_str().expect("Email must be a string");
    let active = data.get("active").expect("Active status is required").as_u64().expect("Active must be a number") as i32;
    
    let resp = user_create(username, &password_hash, email, active);

    match resp {
        Ok(_) => rocket::response::status::Custom(rocket::http::Status::Ok, String::from("User created successfully")),
        Err(_) => {
            log::error!("Error creating user {}", resp.err().unwrap());
            rocket::response::status::Custom(rocket::http::Status::InternalServerError, String::from("Error creating user"))
        },
    }
}

#[post("/users/login", data = "<data>")]
pub fn login(data: String) -> rocket::response::status::Custom<String> {
    let data = serde_json::from_str::<serde_json::Value>(&data).expect("Not a valid JSON object");

    let email: &str = data.get("email").expect("The email is required").as_str().expect("Username must be a string");
    let password = data.get("password").expect("The password is required").as_str().expect("Password must be a string");

    let resp = user_login(email, password);

    match resp {
        Ok(_) => rocket::response::status::Custom(rocket::http::Status::Ok, format!("User created successfully: {:?}", resp)),
        Err(_) => {
            log::error!("Error creating user {}", resp.err().unwrap());
            rocket::response::status::Custom(rocket::http::Status::InternalServerError, String::from("Error creating user"))
        },
    }
}