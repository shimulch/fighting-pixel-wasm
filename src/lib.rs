use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Window, Document, Element, HtmlElement};
use std::rc::Rc;
use std::cell::RefCell;


#[wasm_bindgen]
extern "C" {
    fn requestAnimationFrame(callback: JsValue);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}


fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}


#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {

    init();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        render();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}


fn init() {
    let doc = document();
    let mut canvas = doc.create_element("canvas").unwrap();
    canvas.set_attribute("width", "800px");
    canvas.set_attribute("height", "800px");
    canvas.set_id("fp-canvas");
    body().append_child(&canvas).expect("Could not attach canvas");
}

fn render() {
    let canvas = document().get_element_by_id("fp-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    context.begin_path();
    context.arc(95.0, 50.0, 40.0, 0.0, 2.0 * std::f64::consts::PI);
    context.stroke();
}