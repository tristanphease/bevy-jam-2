use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

use crate::game::{health_bar::HealthBarMaterial, enemies::dragon::create_dragon};

const DISTANCE_SPAWN: f32 = 200.0;
pub const NUM_DRAGONS_SPAWN: usize = 2;

pub const DRAGON_COAL_PATH: &str = "images/dragon_coal.png";

pub fn start_dragon_wave(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<HealthBarMaterial>>,
    position: Vec2,
) {
    let angle = rand::thread_rng().gen::<f32>() * PI * 2.0;

    for i in 0..NUM_DRAGONS_SPAWN {
        let new_pos = Vec2::new(
            f32::cos(angle + (i as f32 / NUM_DRAGONS_SPAWN as f32) * 2.0 * PI) * DISTANCE_SPAWN + position.x,
            f32::sin(angle + (i as f32 / NUM_DRAGONS_SPAWN as f32) * 2.0 * PI) * DISTANCE_SPAWN + position.y,
        );

        create_dragon(
            commands, 
            asset_server, 
            texture_atlases, 
            meshes, 
            materials, 
            new_pos,
        );
    }
}