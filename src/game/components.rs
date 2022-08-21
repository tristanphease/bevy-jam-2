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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShotType {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct CollidesShot(pub ShotType);

#[derive(Component)]
pub struct Health(pub f32);
