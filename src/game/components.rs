use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Hitbox(pub Vec2);

#[derive(Component)]
pub struct Collides;
