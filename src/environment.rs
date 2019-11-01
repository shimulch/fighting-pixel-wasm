use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use js_sys::Math;
use crate::core::{Render, GameState, window_width, window_height};


struct Sun;

impl Render for Sun {

    /// Renders how sun would show and animate
    fn render(&mut self, ctx: &CanvasRenderingContext2d, _state: &mut GameState) {
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

/// Represents a cloud
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

    /// Resets cloud position and velocity
    fn reset(&mut self) {
        self.x = window_width() + Math::random() * 400.0 + 200.0;
        self.y = Math::random() * 250.0 + 10.0;
        self.velocity = Math::random() * 2.0 + 0.2;
    }
}

impl Render for Cloud {

    /// Renders how cloud would show and animate
    fn render(&mut self, ctx: &CanvasRenderingContext2d, _state: &mut GameState) {

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

/// To control cloud behaviour
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

struct Ground;

impl Render for Ground {
    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
        ctx.set_fill_style(&JsValue::from_str("#FFF"));
        ctx.begin_path();
        ctx.rect(0.0, window_height() - 150.0, window_width(), 150.0);
        ctx.fill();

        ctx.set_fill_style(&JsValue::from_str("#EEE"));
        ctx.begin_path();
        ctx.rect(0.0, window_height() - 160.0, window_width(), 11.0);
        ctx.fill();
    }
}

/// Adds environment objects to render stack
/// # Args
/// render_stack - a Vector of objects that can be rendered
pub fn add_env_items(render_stack: &mut Vec<Box<dyn Render>>) -> &mut Vec<Box<dyn Render>> {

    let cloud_ctrl = CloudController::new();

    render_stack.push(Box::new(Ground{}));

    render_stack.push(Box::new(Sun{}));
    render_stack.push(Box::new(cloud_ctrl));

    render_stack
}