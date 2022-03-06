#[macro_use]
extern crate rocket;
use rocket::response::content::Html;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[get("/")]
fn rickroll() -> Html<&'static str> {
    Html("<!doctype html><meta name='referrer' content='no-referrer'><meta http-equiv='refresh' content='0; URL=https://www.bilibili.com/video/av202867917'>")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![rickroll])
}
