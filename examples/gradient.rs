use dioxus::prelude::*;

fn main() {
    dioxus_blitz::launch(app);
}

fn app() -> Element {
    rsx! {
        style { {CSS} }
        div {
            // https://developer.mozilla.org/en-US/docs/Web/CSS/gradient/linear-gradient
            class: "flex flex-row",
            for s in [
                "linear-gradient(#e66465, #9198e5)",
                "linear-gradient(0.25turn, #3f87a6, #ebf8e1, #f69d3c)",
                "linear-gradient(to left, #333, #333 50%, #eee 75%, #333 75%)",
                r#"linear-gradient(217deg, rgba(255,0,0,.8), rgba(255,0,0,0) 70.71%),
                linear-gradient(127deg, rgba(0,255,0,.8), rgba(0,255,0,0) 70.71%),
                linear-gradient(336deg, rgba(0,0,255,.8), rgba(0,0,255,0) 70.71%)"#,] {
                div { background: s, id: "a" }
            }
        }
        div {
            class: "flex flex-row",
            for s in [
                "linear-gradient(to right, red 0%, 0%, blue 100%)",
                "linear-gradient(to right, red 0%, 25%, blue 100%)",
                "linear-gradient(to right, red 0%, 50%, blue 100%)",
                "linear-gradient(to right, red 0%, 100%, blue 100%)",
                "linear-gradient(to right, yellow, red 10%, 10%, blue 100%)",] {
                div { background: s, id: "a" }
            }

        }
        div {
            class: "flex flex-row",
            for s in [
                "repeating-linear-gradient(#e66465, #e66465 20px, #9198e5 20px, #9198e5 25px)",
                "repeating-linear-gradient(45deg, #3f87a6, #ebf8e1 15%, #f69d3c 20%)",
                r#"repeating-linear-gradient(transparent, #4d9f0c 40px),
                repeating-linear-gradient(0.25turn, transparent, #3f87a6 20px)"#, ] {
                div { background: s, id: "a" }
            }

        }
    }
}

const CSS: &str = r#"
.flex { display: flex; }
.flex-row { flex-direction: row; }
#a {
    height: 200px;
    width: 300px;
}
"#;
