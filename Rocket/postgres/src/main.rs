#[macro_use]
extern crate rocket;

use rocket::http::Status;

#[get("/")]
fn index() -> Result<&'static str, Status> {
    Ok("Hello, world!")
}

#[launch]
fn rocket() -> _ {
    
    rocket::build().mount("/", routes![index])
}
