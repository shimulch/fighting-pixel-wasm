use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use js_sys::Math;
use crate::core::{Render, GameState, js_log, window_width};

struct Sun;

impl Render for Sun {
    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
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
    y: f64,
    velocity: f64
}

impl Cloud {
    fn new() -> Cloud {
        Cloud {
            x: window_width() + Math::random() * 400.0 + 200.0,
            y: Math::random() * 250.0 + 10.0,
            velocity: Math::random() * 2.0 + 0.2
        }
    }

    fn reset(&mut self) {
        self.x = window_width() + Math::random() * 400.0 + 200.0;
        self.y = Math::random() * 250.0 + 10.0;
        self.velocity = Math::random() * 2.0 + 0.2;
    }
}

impl Render for Cloud {

    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {

        if self.x + 120.0 > 0.0 {
            self.x = self.x - self.velocity;
        } else {
            self.reset();
        }

        ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
        ctx.begin_path();
        ctx.rect(self.x, self.y, 40.0, 30.0);
        ctx.fill();
        ctx.begin_path();
        ctx.rect(self.x - 40.0, self.y + 28.0, 120.0, 40.0);
        ctx.fill();
    }
}

struct CloudController {
    clouds: Vec<Box<Cloud>>
}

impl CloudController {
    fn new() -> CloudController {
        let mut clouds = Vec::new();

        for _ in 1..5 {
            clouds.push(
                Box::new(
                    Cloud::new()
                )
            );
        }

        CloudController {
            clouds
        }
    }
}

impl Render for CloudController {

    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
        for cloud in &mut self.clouds {
            cloud.render(ctx, state);
        }
    }
}

pub fn add_env_items(render_stack: &mut Vec<Box<dyn Render>>) -> &mut Vec<Box<dyn Render>> {

    let cloud_ctrl = CloudController::new();

    render_stack.push(Box::new(Sun{}));
    render_stack.push(Box::new(cloud_ctrl));
    render_stack
}