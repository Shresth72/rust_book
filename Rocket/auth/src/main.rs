use rocket::http::Status;
use std::env::var;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use rocker::Request;

use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use rocket_codegen::{get, post};

mod services;
use services::{basic_auth, create_article, create_user};

pub struct AppState {
    db: Pool<Postgres>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    id: i32,
}

async fn validator(req: Request<'_>, credentials: B)

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&var("DATABASE_URL").unwrap())
        .await {
            Ok(pool) => pool,
            Err(e) => {
                println!("Error connecting to database: {}", e);
                panic!("Error connecting to database, exiting.");
            }
        };

    rocket::build()
        .manage(pool)
        .mount("/", routes![index])
}
