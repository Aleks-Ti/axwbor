//! Blog WASM Application

use crate::models::Post;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
mod api;
mod auth;
mod models;

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum View {
    List,
    Login,
    Register,
    Create,
    Edit(i64),
}

#[component]
fn App() -> Element {
    let token = use_signal(auth::load_token);
    let view = use_signal(|| View::List);

    rsx! {
        h1 { "Blog" }

        // Статус аутентификации
        p {
            if let Some(_) = token() {
                "Status: Logged in"
            } else {
                "Status: Not logged in"
            }
        }

        match view() {
            View::List => rsx!(PostsPage { token, view }),
            View::Login => rsx!(LoginForm { token, view }),
            View::Register => rsx!(RegisterForm { view }),
            View::Create => rsx!(CreatePost { token, view }),
            View::Edit(id) => rsx!(EditPost { token, view, post_id: id }),
        }
    }
}

// ============ AUTH FORMS ============

#[component]
fn RegisterForm(view: Signal<View>) -> Element {
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(String::new);

    rsx! {
        h2 { "Register" }
        if !error().is_empty() { p { color: "red", "{error}" } }

        input { placeholder: "Username", value: "{username}", oninput: move |e| username.set(e.value()) }
        input { placeholder: "Email", value: "{email}", oninput: move |e| email.set(e.value()) }
        input { r#type: "password", placeholder: "Password", value: "{password}", oninput: move |e| password.set(e.value()) }

        button {
            onclick: move |_| {
                error.set(String::new());
                let usrn = username(); let em = email(); let ps = password();
                if usrn.trim().is_empty() || em.trim().is_empty() || ps.trim().is_empty() {
                    error.set("All fields are required".to_string());
                    return;
                }
                spawn(async move {
                    if api::register(models::RegisterRequest { username: usrn, email: em, password: ps }).await.is_err() {
                        error.set("Registration failed".to_string());
                    } else {
                        view.set(View::List);
                    }
                });
            },
            "Register"
        }
        button { onclick: move |_| view.set(View::List), "Cancel" }
    }
}

#[component]
fn LoginForm(token: Signal<Option<String>>, view: Signal<View>) -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(String::new);

    rsx! {
        h2 { "Login" }
        if !error().is_empty() { p { color: "red", "{error}" } }

        input { placeholder: "Username", value: "{username}", oninput: move |e| username.set(e.value()) }
        input { r#type: "password", placeholder: "Password", value: "{password}", oninput: move |e| password.set(e.value()) }

        button {
            onclick: move |_| {
                error.set(String::new());
                let u = username(); let p = password();
                if u.trim().is_empty() || p.trim().is_empty() {
                    error.set("Username and password required".to_string());
                    return;
                }
                spawn(async move {
                    match api::login(models::LoginRequest { username: u, password: p }).await {
                        Ok(res) => {
                            auth::save_token(&res.access_token);
                            token.set(Some(res.access_token));
                            view.set(View::List);
                        }
                        Err(_) => {
                            error.set("Login failed".to_string());
                        }
                    }
                });
            },
            "Login"
        }
        button { onclick: move |_| view.set(View::List), "Cancel" }
    }
}

// ============ POSTS PAGE ============

#[component]
fn PostsPage(token: Signal<Option<String>>, view: Signal<View>) -> Element {
    let mut posts = use_signal(Vec::<Post>::new);
    let mut loading = use_signal(|| true);

    let user_id = auth::get_user_id();

    // загрузка данных
    use_effect(move || {
        loading.set(true);
        spawn(async move {
            if let Ok(data) = api::load_posts(100, 0).await {
                posts.set(data);
            }
            loading.set(false);
        });
    });

    let posts_snapshot: Vec<Post> = posts().clone();

    rsx! {
        // ===== Auth =====
        if token().is_none() {
            button { onclick: move |_| view.set(View::Login), "Login" }
            button { onclick: move |_| view.set(View::Register), "Register" }
        } else {
            button {
                onclick: move |_| {
                    auth::clear_token();
                    token.set(None);
                },
                "Logout"
            }
            button { onclick: move |_| view.set(View::Create), "Create Post" }
        }

        hr {}

        if loading() {
            p { "Loading posts..." }
        }

        // ===== Posts =====
        for post in posts_snapshot {
            div {
                h3 { "{post.title}" }
                p { "{post.content}" }

                if let Some(uid) = user_id {
                    if uid == post.author_id {
                        button {
                            onclick: move |_| view.set(View::Edit(post.id)),
                            "Edit"
                        }

                        button {
                            onclick: move |_| {
                                if let Some(token_val) = token() {
                                    let token_val = token_val.clone();

                                    spawn(async move {
                                        let _ = api::delete_post(&token_val, post.id).await;
                                        posts.with_mut(|list| list.retain(|p| p.id != post.id));
                                        view.set(View::List);
                                    });
                                }
                            },
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}

// ============ CREATE / EDIT ============

#[component]
fn CreatePost(token: Signal<Option<String>>, view: Signal<View>) -> Element {
    let mut title = use_signal(String::new);
    let mut content = use_signal(String::new);
    let mut error = use_signal(String::new);

    rsx! {
        h2 { "Create New Post" }
        if !error().is_empty() { p { color: "red", "{error}" } }

        input {
            placeholder: "Title",
            value: "{title}",
            oninput: move |e| title.set(e.value())
        }
        textarea {
            placeholder: "Content",
            value: "{content}",
            oninput: move |e| content.set(e.value())
        }

        button {
            onclick: move |_| {
                error.set(String::new());
                let t = title(); let c = content();
                if t.trim().is_empty() || c.trim().is_empty() {
                    error.set("Title and content are required".to_string());
                    return;
                }
                if let Some(token_val) = token() {
                    let token_clone = token_val.clone();
                    spawn(async move {
                        if api::create_post(&token_clone, t, c).await.is_ok() {
                            view.set(View::List);
                        } else {
                            error.set("Failed to create post".to_string());
                        }
                    });
                }
            },
            "Publish"
        }
        button { onclick: move |_| view.set(View::List), "Cancel" }
    }
}

#[component]
fn EditPost(token: Signal<Option<String>>, view: Signal<View>, post_id: i64) -> Element {
    let mut title = use_signal(String::new);
    let mut content = use_signal(String::new);
    let mut error = use_signal(String::new);
    let mut original_post = use_signal(|| None::<Post>);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        if let Some(token_val) = token() {
            let _ = token_val.clone();
            spawn(async move {
                if let Ok(posts) = api::load_posts(100, 0).await
                    && let Some(post) = posts.into_iter().find(|p| p.id == post_id)
                {
                    title.set(post.title.clone());
                    content.set(post.content.clone());
                    original_post.set(Some(post));
                }

                loading.set(false);
            });
        }
    });

    rsx! {
        if loading() { p { "Loading post..." } }
        else {
            h2 { "Edit Post" }
            if !error().is_empty() { p { color: "red", "{error}" } }

            input {
                placeholder: "Title",
                value: "{title}",
                oninput: move |e| title.set(e.value())
            }
            textarea {
                placeholder: "Content",
                value: "{content}",
                oninput: move |e| content.set(e.value())
            }

            button {
                onclick: move |_| {
                    error.set(String::new());
                    let t = title(); let c = content();
                    if t.trim().is_empty() || c.trim().is_empty() {
                        error.set("Title and content are required".to_string());
                        return;
                    }
                    if let Some(token_val) = token() {
                        let token_clone = token_val.clone();
                        spawn(async move {
                            if api::update_post(&token_clone, post_id, Some(t), Some(c)).await.is_ok() {
                                view.set(View::List);
                            } else {
                                error.set("Failed to update post".to_string());
                            }
                        });
                    }
                },
                "Save"
            }
            button { onclick: move |_| view.set(View::List), "Cancel" }
        }
    }
}
