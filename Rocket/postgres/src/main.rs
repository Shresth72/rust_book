use rocket::http::Status;
use rocket::State;
use sea_orm::*;

// mod entities;
// use entities::{prelude::*, *};

#[macro_use]
extern crate rocket;

const POSTGRES_USER: &str = "rocket_db";
const POSTGRES_DB: &str = "postgresdb";
const PORT: &str = "5432";

#[get("/")]
fn get_start(connection: &State<DatabaseConnection>) -> Result<&'static str, Status> {
    Ok("Hello, world!")
}

#[post("/start")]
fn post_start(connection: &State<DatabaseConnection>) -> Result<&'static str, Status> {
    let record = post::ActiveModel {
        title: Set("Title".to_owned()),
        text: Set("Some Text".to_owned()),
        ..Default::default()
    };

    let result = Post::insert(record.clone()).exec(&**connection);

    Ok("Record created!")
}

#[launch]
async fn rocket() -> _ {
    let connection = match Database::connect(format!(
        "postgres://{}:password@localhost:{}/{}",
        POSTGRES_USER,
        PORT,
        POSTGRES_DB
    )).await {
        Ok(connection) => connection,
        Err(e) => {
            println!("Error connecting to database: {}", e);
            panic!("Error connecting to database, exiting.");
        }
    };

    rocket::build()
        .manage(connection)
        .mount("/", routes![get_start, post_start])
}
