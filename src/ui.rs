use dioxus::prelude::*;

pub fn create_ui(cx: Scope) -> Element {
    cx.render(
        rsx! {
            div {
                "HI"
            }
        }
             )
}
