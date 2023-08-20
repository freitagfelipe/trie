use crate::components::header::Header;
use leptos::*;
use styled::style;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
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
            <p>Home page</p>
        </main>
    }
}
