use dioxus::{prelude::*, html::{body, button}};

pub fn create_ui(cx: Scope) -> Element {
    cx.render(
        rsx! {
            style { include_str!("css/main.css") }
            div {
                calculator(cx)
            }
        }
        )
}

fn calculator(cx: Scope) -> Element {
    cx.render(
        rsx! {
            body {
                div {class:"buttons",
                button {"/"}
                button {"7"}
                button {"8"}
                button {"9"}
                button {"*"}
                button {"4"}
                button {"5"}
                button {"6"}
                button {"-"}
                button {"1"}
                button {"2"}
                button {"3"}
                button {"+"}
                button {"0"}
                button {"."}
                button {"="}
                }
            }
        }
    )
}
