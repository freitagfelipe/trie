use crate::colors::MAIN_BLACK;
use leptos::*;
use leptos_router::A;
use styled::style;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    let styles = style! {
        header {
            background-color: ${MAIN_BLACK};
            color: white;
            padding: 10px 15px;
        }

        nav {
            display: flex;
            align-items: center;
            justify-content: space-between;
        }

        ul {
            display: flex;
            align-items: center;
            gap: 10px;
        }

        li {
            list-style: none;
        }

        [aria-current=page] {
            background-color: red;
        }
    };

    styled::view! { cx, styles,
        <header>
            <nav>
                <h1>Trie</h1>
                <ul>
                    <li>
                        <A href="/" exact=true>home</A>
                    </li>
                    <li>
                        <A href="/about" exact=true>about</A>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
