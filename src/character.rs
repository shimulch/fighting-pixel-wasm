use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use js_sys::Math;
use std::f64::consts::PI;
use crate::core::{Render, Create, GameState, window_width, window_height, js_log, Controller};
use crate::utils::{Shape, Point};
use crate::text::FIRE_TEXT;

#[derive(PartialEq, Eq)]
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
    hero: bool
}


impl Character {

    fn create(direction:Direction, color: String, hero: bool) -> Character {

        let x = match direction {
            Direction::Left => window_width() - 80.0,
            Direction::Right => 80.0
        };

        let y = window_height() - 150.0;
        Character {
            x,
            y,
            color,
            direction,
            wheel_rotation: 1.0,
            wheel_velocity: 5.0,
            hero
        }
    }

    fn render_head(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
        let head_start = match self.direction {
            Direction::Right => Point{x: self.x, y: self.y - 1.0},
            Direction::Left => Point{x: self.x, y: self.y + 9.0}
        };
        let head_next = match self.direction {
            Direction::Right => Point{x: self.x + 20.0, y: head_start.y - 20.0},
            Direction::Left => Point{x: self.x + 20.0, y: head_start.y - 28.0}
        };
        let head_last = match self.direction {
            Direction::Right => Point{x: self.x + 40.0, y: head_start.y + 10.0},
            Direction::Left => Point{x: self.x + 40.0, y: head_start.y - 10.0}
        };

        Shape::new(ctx)
            .triangle(&head_start, &head_next, &head_last)
            .fill(&self.color)
            .build();

        let eye_wrapper_center = match self.direction {
            Direction::Right => Point{x: head_start.x + 20.0, y: head_start.y - 5.0},
            Direction::Left => Point{x: head_start.x + 20.0, y: head_start.y - 12.0}
        };
        Shape::new(ctx)
            .with_scale(0.8, 0.8, &eye_wrapper_center)
            .triangle(&head_start, &head_next, &head_last)
            .fill("#FFF")
            .build();


        let eye_center = Point{x: eye_wrapper_center.x, y: eye_wrapper_center.y};
        Shape::new(ctx)
            .with_scale(0.4, 0.4, &eye_center)
            .triangle(&head_start, &head_next, &head_last)
            .fill("#000")
            .build();

    }

    fn render_body(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
        let body_start = match self.direction {
            Direction::Right => Point { x: self.x, y: self.y },
            Direction::Left => Point { x: self.x, y: self.y + 10.0 }
        };
        let body_next = match self.direction {
            Direction::Right => Point {x: self.x + 20.0, y: self.y + 60.0},
            Direction::Left => Point {x: self.x + 15.0, y: self.y + 60.0}
        };
        let body_last = match self.direction {
            Direction::Right => Point {x: self.x + 40.0, y: self.y + 10.0},
            Direction::Left => Point { x: self.x + 40.0, y: self.y }
        };

        Shape::new(ctx)
            .triangle(&body_start, &body_next, &body_last)
            .fill(&self.color)
            .build();

    }

    fn render_wheels(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
        let sides = 10.0;
        let size =20.0;

        let wheel_left_center = Point{x: self.x - 6.0, y: self.y + 50.0};
        self.render_left_wheel(ctx, sides, size, &wheel_left_center);

        let wheel_right_center = Point{x: wheel_left_center.x + 50.0, y: wheel_left_center.y};
        self.render_right_wheel(ctx, sides, size, &wheel_right_center);
    }

    fn render_right_wheel(&mut self, ctx: &CanvasRenderingContext2d, sides: f64, size: f64, wheel_right_center: &Point) {
        if self.direction == Direction::Right {
            Shape::new(ctx)
                .with_rotation(&wheel_right_center, self.wheel_rotation, true)
                .with_scale(0.8, 0.8, &wheel_right_center)
                .draw_xgon(sides, size, &wheel_right_center)
                .stroke(&self.color, 2.0)
                .build();
            Shape::new(ctx)
                .with_rotation(&wheel_right_center, self.wheel_rotation, true)
                .with_scale(0.8, 0.8, &wheel_right_center)
                .draw_xgon(sides, size - 5.0, &wheel_right_center)
                .fill(&self.color)
                .build();
        } else {
            Shape::new(ctx)
                .with_rotation(&wheel_right_center, self.wheel_rotation, false)
                .draw_xgon(sides, size, &wheel_right_center)
                .stroke(&self.color, 2.0)
                .build();
            Shape::new(ctx)
                .with_rotation(&wheel_right_center, self.wheel_rotation, false)
                .draw_xgon(sides, size - 5.0, &wheel_right_center)
                .fill(&self.color)
                .build();
        }
    }

