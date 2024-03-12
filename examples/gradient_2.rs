use dioxus::prelude::*;

fn main() {
    dioxus_blitz::launch(app);
}

fn app() -> Element {
    rsx! {
        style { {CSS} }
        div {
            class: "flex flex-row",
            for s in [
                "radial-gradient(circle, red 20px, black 21px, blue)",
                "radial-gradient(closest-side, #3f87a6, #ebf8e1, #f69d3c)",
                "radial-gradient(circle at 100%, #333, #333 50%, #eee 75%, #333 75%)",
                r#"radial-gradient(ellipse at top, #e66465, transparent),
                radial-gradient(ellipse at bottom, #4d9f0c, transparent)"#,
                "radial-gradient(closest-corner circle at 20px 30px, red, yellow, green)",] {
                div { background: s, id: "a" }
            }
        }
        div {
            class: "flex flex-row",
            for s in [
                "repeating-radial-gradient(#e66465, #9198e5 20%)",
                "repeating-radial-gradient(closest-side, #3f87a6, #ebf8e1, #f69d3c)",
                "repeating-radial-gradient(circle at 100%, #333, #333 10px, #eee 10px, #eee 20px)",
            ] {
                div { background: s, id: "a" }
            }
        }
        div {
            class: "flex flex-row",
            for s in [
                "conic-gradient(red, orange, yellow, green, blue)",
                "conic-gradient(from 0.25turn at 50% 30%, #f69d3c, 10deg, #3f87a6, 350deg, #ebf8e1)",
                "conic-gradient(from 3.1416rad at 10% 50%, #e66465, #9198e5)",
                r#"conic-gradient(
                 red 6deg, orange 6deg 18deg, yellow 18deg 45deg,
                 green 45deg 110deg, blue 110deg 200deg, purple 200deg)"#,
            ] {
                div { background: s, id: "a" }
            }
        }
        div {
            class: "flex flex-row",
            for s in [
                "repeating-conic-gradient(red 0%, yellow 15%, red 33%)",
                r#"repeating-conic-gradient(
                  from 45deg at 10% 50%,
                  brown 0deg 10deg,
                  darkgoldenrod 10deg 20deg,
                  chocolate 20deg 30deg
                )"#,
            ] {
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
