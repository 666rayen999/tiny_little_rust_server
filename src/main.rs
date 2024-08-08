use std::{fmt::Debug, sync::Mutex};

use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use lazy_static::lazy_static;

struct User {
    id: usize,
    username: String,
    posts: Vec<&'static Post>,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}- {}: {:?}",
            self.id,
            self.username,
            self.posts.iter().map(|p| &p.content).collect::<Vec<_>>(),
        ))
    }
}

struct Post {
    creator: &'static User,
    content: String,
}

impl Debug for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\"{}\" by {}",
            self.content, self.creator.username
        ))
    }
}

struct Database {
    users: Vec<User>,
    posts: Vec<Post>,
}

impl Database {
    fn create_user(&mut self, username: String) {
        self.users.push(User {
            id: self.users.len(),
            username,
            posts: Vec::with_capacity(8),
        })
    }
    fn add_post(&mut self, user_id: usize, content: String) {
        let user: *const User = &self.users[user_id];
        self.posts.push(Post {
            creator: unsafe { &*user },
            content,
        });
        if self.posts.len() > 0 {
            let post: *const Post = &self.posts[self.posts.len() - 1];
            self.users[user_id].posts.push(unsafe { &*post });
        }
    }
}

lazy_static! {
    static ref DB: Mutex<Database> = Mutex::new(Database {
        users: Vec::with_capacity(128),
        posts: Vec::with_capacity(1024),
    });
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(users))
        .route("/posts", get(posts))
        .route("/user/:id", get(user))
        .route("/user/:id/create/:post", get(post))
        .route("/create/:username", get(create))
        .route("/", get(init));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn init() -> impl IntoResponse {
    let lock = DB.lock().unwrap();
    format!("Users:\n{:#?}\n\nPosts:\n{:#?}", lock.users, lock.posts)
}

async fn users() -> impl IntoResponse {
    let lock = DB.lock().unwrap();
    format!("{:#?}", lock.users)
}

async fn user(Path(id): Path<usize>) -> impl IntoResponse {
    let lock = DB.lock().unwrap();
    if let Some(user) = lock.users.get(id) {
        format!("{:#?}", user)
    } else {
        "User Doesnt EXIST".into()
    }
}

async fn create(Path(username): Path<String>) -> impl IntoResponse {
    let mut lock = DB.lock().unwrap();
    lock.create_user(username);
    if let Some(user) = lock.users.last() {
        format!("{:#?}", user)
    } else {
        "Couldnt Create User".into()
    }
}

async fn post(Path((user, post)): Path<(usize, String)>) -> impl IntoResponse {
    let mut lock = DB.lock().unwrap();
    if let Some(_) = lock.users.get(user) {
        lock.add_post(user, post);
        if let Some(post) = lock.posts.last() {
            format!("{:#?}", post)
        } else {
            "Couldnt Create Post".into()
        }
    } else {
        "User Doesnt EXIST".into()
    }
}

async fn posts() -> impl IntoResponse {
    let lock = DB.lock().unwrap();
    format!("{:#?}", lock.posts)
}
