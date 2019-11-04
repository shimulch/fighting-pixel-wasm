use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use js_sys::Math;
use std::f64::consts::PI;



pub struct Point {
    pub x: f64,
    pub y: f64
}

pub struct Shape<'b> {
    ctx: &'b CanvasRenderingContext2d
}

impl<'b> Shape<'b> {

    pub fn new(ctx: &CanvasRenderingContext2d) -> Shape {
        ctx.save();
        ctx.begin_path();
        Shape{ctx}
    }

    pub fn with_rotation<'a>(&'a self, center: &Point, deg: f64) -> &'a Shape {
        self.ctx.translate(center.x, center.y);
        self.ctx.rotate((PI / 180.0) * deg);
        self.ctx.translate(-center.x, -center.y);
        self
    }

    pub fn triangle<'a>(&'a self, start: &Point, next: &Point, last: &Point) -> &'a Shape {
        self.ctx.move_to(start.x, start.y);
        self.ctx.line_to(next.x, next.y);
        self.ctx.line_to(last.x, last.y);
        self.ctx.close_path();
        self
    }

    pub fn fill<'a>(&'a self, color: &str) -> &'a Shape {
        self.ctx.set_fill_style(&JsValue::from_str(color));
        self.ctx.fill();
        self
    }

    pub fn stroke<'a>(&'a self, color: &str, stroke_width: f64) -> &'a Shape {
        self.ctx.set_stroke_style(&JsValue::from_str(color));
        self.ctx.set_line_width(stroke_width);
        self.ctx.stroke();
        self
    }

    pub fn draw_xgon<'a>(&'a self, sides: f64, size: f64, center: &Point) -> &'a Shape {
        self.ctx.move_to(center.x + size * Math::cos(0.0), center.y + size * Math::sin(0.0));
        let to_iterate = sides + 1.0;
        for c in 1..to_iterate as i16 {
            let i = c as f64;
            self.ctx.line_to(center.x + size * Math::cos(i * 2.0 * PI / sides), center.y + size * Math::sin(i * 2.0 * PI / sides));
        }
        self
    }

    pub fn build<'a>(&'a self) ->&'a Shape {
        self.ctx.restore();
        self
    }

}