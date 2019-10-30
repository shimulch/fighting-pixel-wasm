use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

pub trait Render {
    fn render(&self, ctx: &CanvasRenderingContext2d, state: &mut GameState);
}

pub struct GameState {
    x: f64
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

    pub fn render(self: &Game, state: &mut GameState) {
        self.clear();
//        for renderable in self.items {
//            renderable.render(&self.ctx, state);
//        }
        self.ctx.set_fill_style(&JsValue::from_str("#bbedf9"));
        self.ctx.fill_rect(0.0, 0.0, self.canvas.width().into(), self.canvas.height().into());
        self.ctx.begin_path();
        self.ctx.arc(state.x, 50.0, 40.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        self.ctx.stroke();
        state.x += 1.0;
    }

    fn clear(self: &Game) {
        self.ctx.clear_rect(0.0, 0.0, self.canvas.width().into(), self.canvas.height().into())
    }
}