use rocket::http::Status;
use rocket::State;
use sea_orm::*;
use std::env::var;
use dotenv::dotenv;

mod entity;
use entity::{prelude::*, *};

#[macro_use]
extern crate rocket;

const POSTGRES_USER: &str = "539srijansiddharth";
const POSTGRES_DB: &str = "neondb";

#[get("/")]
async fn get_start(connection: &State<DatabaseConnection>) -> Result<&'static str, Status> {
    let connection = connection as &DatabaseConnection;
    let _result = Post::find().all(connection).await;

    Ok("Records retrieved!")
}

#[post("/start")]
async fn post_start(connection: &State<DatabaseConnection>) -> Result<&'static str, Status> {
    let connection = connection as &DatabaseConnection;
    let record = post::ActiveModel {
        title: Set("Title".to_owned()),
        text: Set("Some Text".to_owned()),
        ..Default::default()
    };

    let _result = Post::insert(record.clone()).exec(connection).await;

    Ok("Record created!")
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let connection = match Database::connect(format!(
        "postgresql://{}:{}@ep-patient-feather-a189osl1.ap-southeast-1.aws.neon.tech/{}?sslmode=require",
        POSTGRES_USER,
        var("POSTGRES_PASSWORD").unwrap(),
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
