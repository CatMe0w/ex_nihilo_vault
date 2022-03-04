#[macro_use]
extern crate rocket;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[get("/")]
fn rickroll() -> Redirect {
    Redirect::to("https://www.bilibili.com/video/BV1va411w7aM")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![rickroll])
}
