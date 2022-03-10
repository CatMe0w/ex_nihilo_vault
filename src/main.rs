#![allow(unused_variables)]
#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::content::Html;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_sync_db_pools::{database, rusqlite};
use serde_json::json;

#[database("vault")]
struct Vault(rusqlite::Connection);

enum Variant {
    Standard,
    TimeMachine,
}

enum UserType {
    UserId(i64),
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
#[serde(untagged)]
enum AdminLog {
    Post {
        thread_id: i64,
        post_id: Option<i64>,
        title: String,
        content_preview: String,
        username: String,
        post_time: String,
        operation: String,
        operator: String,
        operation_time: String,
    },
    User {
        avatar: String,
        username: String,
        operation: String,
        duration: String,
        operator: String,
        operation_time: String,
    },
    Bawu {
        avatar: String,
        username: String,
        operation: String,
        operator: String,
        operation_time: String,
    },
}

#[derive(Serialize, Deserialize)]
struct User {
    user_id: i64,
    username: String,
    nickname: String,
    avatar: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum UserRecord {
    Thread {
        #[serde(rename = "type")]
        _type: String,
        content: Thread,
    },
    Post {
        #[serde(rename = "type")]
        _type: String,
        content: Post,
    },
    Comment {
        #[serde(rename = "type")]
        _type: String,
        content: Comment,
    },
}

#[derive(Serialize, Deserialize)]
struct ContentItem {
    #[serde(rename = "type")]
    _type: String,
    #[serde(flatten)]
    content: Content,
}

#[derive(Serialize, Deserialize)]
enum Content {
    Text(String),
    Emoticon { id: String, description: String },
    Username { text: String, user_id: i64 },
    Url { url: String, text: String },
    Image(String),
    Video(String),
    Audio(String),
}

#[derive(Serialize, Deserialize)]
struct Thread {
    thread_id: i64,
    title: String,
    user_id: i64,
    reply_num: i32,
    is_good: bool,
}

#[derive(Serialize, Deserialize)]
struct Post {
    post_id: i64,
    floor: i32,
    user_id: i64,
    content: Vec<ContentItem>,
    time: String,
    comment_num: i32,
    signature: String,
    tail: String,
}

#[derive(Serialize, Deserialize)]
struct Comment {
    comment_id: i64,
    user_id: i64,
    content: Vec<ContentItem>,
    time: String,
    post_id: i64,
}

#[derive(Serialize, Deserialize)]
struct ApiRequest {
    request_type: String,
    variant: Option<String>,
    page: i32,
    thread_id: Option<i64>,
    post_id: Option<i64>,
    admin_log_category: Option<String>,
    admin_log_hide_the_showdown: Option<bool>,
    user_id: Option<i64>,
    username: Option<String>,
    nickname: Option<String>,
    avatar: Option<String>,
}

async fn get_thread_metadata(thread_id: i64) -> (String, i64, i32, bool) {
    // TODO: implement
    let title: String = String::from("");
    let reply_num: i32 = 0;
    let is_good: bool = false;
    let user_id: i64 = 0;
    (title, user_id, reply_num, is_good)
}

async fn get_user_metadata(user_type: UserType) -> (i64, String, String, String) {
    // TODO: implement
    let user_id: i64 = 0;
    let username: String = String::from("");
    let nickname: String = String::from("");
    let avatar: String = String::from("");
    (user_id, username, nickname, avatar)
}

async fn respond_thread(variant: Variant, page: i32) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    let threads: Vec<Thread> = Vec::new();
    let users: Vec<User> = Vec::new();
    Ok(Json(json!({"threads": threads, "users": users})))
}

async fn respond_post(
    variant: Variant,
    thread_id: i64,
    page: i32,
) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    let (title, user_id, reply_num, is_good) = get_thread_metadata(thread_id).await;
    let posts: Vec<Post> = Vec::new();
    let comments: Vec<Comment> = Vec::new();
    let users: Vec<User> = Vec::new();
    Ok(Json(json!({
        "title": title,
        "user_id": user_id,
        "reply_num": reply_num,
        "is_good": is_good,
        "comments": comments,
        "users": users
    })))
}

