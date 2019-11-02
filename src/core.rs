extern crate js_sys;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen]
extern "C" {
    fn requestAnimationFrame(callback: JsValue);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


pub fn js_log(s: &str) {
    log(s);
}

pub fn window_width() -> f64 {
    window().unwrap().inner_width().unwrap().as_f64().unwrap()
}

pub fn window_height() -> f64 {
    window().unwrap().inner_height().unwrap().as_f64().unwrap()
}

pub trait Render {
    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState);
}

pub trait Create<T> {
    fn create() -> T;
}

pub struct Controller<T: Render + Create<T>> {
    items: Vec<Box<T>>
}

impl<T: Render + Create<T>> Controller<T> {
    pub fn new(count: i16) -> Controller<T> {
        let mut items: Vec<Box<T>> = Vec::new();
        for _ in 1..count {
            items.push(Box::new(T::create()));
        }
        Controller {
            items
        }
    }
}

impl<T: Render + Create<T>> Render for Controller<T> {
    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: & mut GameState) {
        for item in &mut self.items {
            item.render(ctx, state);
        }
    }
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