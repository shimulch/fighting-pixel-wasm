extern crate js_sys;
use wasm_bindgen::prelude::*;
use web_sys::{Window, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    fn requestAnimationFrame(callback: JsValue);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


pub fn js_log(s: &str) {
    log(s);
}


pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn canvas() -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id("fp-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
}

pub fn context() -> web_sys::CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub trait Render {
    fn render(&self, ctx: &CanvasRenderingContext2d, state: &mut GameState);
}

pub struct GameState {
    pub x: f64
}


impl GameState {
    pub fn new() -> GameState {
        GameState {
            x: 3.0
        }
    }
}
pub struct Game {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d
}

impl Game {

    pub fn new(ctx: CanvasRenderingContext2d, canvas: HtmlCanvasElement) -> Game {
        Game {
            canvas,
            ctx
        }
    }

    pub fn render<'a>(self: &Game, state: &'a mut GameState, renderables: &mut Vec<Box<dyn Render>>) -> &'a mut GameState {
        self.clear();
        self.ctx.set_fill_style(&JsValue::from_str("#bbedf9"));
        self.ctx.fill_rect(0.0, 0.0, self.canvas.width().into(), self.canvas.height().into());

        for renderable in renderables {
            renderable.render(&self.ctx, state);
        }

        state
    }

    fn clear(self: &Game) {
        self.ctx.clear_rect(0.0, 0.0, self.canvas.width().into(), self.canvas.height().into())
    }
}