#![allow(unused_variables)]
#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::content::Html;
use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::json;

enum Variant {
    Standard,
    TimeMachine,
}

enum UserType {
    UserId(i32),
    Username(String),
    Nickname(String),
    Avatar(String),
}

enum AdminLogCategory {
    Post,
    User,
    Bawu,
}

#[derive(Serialize, Deserialize)]
struct User {
    user_id: i32,
    username: String,
    nickname: String,
    avatar: String,
}

#[derive(Serialize, Deserialize)]
struct Content {
    _type: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Thread {
    thread_id: i32,
    title: String,
    user_id: i32,
    reply_num: i32,
    is_good: bool,
}

#[derive(Serialize, Deserialize)]
struct Post {
    post_id: i32,
    floor: i32,
    user_id: i32,
    content: Vec<Content>,
    time: String,
    comment_num: i32,
    signature: String,
    tail: String,
}

#[derive(Serialize, Deserialize)]
struct Comment {
    comment_id: i32,
    user_id: i32,
    content: Vec<Content>,
    time: String,
    post_id: i32,
}

#[derive(Serialize, Deserialize)]
struct ApiRequest {
    request_type: String,
    variant: Option<String>,
    page: i32,
    thread_id: Option<i32>,
    post_id: Option<i32>,
    admin_log_category: Option<String>,
    admin_log_hide_the_showdown: Option<bool>,
    user_id: Option<i32>,
    username: Option<String>,
    nickname: Option<String>,
    avatar: Option<String>,
}

fn respond_thread(variant: Variant, page: i32) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    Ok(Json(json!({})))
}

fn respond_post(
    variant: Variant,
    thread_id: i32,
    page: i32,
) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    Ok(Json(json!({})))
}

fn respond_comment(
    variant: Variant,
    post_id: i32,
    page: i32,
) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    Ok(Json(json!({})))
}

fn respond_user(user_type: UserType) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    Ok(Json(json!({})))
}

fn response_admin_log(
    admin_log_category: AdminLogCategory,
    admin_log_hide_the_showdown: bool,
    page: i32,
) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    Ok(Json(json!({})))
}

#[get("/")]
fn rickroll() -> Html<&'static str> {
    Html("<!doctype html><meta name='referrer' content='no-referrer'><meta http-equiv='refresh' content='0; URL=https://www.bilibili.com/video/av202867917'>")
}

#[post("/", format = "json", data = "<api_request>")]
fn request_dispatcher(api_request: Json<ApiRequest>) -> Result<Json<serde_json::Value>, Status> {
    let request_type = api_request.request_type.as_str();
    let page = api_request.page;
    match request_type {
        "thread" => {
            if let Some(variant) = &api_request.variant {
                match variant.as_str() {
                    "standard" => respond_thread(Variant::Standard, page),
                    "time_machine" => respond_thread(Variant::TimeMachine, page),
                    _ => Err(Status::UnprocessableEntity),
                }
            } else {
                Err(Status::UnprocessableEntity)
            }
        }
        "post" => {
            if let (Some(variant), Some(thread_id)) = (&api_request.variant, api_request.thread_id)
            {
                match variant.as_str() {
                    "standard" => respond_post(Variant::Standard, thread_id, page),
                    "time_machine" => respond_post(Variant::TimeMachine, thread_id, page),
                    _ => Err(Status::UnprocessableEntity),
                }
            } else {
                Err(Status::UnprocessableEntity)
            }
        }
        "comment" => {
            if let (Some(variant), Some(post_id)) = (&api_request.variant, api_request.post_id) {
                match variant.as_str() {
                    "standard" => respond_comment(Variant::Standard, post_id, page),
                    "time_machine" => respond_comment(Variant::TimeMachine, post_id, page),
                    _ => Err(Status::UnprocessableEntity),
                }
            } else {
                Err(Status::UnprocessableEntity)
            }
        }
        "user" => {
            // priority: user_id -> avatar -> username -> nickname
            if let Some(user_id) = api_request.user_id {
                respond_user(UserType::UserId(user_id))
            } else if let Some(avatar) = api_request.avatar.clone() {
                respond_user(UserType::Avatar(avatar))
            } else if let Some(username) = api_request.username.clone() {
                respond_user(UserType::Username(username))
            } else if let Some(nickname) = api_request.nickname.clone() {
                respond_user(UserType::Nickname(nickname))
            } else {
                Err(Status::UnprocessableEntity)
            }
        }
        "admin_log" => {
            if let (Some(admin_log_category), Some(admin_log_hide_the_showdown)) = (
                &api_request.admin_log_category,
                api_request.admin_log_hide_the_showdown,
            ) {
                match admin_log_category.as_str() {
                    "post" => response_admin_log(
                        AdminLogCategory::Post,
                        admin_log_hide_the_showdown,
                        page,
                    ),
                    "user" => response_admin_log(
                        AdminLogCategory::User,
                        admin_log_hide_the_showdown,
                        page,
                    ),
                    "bawu" => response_admin_log(
                        AdminLogCategory::Bawu,
                        admin_log_hide_the_showdown,
                        page,
                    ),
                    _ => Err(Status::UnprocessableEntity),
                }
            } else {
                Err(Status::UnprocessableEntity)
            }
        }
        _ => Err(Status::UnprocessableEntity),
    }
    //Ok(Json(json!({})))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![rickroll])
        .mount("/", routes![request_dispatcher])
}
