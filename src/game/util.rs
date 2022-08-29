use bevy::{prelude::*, math::Vec3Swizzles};

use rand::Rng;

pub fn get_random(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    (rng.gen::<f32>() - 0.5) * (max - min) + min
}

//https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm
const INSIDE: usize = 0b0000;
const LEFT: usize = 0b0001;
const RIGHT: usize = 0b0010;
const BOTTOM: usize = 0b0100;
const TOP: usize = 0b1000;

fn compute_out_code(
    pos: Vec2,
    rect_min: Vec2,
    rect_max: Vec2,
) -> usize {
    let mut code = INSIDE;

    if pos.x < rect_min.x {
        code |= LEFT;
    } else if pos.x > rect_max.x {
        code |= RIGHT;
    }
    if pos.y < rect_min.y {
        code |= BOTTOM;
    } else if pos.y > rect_max.y {
        code |= TOP;
    }

    code
}

pub fn check_collision_line_rectangle(
    mut line_start: Vec2,
    mut line_end: Vec2,
    rect_pos: Vec3,
    rect_scale: Vec2,
) -> bool {
    let rect_min = rect_pos.xy() - rect_scale / 2.0;
    let rect_max = rect_pos.xy() + rect_scale / 2.0;

    let mut code0 = compute_out_code(line_start, rect_min, rect_max);
    let mut code1 = compute_out_code(line_end, rect_min, rect_max);

    let mut accept = false;

    loop {
        if (code0 | code1) == INSIDE {
            accept = true;
            break;
        } else if code0 & code1 != INSIDE {
            break;
        } else {
            let code_out = if code1 > code0 {
                code1
            } else {
                code0
            };

            let (x, y) = if code_out & TOP != INSIDE {
                (line_start.x + (line_end.x - line_start.x) * (rect_max.y - line_start.y) / (line_end.y - line_start.y),
                rect_max.y)
            } else if code_out & BOTTOM != INSIDE {
                (line_start.x + (line_end.x - line_start.x) * (rect_min.y - line_start.y) / (line_end.y - line_start.y),
                rect_min.y)
            } else if code_out & RIGHT != INSIDE {
                (rect_max.x,
                line_start.y + (line_end.y - line_start.y) * (rect_max.x - line_start.x) / (line_end.x - line_start.x))
            } else if code_out & LEFT != INSIDE {
                (rect_min.x,
                line_start.y + (line_end.y - line_start.y) * (rect_min.x - line_start.x) / (line_end.x - line_start.x))
            } else {
                unreachable!();
            };

            if code_out == code0 {
                line_start = Vec2::new(x, y);
                code0 = compute_out_code(line_start, rect_min, rect_max);
            } else {
                line_end = Vec2::new(x, y);
                code1 = compute_out_code(line_end, rect_min, rect_max);
            }
        }
    }

    accept
}