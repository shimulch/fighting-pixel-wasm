use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Window, Document, Element, HtmlElement};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



struct Game;

impl js_sys::Function for Game {

}

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
    mount_canvas(&window, &document, &body);
    Ok(())
}


fn mount_canvas(window: &Window, document: &Document, body: &HtmlElement) {
    let mut canvas = document.create_element("canvas").unwrap();
    canvas.set_attribute("width", "800px");
    canvas.set_attribute("height", "800px");
    canvas.set_id("fp-canvas");
    body.append_child(&canvas).expect("Could not attach canvas");
    window.set_timeout_with_callback_and_timeout_and_arguments_0(render, 3000);
    start_game(&document);
}

fn render() {
    console::log_1(&JsValue::from_str("Rendering"));
}

fn start_game(document: &Document) {
    let canvas = document.get_element_by_id("fp-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    context.begin_path();
    context.arc(95.0, 50.0, 40.0, 0.0, 2.0 * std::f64::consts::PI);
    context.stroke();
    console::log_1(&context)


}