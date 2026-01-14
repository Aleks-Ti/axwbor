#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct RegisterResponse {
    pub user_id: i64,
    pub email: String,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
#[derive(Debug, serde::Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: Option<String>, // или chrono::DateTime
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct PostFilterRequest {
    pub limit: i32,
    pub offset: i32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}