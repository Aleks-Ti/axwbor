use crate::models::Post;
use dioxus::logger::tracing::{Level, info};
use dioxus::prelude::*;
mod api;
mod auth;
mod models;

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum View {
    Login,
    Register,
    List, // всегда можно
    Create,
    Update,
    Get,
    Delete,
}

fn App() -> Element {
    let token = use_signal(|| auth::load_token());
    let view = use_signal(|| View::List);

    rsx! {
        h1 { "Blog" }

        match view() {
        View::List => rsx!(PostsPage{token, view}),
        View::Login => rsx!(LoginForm{token, view}),
        View::Register => rsx!(RegisterForm{view}),
        View::Create => rsx!(RegisterForm{view}),
        View::Update => rsx!(RegisterForm{view}),
        View::Get => rsx!(RegisterForm{view}),
        View::Delete => rsx!(RegisterForm{view}),
        }
    }
}

#[component]
fn RegisterForm(view: Signal<View>) -> Element {
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        h2 { "Register" }
        input { placeholder: "username", oninput: move |e| username.set(e.value()) }
        input { placeholder: "email", oninput: move |e| email.set(e.value()) }
        input { r#type: "password", placeholder: "password", oninput: move |e| password.set(e.value()) }

        button {
            onclick: move |_| {
                spawn(async move {
                    let _ = crate::api::register(
                        crate::models::RegisterRequest {
                            username: username(),
                            email: email(),
                            password: password(),
                        }
                    ).await;
                    view.set(View::List);
                });
            },
            "Create account"
        }

        button {
            onclick: move |_| view.set(View::List),
            "Cancel"
        }
    }
}

#[component]
fn LoginForm(token: Signal<Option<String>>, view: Signal<View>) -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        h2 { "Login" }

        input {
            placeholder: "username",
            oninput: move |e| username.set(e.value())
        }
        input {
            r#type: "password",
            placeholder: "password",
            oninput: move |e| password.set(e.value())
        }

        button {
            onclick: move |_| {
                spawn(async move {
                    if let Ok(res) = api::login(
                        models::LoginRequest {
                            username: username(),
                            password: password(),
                        }
                    ).await {
                        auth::save_token(&res.access_token);
                        token.set(Some(res.access_token));
                        view.set(View::List);
                    }
                });
            },
            "Login"
        }

        button {
            onclick: move |_| view.set(View::List),
            "Cancel"
        }
    }
}

#[component]
fn CreatePost(token: Signal<Option<String>>, posts: Signal<Vec<Post>>) -> Element {
    let mut title = use_signal(String::new);
    let mut content = use_signal(String::new);

    rsx! {
        h3 { "Create post" }

        input {
            placeholder: "title",
            oninput: move |e| title.set(e.value())
        }
        textarea {
            placeholder: "content",
            oninput: move |e| content.set(e.value())
        }

        button {
            onclick: move |_| {
                if let Some(token) = token() {
                    spawn(async move {
                        if let Ok(post) =
                            api::create_post(&token, title(), content()).await
                        {
                            posts.push(post);
                        }
                    });
                }
            },
            "Create post"
        }
    }
}

#[component]
fn PostsPage(token: Signal<Option<String>>, view: Signal<View>) -> Element {
    let posts = use_signal(Vec::<models::Post>::new);

    use_effect(move || {
        println!("Fetching posts...");
        let posts = posts.clone();
        spawn(async move {
            match api::load_posts(20, 0).await {
                Ok(data) => {
                    println!("Got {} posts", data.len());
                    // запись должна быть вне await‑цепочки
                    posts.clone().set(data);
                }
                Err(e) => {
                    println!("Error loading posts: {:?}", e);
                }
            }
        });
    });

    rsx! {
        if token().is_none() {
            button {
                onclick: move |_| view.set(View::Login),
                "Login"
            }
            button {
                onclick: move |_| view.set(View::Register),
                "Register"
            }
        } else {
            button {
                onclick: move |_| {
                    auth::clear_token();
                    token.set(None);
                },
                "Logout"
            }
        }

        // Создание поста — только если залогинен
        if token().is_some() {
            CreatePost { token: token.clone(), posts: posts.clone() }
        }

        hr {}

        // Список постов — всегда
        for post in posts().iter() {
            div {
                h3 { "{post.title}" }
                small { "by {post.author_id} · {post.created_at}" }
                p { "{post.content}" }
            }
        }
    }
}
