use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::core::{Render, GameState, js_log};
use std::f64::consts::PI;

struct Sun;

impl Render for Sun {
    fn render(&self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
        ctx.set_fill_style(&JsValue::from_str("#fbe35f"));
        ctx.begin_path();
        ctx.rect(50.0, 50.0, 100.0, 100.0);
        ctx.fill();

        ctx.set_fill_style(&JsValue::from_str("#f9d71c"));
        ctx.begin_path();
        ctx.rect(60.0, 60.0, 80.0, 80.0);
        ctx.fill();
    }
}

struct Cloud {
    x: f64,
    y: f64
}

impl Cloud {
    fn new (x: f64, y: f64) -> Cloud {
        Cloud {
            x,
            y
        }
    }
}

impl Render for Cloud {

    fn render(&self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
        ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
        ctx.begin_path();
        ctx.rect(self.x, self.y, 150.0, 100.0);
        ctx.fill();
    }
}

pub fn add_env_items(render_stack: &mut Vec<Box<dyn Render>>) -> &mut Vec<Box<dyn Render>> {
    render_stack.push(Box::new(Sun{}));
    render_stack
}