async fn respond_comment(
    variant: Variant,
    post_id: i64,
    page: i32,
) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    let comments: Vec<Comment> = Vec::new();
    let users: Vec<User> = Vec::new();
    Ok(Json(json!({"comments": comments, "users": users})))
}

async fn respond_user(user_type: UserType) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    let (user_id, username, nickname, avatar) = get_user_metadata(user_type).await;
    let records: Vec<UserRecord> = Vec::new();
    Ok(Json(json!({
        "user_id": user_id,
        "username": username,
        "nickname": nickname,
        "avatar": avatar,
        "records": records
    })))
}

async fn respond_admin_log(
    admin_log_category: AdminLogCategory,
    admin_log_hide_the_showdown: bool,
    page: i32,
) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    match admin_log_category {
        AdminLogCategory::Post => {
            let admin_logs: Vec<AdminLog> = Vec::new();
            Ok(Json(json!(admin_logs)))
        }
        AdminLogCategory::User => {
            let admin_logs: Vec<AdminLog> = Vec::new();
            Ok(Json(json!(admin_logs)))
        }
        AdminLogCategory::Bawu => {
            let admin_logs: Vec<AdminLog> = Vec::new();
            Ok(Json(json!(admin_logs)))
        }
    }
}

#[get("/")]
async fn rickroll() -> Html<&'static str> {
    Html("<!doctype html><meta name='referrer' content='no-referrer'><meta http-equiv='refresh' content='0; URL=https://www.bilibili.com/video/av202867917'>")
}

#[post("/", format = "json", data = "<api_request>")]
async fn request_dispatcher(
    api_request: Json<ApiRequest>,
) -> Result<Json<serde_json::Value>, Status> {
    let request_type = api_request.request_type.as_str();
    let page = api_request.page;
    match request_type {
        "thread" => {
            if let Some(variant) = &api_request.variant {
                match variant.as_str() {
                    "standard" => respond_thread(Variant::Standard, page).await,
                    "time_machine" => respond_thread(Variant::TimeMachine, page).await,
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
                    "standard" => respond_post(Variant::Standard, thread_id, page).await,
                    "time_machine" => respond_post(Variant::TimeMachine, thread_id, page).await,
                    _ => Err(Status::UnprocessableEntity),
                }
            } else {
                Err(Status::UnprocessableEntity)
            }
        }
        "comment" => {
            if let (Some(variant), Some(post_id)) = (&api_request.variant, api_request.post_id) {
                match variant.as_str() {
                    "standard" => respond_comment(Variant::Standard, post_id, page).await,
                    "time_machine" => respond_comment(Variant::TimeMachine, post_id, page).await,
                    _ => Err(Status::UnprocessableEntity),
                }
            } else {
                Err(Status::UnprocessableEntity)
            }
        }
        "user" => {
            // priority: user_id -> avatar -> username -> nickname
            if let Some(user_id) = api_request.user_id {
                respond_user(UserType::UserId(user_id)).await
            } else if let Some(avatar) = api_request.avatar.clone() {
                respond_user(UserType::Avatar(avatar)).await
            } else if let Some(username) = api_request.username.clone() {
                respond_user(UserType::Username(username)).await
            } else if let Some(nickname) = api_request.nickname.clone() {
                respond_user(UserType::Nickname(nickname)).await
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
                    "post" => {
                        respond_admin_log(AdminLogCategory::Post, admin_log_hide_the_showdown, page)
                            .await
                    }
                    "user" => {
                        respond_admin_log(AdminLogCategory::User, admin_log_hide_the_showdown, page)
                            .await
                    }
                    "bawu" => {
                        respond_admin_log(AdminLogCategory::Bawu, admin_log_hide_the_showdown, page)
                            .await
                    }
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
        .attach(Vault::fairing())
        .mount("/", routes![rickroll, request_dispatcher])
}
