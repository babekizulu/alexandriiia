#![allow(non_snake_case)]
//libs
use dioxus::prelude::*;
fn main() {
    //launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello from Dioxus!"
        }
    })
}
