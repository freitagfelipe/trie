use leptos::*;

mod app;
mod trie;

use app::App;

pub fn start() {
    mount_to_body(|cx| {
        view! {
            cx, <App />
        }
    });
}
