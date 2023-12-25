extern crate diesel;
#[macro_use] extern crate rocket;

use rocket::http::Status;
use diesel::pg::PgConnection;
use diesel::{
    // prelude::*,
    r2d2::{self, ConnectionManager},
};
mod db;
mod models;
mod routes;
mod schema;
mod server;
mod session;

#[get("/")]
fn index() -> Result<&'static str, Status> {
    Ok("Hello, world!")
}

// diesel
#[launch]
fn rocket() -> _ {
    let manager = ConnectionManager::<PgConnection>::new("postgres://postgres:postgres@localhost:5432/rocket");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    rocket::build()
        .manage(pool)
        .mount("/", routes![index])
}

