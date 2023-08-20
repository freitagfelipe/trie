use crate::views::{about::About, home::Home, not_found::NotFound};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router fallback=|cx| view! { cx, <NotFound /> }>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/about" view=About />
            </Routes>
        </Router>
    }
}
