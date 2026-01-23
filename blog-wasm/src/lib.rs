use dioxus::prelude::*;

fn app() -> Element {
    println!("App function called!");
    log::info!("App function called via log!");

    rsx! {
        div {
            h1 { "Hello World!" }
            p { "Test message" }
        }
    }
}

fn main() {
    println!("Main function called!");
    dioxus::launch(app);
}
