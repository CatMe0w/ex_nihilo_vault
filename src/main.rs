#![allow(unused_variables)]
#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::content::Html;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_sync_db_pools::rusqlite::params;
use rocket_sync_db_pools::{database, rusqlite};
use serde_json::json;

#[database("vault")]
struct Vault(rusqlite::Connection);

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
    content: serde_json::Value,
    time: String,
    comment_num: i32,
    signature: Option<String>,
    tail: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Comment {
    comment_id: i64,
    user_id: i64,
    content: serde_json::Value,
    time: String,
}

async fn get_thread_metadata(vault: &Vault, thread_id: i64) -> Option<Thread> {
    let thread = vault
        .run(move |c| {
            c.query_row(
                "SELECT title, user_id, reply_num, is_good FROM pr_thread WHERE id = ?",
                params![thread_id],
                |r| {
                    Ok(Thread {
                        thread_id,
                        title: r.get(0)?,
                        user_id: r.get(1)?,
                        reply_num: r.get(2)?,
                        is_good: r.get(3)?,
                    })
                },
            )
        })
        .await
        .ok()?;
    Some(thread)
}

async fn get_user_metadata(vault: &Vault, user_type: String, user_clue: String) -> Option<User> {
    let sql = match user_type.as_str() {
        "user_id" => "SELECT id, username, nickname, avatar FROM pr_user WHERE id = ?",
        "username" => "SELECT id, username, nickname, avatar FROM pr_user WHERE username = ?",
        "nickname" => "SELECT id, username, nickname, avatar FROM pr_user WHERE nickname = ?",
        "avatar" => "SELECT id, username, nickname, avatar FROM pr_user WHERE avatar = ?",
        _ => return None,
    };
    let user = vault
        .run(move |c| {
            c.query_row(sql, params![user_clue], |r| {
                Ok(User {
                    user_id: r.get(0)?,
                    username: r.get(1)?,
                    nickname: r.get(2)?,
                    avatar: r.get(3)?,
                })
            })
        })
        .await
        .ok()?;
    Some(user)
}

async fn get_post(vault: &Vault, thread_id: i64, page: i32) -> Result<Vec<Post>, rusqlite::Error> {
    let posts = vault
        .run(move |c| {
            c.prepare("SELECT * FROM pr_post WHERE thread_id = ? ORDER BY floor LIMIT ?,30")?
                .query_map(params![thread_id, (page - 1) * 30], |r| {
                    Ok(Post {
                        post_id: r.get(0)?,
                        floor: r.get(1)?,
                        user_id: r.get(2)?,
                        content: serde_json::from_str(r.get::<usize, String>(3)?.as_str()).unwrap(), // so ugly
                        time: r.get(4)?,
                        comment_num: r.get(5)?,
                        signature: r.get(6)?,
                        tail: r.get(7)?,
                    })
                })?
                .collect::<Result<Vec<Post>, _>>()
        })
        .await?;
    Ok(posts)
}

async fn get_comment(
    vault: &Vault,
    post_id: i64,
    page: i32,
    limit: i32,
) -> Result<Vec<Comment>, rusqlite::Error> {
    let comments = vault
        .run(move |c| {
            c.prepare("SELECT * FROM pr_comment WHERE post_id = ? ORDER BY time LIMIT ?,?")?
                .query_map(params![post_id, (page - 1) * 10, limit], |r| {
                    Ok(Comment {
                        comment_id: r.get(0)?,
                        user_id: r.get(1)?,
                        content: serde_json::from_str(r.get::<usize, String>(2)?.as_str()).unwrap(),
                        time: r.get(3)?,
                    })
                })?
                .collect::<Result<Vec<Comment>, _>>()
        })
        .await?;
    Ok(comments)
}

#[get("/thread/<page>?<time_machine_datetime>")]
async fn respond_thread(
    vault: Vault,
    page: i32,
    time_machine_datetime: Option<String>,
) -> Result<Json<serde_json::Value>, Status> {
    match time_machine_datetime {
        Some(time_machine_datetime) => {
            // TODO: implement
            let threads: Vec<Thread> = Vec::new();
            let users: Vec<User> = Vec::new();
            let admin_logs: Vec<AdminLog> = Vec::new(); // for standard variant
            Ok(Json(
                json!({"threads": threads, "users": users, "admin_logs": admin_logs}),
            ))
        }
        None => {
            let threads: Vec<Thread> = Vec::new();
            let users: Vec<User> = Vec::new();
            Ok(Json(json!({"threads": threads, "users": users})))
        }
    }
}

