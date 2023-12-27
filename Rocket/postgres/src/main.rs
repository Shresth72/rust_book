use rocket::http::Status;
use rocket::State;
use sea_orm::*;
use std::env::var;
use dotenv::dotenv;

mod entity;
mod models;

use entity::{prelude::*, *};
use models::object::Object;

#[macro_use]
extern crate rocket;

const POSTGRES_USER: &str = "539srijansiddharth";
const POSTGRES_DB: &str = "neondb";

#[get("/")]
async fn get_start(connection: &State<DatabaseConnection>) -> Result<&'static str, Status> {
    let connection = connection as &DatabaseConnection;
    let _result = Post::find().all(connection).await.unwrap();

    Ok("Records retrieved!")
}

#[post("/start", format = "application/json", data = "<object>")]
async fn post_start(connection: &State<DatabaseConnection>, object: Object) -> Result<&'static str, Status> {
    let connection = connection as &DatabaseConnection;
    let record = post::ActiveModel {
        title: Set(object.title.to_owned()),
        text: Set(object.text.to_owned()),
        ..Default::default()
    };

    let _result = Post::insert(record.clone()).exec(connection).await.unwrap();

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