    fn render_left_wheel(&mut self, ctx: &CanvasRenderingContext2d, sides: f64, size: f64, wheel_left_center: &Point) {
        if self.direction == Direction::Right {
            Shape::new(ctx)
                .with_rotation(&wheel_left_center, self.wheel_rotation, true)
                .draw_xgon(sides, size, &wheel_left_center)
                .stroke(&self.color, 2.0)
                .build();
            Shape::new(ctx)
                .with_rotation(&wheel_left_center, self.wheel_rotation, true)
                .draw_xgon(sides, size - 5.0, &wheel_left_center)
                .fill(&self.color)
                .build();
        } else {
            Shape::new(ctx)
                .with_rotation(&wheel_left_center, self.wheel_rotation, false)
                .with_scale(0.8, 0.8, &wheel_left_center)
                .draw_xgon(sides, size, &wheel_left_center)
                .stroke(&self.color, 2.0)
                .build();
            Shape::new(ctx)
                .with_rotation(&wheel_left_center, self.wheel_rotation, false)
                .with_scale(0.8, 0.8, &wheel_left_center)
                .draw_xgon(sides, size - 5.0, &wheel_left_center)
                .fill(&self.color)
                .build();
        }
    }

    fn render_gun(&mut self, ctx: &CanvasRenderingContext2d) {
        let rect_p = match self.direction {
            Direction::Right => Point { x: self.x + 32.0, y: self.y + 15.0 },
            Direction::Left => Point { x: self.x - 12.0, y: self.y + 15.0 }
        };
        Shape::new(ctx)
            .rect(&rect_p, 20.0, 10.0)
            .fill(&self.color)
            .build();

        let nuzzle_p = match self.direction {
            Direction::Right => Point { x: rect_p.x + 25.0, y: rect_p.y + 5.0 },
            Direction::Left => Point { x: rect_p.x - 8.0, y: rect_p.y + 5.0 }
        };
        Shape::new(ctx)
            .with_rotation(&nuzzle_p, self.wheel_rotation, false)
            .draw_xgon(6.0, 10.0, &nuzzle_p)
            .fill(&self.color)
            .build();
        Shape::new(ctx)
            .with_scale(0.8, 0.8, &nuzzle_p)
            .with_rotation(&nuzzle_p, self.wheel_rotation, true)
            .draw_xgon(6.0, 10.0, &nuzzle_p)
            .fill("#FFF")
            .build();
    }
}

impl Render for Character {

    fn render(&mut self, ctx: &CanvasRenderingContext2d, state: &mut GameState) {
//        let text_point = Point{x: self.x + 50.0, y: self.y - 50.0};
//        Shape::new(ctx).fill_text(&text_point, FIRE_TEXT, &self.color, "1em 'Atma'").build();

        if self.hero {
            if state.right {
                self.x += 5.0;
                self.wheel_velocity = 10.0;
            } else {
                self.wheel_velocity = 5.0;
            }

            if state.left {
                self.x -= 5.0;
            }
        }

        self.render_body(ctx, state);
        self.render_head(ctx, state);
        self.render_wheels(ctx, state);
        self.render_gun(ctx);

        self.wheel_rotation = self.wheel_rotation + self.wheel_velocity;
        if self.wheel_rotation > 360.0 {
            self.wheel_rotation = 1.0;
        }
    }
}


pub fn add_characters(render_stack: &mut Vec<Box<dyn Render>>) {

    render_stack.push(Box::new(Character::create(Direction::Right, "#2980b9".to_string(), true)));
    render_stack.push(Box::new(Character::create(Direction::Left, "#2980b9".to_string(),false)));
}
