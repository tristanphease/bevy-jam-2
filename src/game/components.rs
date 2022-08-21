use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Hitbox(pub Vec2);

/// Blocks player from moving on it
#[derive(Component)]
pub struct CollidesPlayer;

#[derive(Component)]
pub struct Direction(pub f32);
