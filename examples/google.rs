//! Render google.com!

use dioxus::prelude::*;

fn main() {
    dioxus_blitz::launch(app);
}

fn app() -> Element {
    let content = include_str!("./google_bits/google.html");

    rsx! {
        div { dangerous_inner_html: "{content}" }
    }
}
