use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element, HtmlElement};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hi rust!"));

    let window = web_sys::window().expect("Could not get window");
    let document = window.document().expect("Could not get document");
    let body = document.body().expect("Could not get body");
    mount_canvas(&document, &body);
    Ok(())
}


fn mount_canvas(document: &Document, body: &HtmlElement) {
    let mut canvas = document.create_element("canvas").unwrap();
    canvas.set_attribute("width", "800px");
    canvas.set_attribute("height", "800px");
    canvas.set_id("fp-canvas");
    body.append_child(&canvas).expect("Could not attach canvas");
    start_game(&document);
}

fn start_game(document: &Document) {
    let canvas = document.get_element_by_id("fp-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    console::log_1(&canvas)
}