use rocket::post;
use super::super::super::db::queries::user_create;

pub fn test() {
    let path = std::path::Path::new("test");
   
}



#[post("/users/create", data = "<data>")]
pub fn create_user(data: String) -> rocket::response::status::Custom<String> {
    let data = serde_json::from_str::<serde_json::Value>(&data).expect("Not a valid JSON object");

    let username = data.get("username").expect("The username is required").as_str().expect("Username must be a string");
    let password = data.get("password").expect("The password is required").as_str().expect("Password must be a string");
    
    let resp = user_create(username, password);

    match resp {
        Ok(_) => rocket::response::status::Custom(rocket::http::Status::Ok, String::from("User created successfully")),
        Err(_) => rocket::response::status::Custom(rocket::http::Status::InternalServerError, String::from("Error creating user")),
    }
}