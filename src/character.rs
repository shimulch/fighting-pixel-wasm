use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use js_sys::Math;
use std::f64::consts::PI;
use crate::core::{Render, Create, GameState, window_width, window_height, js_log, Controller};
use crate::utils::{Shape, Point};



struct Hero {
    x: f64,
    y: f64,
    color: String,
    wheel_rotation: f64,
    wheel_velocity: f64,
}


impl Create<Hero> for Hero {
    fn create() -> Hero {
        Hero {
            x: 80.0,
            y: window_height() - 150.0,
            color: "#2980b9".to_string(),
            wheel_rotation: 1.0,
            wheel_velocity: 1.0
        }
    }
}

impl Render for Hero {

    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {

        let body_start = Point {x: self.x, y: self.y};
        let body_next = Point {x: self.x + 20.0, y: self.y + 60.0};
        let body_last = Point {x: self.x + 40.0, y: self.y + 10.0};

        Shape::new(ctx)
            .triangle(&body_start, &body_next, &body_last)
            .fill(&self.color)
            .build();


        let head_start = Point{x: self.x, y: self.y - 1.0};
        let head_next = Point{x: self.x + 20.0, y: head_start.y - 20.0};
        let head_last = Point{x: self.x + 40.0, y: head_start.y + 10.0};
        Shape::new(ctx)
            .triangle(&head_start, &head_next, &head_last)
            .fill(&self.color)
            .build();

//
        let sides = 10.0;
        let size =20.0;
        let wheel_left_center = Point{x: self.x - 6.0, y: self.y + 50.0};
        Shape::new(ctx)
            .with_rotation(&wheel_left_center, self.wheel_rotation)
            .draw_xgon(sides, size, &wheel_left_center)
            .stroke(&self.color, 2.0)
            .build();

        Shape::new(ctx)
            .with_rotation(&wheel_left_center, self.wheel_rotation)
            .draw_xgon(sides, size-5.0, &wheel_left_center)
            .fill(&self.color)
            .build();

        let wheel_right_center = Point{x: wheel_left_center.x + 54.0, y: wheel_left_center.y};
        Shape::new(ctx)
            .with_rotation(&wheel_right_center, self.wheel_rotation)
            .draw_xgon(sides, size, &wheel_right_center)
            .stroke(&self.color, 2.0)
            .build();

        Shape::new(ctx)
            .with_rotation(&wheel_right_center, self.wheel_rotation)
            .draw_xgon(sides, size-5.0, &wheel_right_center)
            .fill(&self.color)
            .build();

        self.wheel_rotation = self.wheel_rotation + self.wheel_velocity;
        if self.wheel_rotation > 360.0 {
            self.wheel_rotation = 1.0;
        }
    }
}


pub fn add_characters(render_stack: &mut Vec<Box<dyn Render>>) {
    render_stack.push(Box::new(Hero::create()))
}