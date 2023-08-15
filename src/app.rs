use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <main>
                <p>"Hello, World!"</p>
            </main>
        </Router>
    }
}
