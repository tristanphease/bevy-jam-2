use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Hitbox(pub Vec2);


#[derive(Component)]
pub struct CollidesPlayer;

#[derive(Component)]
pub struct CollidesEnemy;

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

#[derive(Component)]
pub struct ShotSpawnOffset(pub Vec2);

#[derive(Component)]
pub struct DamageCooldown {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Damage(pub f32);


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Debug, Clone, Copy)]
pub enum ItemDropType {
    GoldenInsectWings,
    DiggerEyes,
    CatTail,
}

#[derive(Component)]
pub struct DropsItemOnDeath {
    pub drop: ItemDropType,
}
