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
    let sum = use_state(cx, || 0.0);
    let num = use_ref(cx, || String::new());
    let op = use_state(cx, || 0);
    // Division = 1
    // Multiply = 2
    // Addition = 3
    // Subtract = 4
    cx.render(
        rsx! {
            body {
                h2 {"{sum}"}
                h2 {"{num.read()}"}
                div {
                    class: "buttons",
                    button {
                        onclick: move |_| op.set(1),
                        "/"}
                    button {
                        onclick: move |_| {
                            num.with_mut(|x| x.push('7'));
                        },
                        "7"}
                    button {
                        onclick: move |_| {
                            num.with_mut(|x| x.push('8'));
                        },
                        "8"}
                    button {
                        onclick: move |_| {
                            num.with_mut(|x| x.push('9'));
                        },
                        "9"}
                    button {
                        onclick: move |_| op.set(2),
                        "*"}
                    button {
                        onclick: move |_| {
                            num.with_mut(|x| x.push('4'));
                        },
                        "4"}
                    button {
                        onclick: move |_| {
                            num.with_mut(|x| x.push('5'));
                        },
                        "5"}
                    button {
                        onclick: move |_| {
                            num.with_mut(|x| x.push('6'));
                        },
                        "6"}
                    button {
                        onclick: move |_| op.set(3),
                        "-"}
                    button {
                        onclick: move |_| {
                            num.with_mut(|x| x.push('1'));
                        },
                        "1"}
                    button {
                        onclick: move |_| {
                            num.with_mut(|x| x.push('2'));
                        },
                        "2"}
                    button {
                        onclick: move |_| {
                            num.with_mut(|x| x.push('3'));
                        },
                        "3"}
                    button {
                        onclick: move |_| {
                            if num.read().len() > 0 {
                                sum.set(num.read().parse::<f64>().unwrap());
                                num.with_mut(|x| x.clear());
                                //println!("num: {}", num.read());
                            }
                            op.set(4);
                        },
                        "+"}
                    button {
                        onclick: move |_| {
                            num.with_mut(|x| x.push('0'));
                        },
                        "0"}
                    button {
                        onclick: move |_| {
                            //sum.set(eval_math(sum.get(), '.', op.get()))
                        },
                        "."}
                    button {
                        onclick: move |_| {
                            sum.set(eval_math(sum.get(), &num.read(), op.get()));
                            num.with_mut(|x| x.clear());
                        },
                        "="}
                }
            }
        }    )
}
