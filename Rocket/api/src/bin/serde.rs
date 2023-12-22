use std::borrow::Cow;
use reqwest::Client;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value, Json};
use rocket::response::status::NotFound;
use serde::{Deserialize, Serialize};
use rocket::{Route, State};

pub const BASE: Origin<'static> = uri!("/serde");

#derive