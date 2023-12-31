use crate::{AppState. TokenClaims};
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate rocket;

#[post("/user")]
async fn create_user() -> &'static str {
    "Hello, world!"
}

#[get("/auth")]
async fn auth() -> &'static str {
    "Hello, world!"
}

#[post("/auth")]
async fn auth_post() -> &'static str {
    "Hello, world!"
}