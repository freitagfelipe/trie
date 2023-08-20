use leptos::*;
use leptos_router::A;
use styled::style;

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    let styles = style! {
        main {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100svh;
        }
    };

    styled::view! { cx, styles,
        <main>
            <h1>Page not found!</h1>
            <A href="/">Home</A>
        </main>
    }
}
