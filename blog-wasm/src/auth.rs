use web_sys::window;

const TOKEN_KEY: &str = "blog_token";

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
