use crate::models::*;
use dioxus::logger::tracing::{info};
use gloo_net::http::Request;
const API: &str = "http://localhost:8080/api";

pub async fn register(req: RegisterRequest) -> anyhow::Result<RegisterResponse> {
    Ok(Request::post(&format!("{API}/auth/register"))
        .json(&req)?
        .send()
        .await?
        .json()
        .await?)
}

pub async fn login(req: LoginRequest) -> anyhow::Result<LoginResponse> {
    Ok(Request::post(&format!("{API}/auth/login"))
        .json(&req)?
        .send()
        .await?
        .json()
        .await?)
}

pub async fn load_posts(limit: i32, offset: i32) -> anyhow::Result<Vec<Post>> {
    info!("load_posts: sending request");
    let res: Vec<Post> = Request::get(&format!("{API}/post?limit={limit}&offset={offset}"))
        .send()
        .await?
        .json()
        .await?;
    info!("load_posts: got {} posts", res.len());
    Ok(res)
}

pub async fn create_post(token: &str, title: String, content: String) -> anyhow::Result<Post> {
    let res: serde_json::Value = Request::post(&format!("{API}/post"))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&serde_json::json!({
            "title": title,
            "content": content
        }))?
        .send()
        .await?
        .json()
        .await?;

    Ok(serde_json::from_value(res["post"].clone())?)
}

pub async fn update_post(
    token: &str,
    id: i64,
    title: Option<String>,
    content: Option<String>,
) -> anyhow::Result<Post> {
    let res: serde_json::Value = Request::put(&format!("{API}/post/{id}"))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&serde_json::json!({
            "title": title,
            "content": content
        }))?
        .send()
        .await?
        .json()
        .await?;

    Ok(serde_json::from_value(res["post"].clone())?)
}

pub async fn delete_post(token: &str, id: i64) -> anyhow::Result<()> {
    Request::delete(&format!("{API}/post/{id}"))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await?;

    Ok(())
}
