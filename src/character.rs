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
            wheel_velocity: 5.0
        }
    }
}

impl Render for Hero {

    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {

        if state.right {
            self.x += 5.0;
            self.wheel_velocity = 10.0;
        } else {
            self.wheel_velocity = 5.0;
        }

        if state.left {
            self.x -= 5.0;
        }

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

        let eye_wrapper_center = Point{x: head_start.x + 20.0, y: head_start.y - 5.0};
        Shape::new(ctx)
            .with_scale(0.8, 0.8, &eye_wrapper_center)
            .triangle(&head_start, &head_next, &head_last)
            .fill("#FFF")
            .build();

        let eye_center = Point{x: eye_wrapper_center.x + 10.0, y: eye_wrapper_center.y + 8.0};
        Shape::new(ctx)
            .with_scale(0.4, 0.4, &eye_center)
            .triangle(&head_start, &head_next, &head_last)
            .fill("#000")
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

        let wheel_right_center = Point{x: wheel_left_center.x + 50.0, y: wheel_left_center.y};
        Shape::new(ctx)
            .with_rotation(&wheel_right_center, self.wheel_rotation)
            .with_scale(0.8, 0.8, &wheel_right_center)
            .draw_xgon(sides, size, &wheel_right_center)
            .stroke(&self.color, 2.0)
            .build();

        Shape::new(ctx)
            .with_rotation(&wheel_right_center, self.wheel_rotation)
            .with_scale(0.8, 0.8, &wheel_right_center)
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