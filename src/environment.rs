use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use js_sys::Math;
use crate::core::{Render, Create, GameState, window_width, window_height, js_log, Controller};


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

impl Create<Cloud> for Cloud {
    fn create() -> Cloud {
        Cloud {
            x: window_width() + Math::random() * 400.0 + 200.0,
            y: Math::random() * 250.0 + 10.0,
            velocity: Math::random() * 2.0 + 0.2
        }
    }
}

impl Cloud {

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

#[derive(Debug)]
struct Mountain {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    velocity: f64,
    color: String
}

impl Mountain {
    fn update(&mut self) {
        let right_edge_position = self.x + (self.width/2.0);
        if right_edge_position < 0.0 {
            self.width = Mountain::generate_width();
            self.x = Mountain::generate_x(self.width);
            self.color = Mountain::generate_color();
            self.velocity = Math::random();
        } else {
            self.x = self.x - self.velocity;
        }
    }
    fn generate_color() -> String {
        let hill_colors = ["#8dc63f", "#9edb4c"];
        let idx = Math::round(Math::random() * ((hill_colors.len() as f64) - 1.0)) as usize;
        hill_colors[idx].to_string()
    }

    fn generate_width() -> f64 {
        Math::random() * 800.0 + 400.0
    }

    fn generate_height() -> f64 {
        Math::random() * 400.0
    }
    fn generate_x(width: f64) -> f64 {
        (window_width() + width / 2.0) + (Math::random() * 400.0)
    }

}

impl Create<Mountain> for Mountain {
    fn create() -> Mountain {

        let height = Mountain::generate_height();
        let width = Mountain::generate_width();
        Mountain{
            x: Mountain::generate_x(width),
            y: window_height() - height,
            width,
            height,
            velocity: Math::random(),
            color: Mountain::generate_color()
        }
    }
}

impl Render for Mountain {

    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {

        js_log(&format!("{:?}", self));

        ctx.set_fill_style(&JsValue::from_str(&self.color));
        ctx.begin_path();
        ctx.move_to(self.x, self.y);
        ctx.line_to(self.x - self.width / 2.0, self.y + self.height);
        ctx.line_to(self.x + self.width, self.y + self.height);
        ctx.close_path();
        ctx.fill();
        self.update();
    }
}


/// Adds environment objects to render stack
/// # Args
/// render_stack - a Vector of objects that can be rendered
pub fn add_env_items(render_stack: &mut Vec<Box<dyn Render>>) -> &mut Vec<Box<dyn Render>> {

//    let cloud_ctrl = CloudController::new();
    let cloud_ctrl: Controller<Cloud> = Controller::new(5);
    let mountain_ctrl: Controller<Mountain> = Controller::new(10);


    render_stack.push(Box::new(Sun{}));

    render_stack.push(Box::new(cloud_ctrl));

    render_stack.push(Box::new(mountain_ctrl));

    render_stack.push(Box::new(Ground{}));

    render_stack
}