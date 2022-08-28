/* use bevy::prelude::*;

pub fn rotated_rect_collide(
    pos_a: Vec3,
    scale_a: Vec2,
    rotation_a: f32,
    pos_b: Vec3,
    scale_b: Vec2,
    rotation_b: f32,
) -> bool {


    false
} */

use rand::Rng;

pub fn get_random(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    (rng.gen::<f32>() - 0.5) * (max - min) + min
}