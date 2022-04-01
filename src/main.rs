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

enum UserType {
    UserId,
    Username,
    Nickname,
    Avatar,
}

#[derive(Serialize, Deserialize)]
struct User {
    user_id: i64,
    username: Option<String>,
    nickname: String,
    avatar: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum UserRecord {
    Post {
        #[serde(rename = "type")]
        _type: String,
        thread_id: i64,
        title: String,
        post_id: i64,
        floor: i32,
        post_content: serde_json::Value,
        time: String,
    },
    Comment {
        #[serde(rename = "type")]
        _type: String,
        thread_id: i64,
        title: String,
        post_id: i64,
        floor: i32,
        post_content: serde_json::Value,
        comment_id: i64,
        comment_content: serde_json::Value,
        time: String,
    },
}

enum AdminLogCategory {
    User,
    Post,
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
        media: Option<String>,
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
        operator: Option<String>,
        operation_time: String,
    },
}

struct ThreadMetadata {
    title: String,
    user_id: i64,
    reply_num: i32,
    is_good: bool,
}

#[derive(Serialize, Deserialize)]
struct Thread {
    thread_id: i64,
    op_user_id: i64, // op: original poster, aka. floor == 1
    title: String,
    user_id: i64,
    time: String,
    reply_num: i32,
    is_good: bool,
    op_post_content: serde_json::Value,
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

async fn get_thread_metadata(vault: &Vault, thread_id: i64) -> Option<ThreadMetadata> {
    let thread_metadata = vault
        .run(move |c| {
            c.query_row(
                "SELECT title, user_id, reply_num, is_good FROM pr_thread WHERE id = ?",
                params![thread_id],
                |r| {
                    Ok(ThreadMetadata {
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
    Some(thread_metadata)
}

async fn get_user_metadata(vault: &Vault, user_type: UserType, user_clue: String) -> Option<User> {
    let sql = match user_type {
        UserType::UserId => "SELECT * FROM pr_user WHERE id = ?",
        UserType::Username => "SELECT * FROM pr_user WHERE username = ?",
        UserType::Nickname => "SELECT * FROM pr_user WHERE nickname = ?",
        UserType::Avatar => "SELECT * FROM pr_user WHERE avatar = ?",
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

fn get_datetime_sql_param(datetime: Option<String>) -> String {
    match datetime {
        Some(datetime) => datetime,
        None => "9999-12-31 23:59:59".to_string(), // so ugly
    }
}

async fn get_threads(
    vault: &Vault,
    page: u32,
    time_machine_datetime: Option<String>,
) -> Result<Vec<Thread>, rusqlite::Error> {
    let sql = match &time_machine_datetime {
        None => "SELECT x.thread_id, t.user_id, title, x.user_id, x.time, reply_num, is_good, p.content FROM (
            SELECT * FROM (
                SELECT * FROM (
                    SELECT thread_id, user_id, time
                    FROM pr_post
                    WHERE time < ?2
                    ORDER BY time DESC
                )
                GROUP BY thread_id
                UNION
                SELECT * FROM (
                    SELECT thread_id, pr_comment.user_id, pr_comment.time
                    FROM pr_comment
                    JOIN pr_post ON pr_comment.post_id = pr_post.id
                    WHERE pr_comment.time < ?3
                    ORDER BY pr_comment.time DESC
                )
                GROUP BY thread_id
                ORDER BY time DESC
            )
            GROUP BY thread_id
            ORDER BY time DESC
            LIMIT ?4,50
        ) AS x
        JOIN pr_thread AS t ON x.thread_id = t.id
        JOIN pr_post AS p ON x.thread_id = p.thread_id AND p.floor = 1
        ORDER BY x.time DESC", // feel the pain: this monster takes ~80 ms to execute and eats a lot of cpu, use proxy_cache to mitigate
        Some(_) => "SELECT x.thread_id, t.user_id, title, x.user_id, x.time, reply_num, is_good, p.content,operation FROM (
            SELECT * FROM (
                SELECT y.*,operation FROM (
                    SELECT * FROM (
                        SELECT * FROM (
                            SELECT thread_id, user_id, time
                            FROM pr_post
                            WHERE time < ?1
                            ORDER BY time DESC
                        )
                        GROUP BY thread_id
                        UNION
                        SELECT * FROM (
                            SELECT thread_id, pr_comment.user_id, pr_comment.time
                            FROM pr_comment
                            JOIN pr_post ON pr_comment.post_id = pr_post.id
                            WHERE pr_comment.time < ?2
                            ORDER BY pr_comment.time DESC
                        )
                        GROUP BY thread_id
                        ORDER BY time DESC
                    )
                    GROUP BY thread_id
                    ORDER BY time DESC
                ) AS y 
                LEFT JOIN un_post AS u ON u.thread_id = y.thread_id AND u.post_id IS NULL AND u.operation LIKE '%删贴' AND operation_time < ?3 AND operation_time NOT LIKE '2022-02-26 23:%' AND operation_time NOT LIKE '2022-02-16 01:%' 
                GROUP BY y.thread_id
            )
            WHERE operation IS NULL OR operation <> '删贴'
            ORDER BY time DESC
			LIMIT ?4,50
        ) AS x
        JOIN pr_thread AS t ON x.thread_id = t.id
        JOIN pr_post AS p ON x.thread_id = p.thread_id AND p.floor = 1
        ORDER BY x.time DESC" // this one takes 90 ms, fuck
    };
    let datetime = get_datetime_sql_param(time_machine_datetime);
    let threads = vault
        .run(move |c| {
            c.prepare(sql)?
                .query_map(
                    params![datetime, datetime, datetime, (page - 1) * 50],
                    |r| {
                        Ok(Thread {
                            thread_id: r.get(0)?,
                            op_user_id: r.get(1)?,
                            title: r.get(2)?,
                            user_id: r.get(3)?,
                            time: r.get(4)?,
                            reply_num: r.get(5)?,
                            is_good: r.get(6)?,
                            op_post_content: serde_json::from_str(
                                r.get::<usize, String>(7)?.as_str(),
                            )
                            .unwrap(),
                        })
                    },
                )?
                .collect::<Result<Vec<Thread>, _>>()
        })
        .await?;
    Ok(threads)
}

async fn get_posts(
    vault: &Vault,
    thread_id: i64,
    page: u32,
    time_machine_datetime: Option<String>,
) -> Result<Vec<Post>, rusqlite::Error> {
    let datetime = get_datetime_sql_param(time_machine_datetime);
    let posts = vault
        .run(move |c| {
            c.prepare(
                "SELECT * FROM pr_post WHERE thread_id = ? AND time < ? ORDER BY floor LIMIT ?,30",
            )?
            .query_map(params![thread_id, datetime, (page - 1) * 30], |r| {
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

async fn get_comments(
    vault: &Vault,
    post_id: i64,
    page: u32,
    time_machine_datetime: Option<String>,
) -> Result<Vec<Comment>, rusqlite::Error> {
    let datetime = get_datetime_sql_param(time_machine_datetime);
    let comments = vault
        .run(move |c| {
            c.prepare(
                "SELECT * FROM pr_comment WHERE post_id = ? AND time < ? ORDER BY time LIMIT ?,10",
            )?
            .query_map(params![post_id, datetime, (page - 1) * 10], |r| {
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

async fn get_post_related_admin_logs(
    vault: &Vault,
    thread_id: Option<i64>,
    post_id: Option<i64>,
    time_machine_datetime: Option<String>,
) -> Result<Vec<AdminLog>, rusqlite::Error> {
    let datetime = get_datetime_sql_param(time_machine_datetime);
    let sql = match thread_id {
        Some(_) => "SELECT * FROM un_post WHERE thread_id = ? AND operation_time < ? AND operation_time NOT LIKE '2022-02-26 23:%' AND operation_time NOT LIKE '2022-02-16 01:%'",
        None => "SELECT * FROM un_post WHERE post_id = ? AND operation_time < ? AND operation_time NOT LIKE '2022-02-26 23:%' AND operation_time NOT LIKE '2022-02-16 01:%'",
    };
    // 2022-02-16 01:XX -> rewinder
    // 2022-02-26 23:XX -> rollwinder
    let id_param = match thread_id {
        Some(_) => thread_id,
        None => post_id,
    };
    let admin_logs = vault
        .run(move |c| {
            c.prepare(sql)?
                .query_map(params![id_param, datetime], |r| {
                    Ok(AdminLog::Post {
                        thread_id: r.get(0)?,
                        post_id: r.get(1)?,
                        title: r.get(2)?,
                        content_preview: r.get(3)?,
                        media: r.get(4)?,
                        username: r.get(5)?,
                        post_time: r.get(6)?,
                        operation: r.get(7)?,
                        operator: r.get(8)?,
                        operation_time: r.get(9)?,
                    })
                })?
                .collect::<Result<Vec<AdminLog>, _>>()
        })
        .await?;
    Ok(admin_logs)
}

async fn get_user_records(
    vault: &Vault,
    user_id: i64,
    page: u32,
    time_machine_datetime: Option<String>,
) -> Result<Vec<UserRecord>, rusqlite::Error> {
    let datetime = get_datetime_sql_param(time_machine_datetime);
    let user_records = vault
        .run(move |c| {
            c.prepare(
                "SELECT thread_id, pr_thread.title AS title, pr_post.id AS post_id, floor, content AS post_content, NULL AS comment_id, NULL AS comment_content, time
                     FROM pr_post
                     JOIN pr_thread
                     ON pr_post.thread_id = pr_thread.id
                     WHERE pr_post.user_id = ?
                     AND pr_post.time < ?
                     UNION
                     SELECT pr_thread.id, pr_thread.title, post_id, pr_post.floor, pr_post.content, pr_comment.id, pr_comment.content, pr_comment.time
                     FROM pr_comment
                     JOIN pr_post
                     ON pr_comment.post_id = pr_post.id
                     JOIN pr_thread
                     ON pr_post.thread_id = pr_thread.id
                     WHERE pr_comment.user_id = ?
                     AND pr_comment.time < ?
                     ORDER BY time DESC
                     LIMIT ?,50",
            )? // won't use sql next time
            .query_map(params![user_id, datetime, user_id, datetime, (page - 1) * 50], |r| {
                match r.get::<usize, Option<i64>>(5)? {
                    None => Ok(UserRecord::Post {
                        _type: "post".to_string(),
                        thread_id: r.get(0)?,
                        title: r.get(1)?,
                        post_id: r.get(2)?,
                        floor: r.get(3)?,
                        post_content: serde_json::from_str(r.get::<usize, String>(4)?.as_str()).unwrap(),
                        time: r.get(7)?,
                    }),
                    Some(_) => Ok(UserRecord::Comment {
                        _type: "comment".to_string(),
                        thread_id: r.get(0)?,
                        title: r.get(1)?,
                        post_id: r.get(2)?,
                        floor: r.get(3)?,
                        post_content: serde_json::from_str(r.get::<usize, String>(4)?.as_str()).unwrap(),
                        comment_id: r.get(5)?,
                        comment_content: serde_json::from_str(r.get::<usize, String>(6)?.as_str()).unwrap(),
                        time: r.get(7)?,
                    }),
                }
            })?
            .collect::<Result<Vec<UserRecord>, _>>()
        })
        .await?;
    Ok(user_records)
}

async fn get_admin_logs(
    vault: &Vault,
    category: AdminLogCategory,
    page: u32,
    hide_the_showdown: bool,
) -> Result<Vec<AdminLog>, rusqlite::Error> {
    let admin_logs = match category {
        AdminLogCategory::Post => {
            let sql = match hide_the_showdown {
                true => "SELECT * FROM un_post WHERE operation_time NOT LIKE '2022-02-26 23:%' AND operation_time NOT LIKE '2022-02-16 01:%' LIMIT ?,50",
                false => "SELECT * FROM un_post LIMIT ?,50",
            };
            vault
                .run(move |c| {
                    c.prepare(sql)?
                        .query_map(params![(page - 1) * 50], |r| {
                            Ok(AdminLog::Post {
                                thread_id: r.get(0)?,
                                post_id: r.get(1)?,
                                title: r.get(2)?,
                                content_preview: r.get(3)?,
                                media: r.get(4)?,
                                username: r.get(5)?,
                                post_time: r.get(6)?,
                                operation: r.get(7)?,
                                operator: r.get(8)?,
                                operation_time: r.get(9)?,
                            })
                        })?
                        .collect::<Result<Vec<AdminLog>, _>>()
                })
                .await?
        }
        AdminLogCategory::User => {
            vault
                .run(move |c| {
                    c.prepare("SELECT * FROM un_user LIMIT ?,50")?
                        .query_map(params![(page - 1) * 50], |r| {
                            Ok(AdminLog::User {
                                avatar: r.get(0)?,
                                username: r.get(1)?,
                                operation: r.get(2)?,
                                duration: r.get(3)?,
                                operator: r.get(4)?,
                                operation_time: r.get(5)?,
                            })
                        })?
                        .collect::<Result<Vec<AdminLog>, _>>()
                })
                .await?
        }
        AdminLogCategory::Bawu => {
            vault
                .run(move |c| {
                    c.prepare("SELECT * FROM un_bawu LIMIT ?,50")?
                        .query_map(params![(page - 1) * 50], |r| {
                            Ok(AdminLog::Bawu {
                                avatar: r.get(0)?,
                                username: r.get(1)?,
                                operation: r.get(2)?,
                                operator: r.get(3)?,
                                operation_time: r.get(4)?,
                            })
                        })?
                        .collect::<Result<Vec<AdminLog>, _>>()
                })
                .await?
        }
    };
    Ok(admin_logs)
}

#[get("/thread/<page>?<time_machine_datetime>")]
async fn respond_thread(
    vault: Vault,
    page: u32,
    time_machine_datetime: Option<String>,
) -> Result<Json<serde_json::Value>, Status> {
    let threads = get_threads(&vault, page, time_machine_datetime.clone())
        .await
        .unwrap();

    let mut op_users: Vec<User> = Vec::new();
    for thread in &threads {
        op_users.push(
            get_user_metadata(&vault, UserType::UserId, thread.op_user_id.to_string())
                .await
                .unwrap(),
        );
    }

    let mut last_reply_users: Vec<User> = Vec::new();
    for thread in &threads {
        last_reply_users.push(
            get_user_metadata(&vault, UserType::UserId, thread.user_id.to_string())
                .await
                .unwrap(),
        );
    }

    Ok(Json(
        json!({"threads": threads, "op_users": op_users, "last_reply_users": last_reply_users}),
    ))
}

#[get("/post/<thread_id>/<page>?<time_machine_datetime>")]
async fn respond_post(
    vault: Vault,
    thread_id: i64,
    page: u32,
    time_machine_datetime: Option<String>,
) -> Result<Json<serde_json::Value>, Status> {
    let thread = match get_thread_metadata(&vault, thread_id).await {
        Some(thread) => thread,
        None => return Err(Status::NotFound),
    };

    let posts = get_posts(&vault, thread_id, page, time_machine_datetime.clone())
        .await
        .unwrap();

    let mut comments: Vec<Vec<Comment>> = Vec::new();
    for post in &posts {
        comments.push(
            get_comments(&vault, post.post_id, 1, time_machine_datetime.clone())
                .await
                .unwrap(),
        );
    }

    let mut users: Vec<User> = Vec::new();
    for post in &posts {
        users.push(
            get_user_metadata(&vault, UserType::UserId, post.user_id.to_string())
                .await
                .unwrap(),
        );
    }

    let admin_logs: Vec<AdminLog> =
        get_post_related_admin_logs(&vault, Some(thread_id), None, time_machine_datetime)
            .await
            .unwrap();

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
    page: u32,
    time_machine_datetime: Option<String>,
) -> Result<Json<serde_json::Value>, Status> {
    let comments = get_comments(&vault, post_id, page, time_machine_datetime.clone())
        .await
        .unwrap();

    let mut users: Vec<User> = Vec::new();
    for comment in &comments {
        users.push(
            get_user_metadata(&vault, UserType::UserId, comment.user_id.to_string())
                .await
                .unwrap(),
        );
    }

    let admin_logs: Vec<AdminLog> =
        get_post_related_admin_logs(&vault, None, Some(post_id), time_machine_datetime)
            .await
            .unwrap();

    Ok(Json(
        json!({"comments": comments, "users": users, "admin_logs": admin_logs}),
    ))
}

#[get("/user/<user_type>/<user_clue>/<page>?<time_machine_datetime>")]
async fn respond_user(
    vault: Vault,
    user_type: String,
    user_clue: String,
    page: u32,
    time_machine_datetime: Option<String>,
) -> Result<Json<serde_json::Value>, Status> {
    let user_type = match user_type.as_str() {
        "user_id" => UserType::UserId,
        "username" => UserType::Username,
        "nickname" => UserType::Nickname,
        "avatar" => UserType::Avatar,
        _ => return Err(Status::NotFound),
    };
    match get_user_metadata(&vault, user_type, user_clue).await {
        Some(user) => {
            let records = get_user_records(&vault, user.user_id, page, time_machine_datetime)
                .await
                .unwrap();
            Ok(Json(json!({
                "user_id": user.user_id,
                "username": user.username,
                "nickname": user.nickname,
                "avatar": user.avatar,
                "records": records,
            })))
        }
        None => Err(Status::NotFound),
    }
}

#[get("/admin_log/<category>/<page>?<hide_the_showdown>")]
async fn respond_admin_log(
    vault: Vault,
    category: String,
    page: u32,
    hide_the_showdown: bool,
) -> Result<Json<serde_json::Value>, Status> {
    match category.as_str() {
        "post" => {
            let admin_logs =
                get_admin_logs(&vault, AdminLogCategory::Post, page, hide_the_showdown)
                    .await
                    .unwrap();
            Ok(Json(json!(admin_logs)))
        }
        "user" => {
            let admin_logs =
                get_admin_logs(&vault, AdminLogCategory::User, page, hide_the_showdown)
                    .await
                    .unwrap();
            Ok(Json(json!(admin_logs)))
        }
        "bawu" => {
            let admin_logs =
                get_admin_logs(&vault, AdminLogCategory::Bawu, page, hide_the_showdown)
                    .await
                    .unwrap();
            Ok(Json(json!(admin_logs)))
        }
        _ => Err(Status::NotFound),
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
            respond_thread,
            respond_post,
            respond_comment,
            respond_user,
            respond_admin_log,
            rickroll
        ],
    )
}
