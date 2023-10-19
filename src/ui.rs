use dioxus::prelude::*;
use crate::functionality::*;

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
    let sum = use_state(cx, || 0);
    let op = use_state(cx, || 0);
    // Division = 1
    // Multiply = 2
    // Addition = 3
    // Subtract = 4
    cx.render(
        rsx! {
            body {
                h2 {"{sum}"}
                div {
                    class:"buttons",
                    button {
                        onclick: move |_| op.set(1),
                        "/"}
                    button {
                        onclick: move |_| {
                            sum.set(eval_math(sum.get(), 7, op.get()))
                        },
                        "7"}
                    button {"8"}
                    button {"9"}
                    button {
                        onclick: move |_| op.set(2),
                        "*"}
                    button {"4"}
                    button {"5"}
                    button {"6"}
                    button {
                        onclick: move |_| op.set(3),
                        "-"}
                    button {"1"}
                    button {"2"}
                    button {"3"}
                    button {
                        onclick: move |_| op.set(4),
                        "+"}
                    button {"0"}
                    button {"."}
                    button {"="}
                }
            }
        }
    )
}
