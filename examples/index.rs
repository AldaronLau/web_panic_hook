#![recursion_limit = "512"]

use std::fs;

use html::root::Html;

fn main() {
    const SCRIPT: &str = "import init, {} from './example.js';\
                          init('./example_bg.wasm');";

    let html = Html::builder()
        .head(|h| h.script(|s| s.type_("module").push(SCRIPT)))
        .body(|b| b)
        .build();

    fs::create_dir("./html").unwrap();
    fs::write("./html/index.html", html.to_string()).unwrap();
}
