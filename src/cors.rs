use rocket::{Request, Response, fairing::{Fairing, Info, Kind}, http::Header};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add comprehensive CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, PATCH, DELETE, OPTIONS, HEAD",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Authorization, Content-Type, Accept, Origin, X-Requested-With",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Access-Control-Max-Age", "86400"));
    }
}

#[options("/<_..>")]
pub fn cors_preflight() -> &'static str {
    ""
}