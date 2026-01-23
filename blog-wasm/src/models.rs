use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterResponse {
    pub email: String,
    pub id: i64,
}

#[derive(Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct GetPostsResponse {
    pub posts: Vec<Post>,
}
