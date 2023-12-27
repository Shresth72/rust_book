/*
    - Starting Rocket
    - Mounting routes
    - Variables including optional query variables
    - Redirects
    - URI Prefixes
    - JSON responses
    - Error handling
    - Returning Two Responses

    - Using Client to make requests
    - Using States to share data between requests

    - JSON manipulation

    - Caching
*/


#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value};
use rocket::State;
use reqwest::{self, Client};
use std::collections::HashMap;

const TAURI_RELEASES_PREFIX: Origin<'static> = uri!("/tauri-releases");
const GOOGLE_KEEP_DESKTOP_REPO: &str = "elibroftw/google-keep-desktop-app";

#[get("/")]
fn index() -> Redirect {
    let msg: Option<String> = None;
    Redirect::to(uri!(TAURI_RELEASES_PREFIX, google_keep_desktop("linux".to_string(), "0.1.0".to_string(), msg)))
}

async fn text_request(client: &State<Client>, url: &str) -> Result<String, reqwest::Error> {
    client.get(url).send().await?.text().await
}

async fn create_tauri_response(client: &State<Client>,github_release: &Value) -> Option<Value> {
    let platforms_available: HashMap<&str, Vec<&str>> = HashMap::from([
        ("amd64.AppImage.tar.gz", vec!["linux-x86_64"]),
        ("app.tar.gz", vec!["darwin-x86_64", "darwin-aarch64"]),
        ("x64_en-US.msi.zip", vec!["windows-z86_64"]),
    ]);

    let mut response = json!({
        "version": github_release["tag_name"].as_str()?,
        "notes": github_release["body"].as_str()?,
        "pub_date": github_release["published_at"].as_str()?,
        "platforms": {}
    });

    let response_platforms = response["platforms"].as_object_mut()?;
    for asset in github_release["assets"].as_array()?.iter() {
        let asset = asset.as_object()?;
        let asset_name = asset["name"].as_str()?;
        let browser_download_url = asset["browser_download_url"].as_str()?;

        for (extension, os_archs) in platforms_available.iter() {
            if asset_name.ends_with(extension) {
                for os_arch in os_archs.iter() {
                    if !response_platforms.contains_key(*os_arch) {
                        response_platforms.insert(os_arch.to_string(), json!({}));
                    }
                    response_platforms[*os_arch].as_object_mut()?.insert("url".to_string(), Value::String(browser_download_url.to_string()));
                }

            } else if asset_name.ends_with(&format!("{extension}.sig")) {
                let signature = match text_request(client, browser_download_url).await {
                    Ok(s) => s,
                    _ => String::new()
                };
                for os_arch in os_archs.iter() {
                    if !response_platforms.contains_key(*os_arch) {
                        response_platforms.insert(os_arch.to_string(), json!({}));
                    }
                    response_platforms[*os_arch].as_object_mut()?.insert("signature".to_string(), Value::String(signature.clone()));
                }
            }
        }
    }

    Some(response)
}

// if you plan to perform multiple requests to the same host, consider creating a Client instance and reusing it
async fn get_latest_release (client: &State<Client>, repo: &str) -> Result<Value, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");

    let resp = client.get(&url).send().await?;

    // deserialize the response
    let github_release = resp.json::<Value>().await?;

    // ok_or -> Transform Option to Result, mapping Some to Ok, and None to Err
    // or_else -> Calls another function if the Result is an Err, otherwise returns the Ok value
    create_tauri_response(client, &github_release).await.ok_or(json!({})).or_else(|e| Ok(e))
}

#[get("/google-keep-desktop/<_platform>/<_current_version>?<msg>")]
async fn google_keep_desktop(_platform: String, _current_version: String, msg: Option<String>, client: &State<Client>) -> Result<Value, Status> {
    // format!("Hello, {} year old named {}!", name, age)
    // Status::NoContent

    if let Some(msg) = msg {
        println!("{msg}");
        return Err(Status::NoContent);
    }

    get_latest_release(client, GOOGLE_KEEP_DESKTOP_REPO).await.or(Err(Status::NoContent))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(
            reqwest::Client::builder()
                .user_agent("reqwest")
                .build()
                .unwrap(),
        )
        .mount("/", routes![index])
        .mount("/tauri-releases", routes![google_keep_desktop])
}
