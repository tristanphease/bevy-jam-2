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
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Self {
            current: amount,
            maximum: amount,
        }
    }
}


#[derive(Component)]
pub struct Spawner {
    pub timer: Timer,
}