#[get("/post/<thread_id>/<page>?<time_machine_datetime>")]
async fn respond_post(
    vault: Vault,
    thread_id: i64,
    page: i32,
    time_machine_datetime: Option<String>,
) -> Result<Json<serde_json::Value>, Status> {
    // // TODO: implement
    // match variant.as_str() {
    //     "standard" => {
    //         // TODO: implement
    //         let threads: Vec<Thread> = Vec::new();
    //         let users: Vec<User> = Vec::new();
    //         let admin_logs: Vec<AdminLog> = Vec::new(); // for standard variant
    //         Ok(Json(
    //             json!({"threads": threads, "users": users, "admin_logs": admin_logs}),
    //         ))
    //     }
    //     "time_machine" => {
    //         let threads: Vec<Thread> = Vec::new();
    //         let users: Vec<User> = Vec::new();
    //         Ok(Json(json!({"threads": threads, "users": users})))
    //     }
    //     _ => Err(Status::UnprocessableEntity),
    // };
    let thread = match get_thread_metadata(&vault, thread_id).await {
        Some(thread) => thread,
        None => return Err(Status::NotFound),
    };
    let posts = get_post(&vault, thread_id, page).await.unwrap();
    let comments: Vec<Comment> = Vec::new();
    let users: Vec<User> = Vec::new();
    let admin_logs: Vec<AdminLog> = Vec::new(); // for standard variant
    Ok(Json(json!({
        "title": thread.title,
        "user_id": thread.user_id,
        "reply_num":thread.reply_num,
        "is_good": thread.is_good,
        "comments": comments,
        "users": users,
        "posts": posts,
        "admin_logs": admin_logs
    })))
}

#[get("/comment/<post_id>/<page>?<time_machine_datetime>")]
async fn respond_comment(
    vault: Vault,
    post_id: i64,
    page: i32,
    time_machine_datetime: Option<String>,
) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    let comments = get_comment(&vault, post_id, page, 10).await.unwrap();
    let users: Vec<User> = Vec::new();
    let admin_logs: Vec<AdminLog> = Vec::new(); // for standard variant
    Ok(Json(
        json!({"comments": comments, "users": users, "admin_logs": admin_logs}),
    ))
}

#[get("/user/<user_type>/<user_clue>/<page>?<time_machine_datetime>")]
async fn respond_user(
    vault: Vault,
    user_type: String,
    user_clue: String,
    page: i32,
    time_machine_datetime: Option<String>,
) -> Result<Json<serde_json::Value>, Status> {
    match get_user_metadata(&vault, user_type, user_clue).await {
        Some(user) => Ok(Json(json!({
            "user_id": user.user_id,
            "username": user.username,
            "nickname": user.nickname,
            "avatar": user.avatar,
        }))),
        None => Err(Status::NotFound),
    }
}

#[get("/admin_log/<category>/<page>?<hide_the_showdown>")]
async fn respond_admin_log(
    vault: Vault,
    category: String,
    hide_the_showdown: bool,
    page: i32,
) -> Result<Json<serde_json::Value>, Status> {
    // TODO: implement
    match category.as_str() {
        "post" => {
            let admin_logs: Vec<AdminLog> = Vec::new();
            Ok(Json(json!(admin_logs)))
        }
        "user" => {
            let admin_logs: Vec<AdminLog> = Vec::new();
            Ok(Json(json!(admin_logs)))
        }
        "bawu" => {
            let admin_logs: Vec<AdminLog> = Vec::new();
            Ok(Json(json!(admin_logs)))
        }
        _ => Err(Status::UnprocessableEntity),
    }
}

#[get("/")]
async fn rickroll() -> Html<&'static str> {
    Html("<!doctype html><meta name='referrer' content='no-referrer'><meta http-equiv='refresh' content='0; URL=https://www.bilibili.com/video/av202867917'>")
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Vault::fairing()).mount(
        "/",
        routes![
            rickroll,
            respond_thread,
            respond_post,
            respond_comment,
            respond_user,
            respond_admin_log
        ],
    )
}
