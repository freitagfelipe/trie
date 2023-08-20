use leptos::*;

mod app;
mod colors;
mod components;
mod trie;
mod views;

use app::App;

pub fn start() {
    mount_to_body(|cx| {
        view! {
            cx, <App />
        }
    });
}
