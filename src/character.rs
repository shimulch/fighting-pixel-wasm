use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use js_sys::Math;
use std::f64::consts::PI;
use crate::core::{Render, Create, GameState, window_width, window_height, js_log, Controller};
use crate::utils::{Shape, Point};
use crate::text::FIRE_TEXT;

enum Direction {
    Left,
    Right
}

struct Character {
    x: f64,
    y: f64,
    color: String,
    direction: Direction,
    wheel_rotation: f64,
    wheel_velocity: f64,
}


impl Character {

    fn set_direction(&mut self, direction:Direction) {
        self.direction = direction;
    }

    fn render_head(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
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
    }

    fn render_body(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
        let body_start = Point {x: self.x, y: self.y};
        let body_next = Point {x: self.x + 20.0, y: self.y + 60.0};
        let body_last = Point {x: self.x + 40.0, y: self.y + 10.0};

        Shape::new(ctx)
            .triangle(&body_start, &body_next, &body_last)
            .fill(&self.color)
            .build();

    }

    fn render_wheels(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
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

    }

}

impl Create<Character> for Character {
    fn create() -> Character {
        Character {
            x: 80.0,
            y: window_height() - 150.0,
            color: "#2980b9".to_string(),
            direction: Direction::Left,
            wheel_rotation: 1.0,
            wheel_velocity: 5.0
        }
    }
}

impl Render for Character {

    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
        let text_point = Point{x: self.x + 50.0, y: self.y - 50.0};
        Shape::new(ctx).fill_text(&text_point, FIRE_TEXT, &self.color, "1em 'Atma'").build();

        if state.right {
            self.x += 5.0;
            self.wheel_velocity = 10.0;
        } else {
            self.wheel_velocity = 5.0;
        }

        if state.left {
            self.x -= 5.0;
        }

        self.render_body(ctx, state);
        self.render_head(ctx, state);
        self.render_wheels(ctx, state);


        self.wheel_rotation = self.wheel_rotation + self.wheel_velocity;
        if self.wheel_rotation > 360.0 {
            self.wheel_rotation = 1.0;
        }
    }
}


pub fn add_characters(render_stack: &mut Vec<Box<dyn Render>>) {

    let mut hero = Character::create();
    hero.set_direction(Direction::Left);

    render_stack.push(Box::new(Character::create()));
    render_stack.push(Box::new(Character::create()));
}