use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::Deserialize;
use web_sys::window;
const TOKEN_KEY: &str = "blog_token";

#[derive(Deserialize)]
struct JwtClaims {
    sub: String, // user_id
}

pub fn save_token(token: &str) {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let _ = storage.set_item(TOKEN_KEY, token);
}

pub fn load_token() -> Option<String> {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    storage.get_item(TOKEN_KEY).ok().flatten()
}

pub fn clear_token() {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let _ = storage.remove_item(TOKEN_KEY);
}

pub fn get_user_id() -> Option<i64> {
    let token = load_token()?;

    let payload = token.split('.').nth(1)?;
    let decoded = URL_SAFE_NO_PAD.decode(payload).ok()?;
    let claims: JwtClaims = serde_json::from_slice(&decoded).ok()?;

    claims.sub.parse::<i64>().ok()
}
