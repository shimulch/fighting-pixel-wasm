extern crate js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Window, Document, Element, HtmlElement};
use std::rc::Rc;
use std::cell::RefCell;
use console_error_panic_hook;

mod core;


#[wasm_bindgen]
extern "C" {
    fn requestAnimationFrame(callback: JsValue);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


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

fn canvas() -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id("fp-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
}

fn context() -> web_sys::CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}


#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let game = init();
    let mut state = core::GameState::new();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();


    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game.render(&mut state);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}


fn init() -> core::Game {
    core::Game::new(context(), canvas().dyn_into::<web_sys::HtmlCanvasElement>().unwrap())
}
