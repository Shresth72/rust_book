use rocket::outcome::Outcome::{*, self};
use rocket::request::{self, Request};
use rocket::http::Status;
use rocket::data::{Data, FromData, ToByteUnit};
use rocket::serde::json::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Object {
    pub title: String,
    pub text: String,
}

#[derive(Debug)]
pub enum MyError {
    TooLarge,
    NoBodyProvided,
    Io(std::io::Error)
}

// Payload Validation
// 'r is the lifetime of the request
#[rocket::async_trait]
impl<'r> FromData<'r> for Object {
    type Error = MyError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<Self, (Status, MyError), Data<'r>> {
        // Read the data into a String.
        let string_data = match data.open(10.kilobytes()).into_string().await {
            Ok(s) => s,
            Ok(_) => return Outcome::Failure((Status::BadRequest, MyError::TooLarge)),
            Err(e) => return Outcome::Failure((Status::InternalServerError, MyError::Io(e))),
        };

        let string_body = request::local_cache!(req, string_data);

        match serde_json::from_str(string_body) {
            Ok(value) => Outcome::Success(value),
            Err(e) => {
                println!("Failed to deserialize: {:?}", e);
                Outcome::Failure((Status::BadRequest, MyError::NoBodyProvided))
            }
        }
    }
}