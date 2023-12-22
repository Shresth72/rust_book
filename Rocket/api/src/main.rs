/*
    - Starting Rocket
    - Mounting routes
    - Variables including optional query variables
    - Redirects
    - URI Prefixes
    - JSON responses
    - Error handling
    - Returning Two Responses
*/


#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value};
use reqwest::{self, Client};

const TAURI_RELEASES_PREFIX: Origin<'static> = uri!("/tauri-releases");
const GOOGLE_KEEP_DESKTOP_REPO: &str = "elibroftw/google-keep-desktop-app";

#[get("/")]
fn index() -> Redirect {
    let msg: Option<String> = None;
    Redirect::to(uri!(TAURI_RELEASES_PREFIX, google_keep_desktop("linux".to_string(), "0.1.0".to_string(), msg)))
}

// if you plan to perform multiple requests to the same host, consider creating a Client instance and reusing it
async fn get_latest_release (client: &Client, repo: &str) -> Result<Value, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");

    let resp = client.get(&url).send().await?;

    let github_release = resp.json::<Value>().await?;
    Ok(github_release)
}

#[get("/google-keep-desktop/<_platform>/<current_version>?<msg>")]
fn google_keep_desktop(_platform: String, current_version: String, msg: Option<String>) -> Result<Value, Status> {
    // format!("Hello, {} year old named {}!", name, age)
    // Status::NoContent

    if let Some(msg) = msg {
        println!("{msg}");
        return Err(Status::NoContent);
    }

    Ok(json!({
        "current_version": current_version,
        "msg": msg
    }))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/tauri-releases", routes![google_keep_desktop])
}

// M