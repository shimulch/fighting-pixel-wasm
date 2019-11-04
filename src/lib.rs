extern crate js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use console_error_panic_hook;

mod core;
mod environment;
mod character;
mod utils;


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

    let mut renderables: Vec<Box<dyn core::Render>> =  Vec::new();

    environment::add_env_items(&mut renderables);
    character::add_characters(&mut renderables);

    let game_main = Rc::new(
        RefCell::new(
            core::Game::new(
                context(),
                canvas().dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
            )
        )
    );

    let game_animation = game_main.clone();
    let game_keyup = game_main.clone();
    let game_keydown = game_main.clone();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();


    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game_animation.borrow_mut().render(&mut renderables);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());


    let onkeydown = Closure::wrap(Box::new(move |event: &web_sys::KeyboardEvent| {
        game_keydown.borrow_mut().onkeydown(event.key_code());
    }) as Box<dyn Fn(&web_sys::KeyboardEvent)>);

    window().set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));


    let onkeyup = Closure::wrap(Box::new(move |event: &web_sys::KeyboardEvent| {
        game_keyup.borrow_mut().onkeyup(event.key_code());
    }) as Box<dyn Fn(&web_sys::KeyboardEvent)>);
    window().set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));


    onkeyup.forget();
    onkeydown.forget();
    Ok(())
}
