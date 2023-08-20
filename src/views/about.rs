use crate::components::header::Header;
use leptos::*;
use styled::style;

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    let styles = style! {
        main {
            display: flex;
            align-items: center;
            justify-content: center;
        }
    };

    styled::view! { cx, styles,
        <Header />
        <main>
            <p>About page</p>
        </main>
    }
}
