use std::borrow::Cow; // clone-on-write
use reqwest::Client;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value, Json};
use rocket::response::status::NotFound;
use serde::{Deserialize, Serialize};
use rocket::{Route, State};

#[macro_use]
extern crate rocket;

pub const BASE: Origin<'static> = uri!("/serde");

#[derive(Serialize, Deserialize, Clone)]
struct Name<'r> {
    first: Cow<'r, str>,
    last: Cow<'r, str>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Names<'r> (Vec<Name<'r>>);

pub fn routes() -> Vec<Route> {
    routes![get_json_example, post_json_example, json_value_example]
}

// Deserialize a JSON value from a string
#[get("/get-json-example")]
pub async fn get_json_example(client: &State<Client>) -> Result<Json<Names<'_>>, NotFound<String>> {
    let response = client.get("https://jsonplaceholder.typicode.com/users").send().await.map_err(|e| NotFound(e.to_string()))?;
    let mut names = response.json::<Names<'_>>().await.map_err(|e| NotFound(e.to_string()))?;
    
    // 0 is used because it's a struct with no names
    names.0.push(
        Name {
            first: "Rachel".into(),
            last: "Green".into(),
        }
    );
    Ok(Json(names))
}

#[post("/post-json-example", data = "<names>")]
pub fn post_json_example(mut names: Json<Names<'_>>) -> Json<Names<'_>> {
    names.0.0.push(
        Name {
            first: "Rachel".into(),
            last: "Green".into(),
        }
    );
    names
}

#[get("/value-example")]
pub async fn json_value_example() -> Value {
    // assume we are working with a 3rd party API and respond with a JSON
    // where u=you only care about one field. Deserializing it would be a waste of effort
    // therefore, it's better to just handle the errors (either use ? or let Some(x) = ...)
    let mut response: Value = json!({
        "names": [
            {
                "first": "Eli",
                "last": "B."
            }
        ],
    });

    // lets add a name
    // here I used unwrap because I already know the mapping
    // if you are working with arbitary json, make sure to handle errors
    response["names"].as_array_mut().unwrap().push(
        json!({
            "first": "Rachel",
            "last": "Green"
        })
    );

    // example mutating a hashmap (the value insert has to be of type Value::...)
    response.as_object_mut().unwrap().insert("irrelevant".to_string(), Value::String("value".to_string()));

    // return names
    response["names"].clone()
}

fn main() {
    // Add your code here
}

