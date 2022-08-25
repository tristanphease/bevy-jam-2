use std::f32::consts::PI;

use bevy::prelude::*;

use crate::game::{health_bar::HealthBarMaterial, enemies::insect_spawner::create_insect_spawner};

const INSECT_SPAWNER_NUM: usize = 3;
const INSECT_SPAWNER_RAD: f32 = 300.0;

pub fn start_insect_wave(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<HealthBarMaterial>>,
    position: Vec2,
) {
    //position is centre position
    for i in 0..INSECT_SPAWNER_NUM {
        let spawner_pos = Vec2::new(
            f32::cos(i as f32 / INSECT_SPAWNER_NUM as f32 * 2.0 * PI) * INSECT_SPAWNER_RAD + position.x,
            f32::sin(i as f32 / INSECT_SPAWNER_NUM as f32 * 2.0 * PI) * INSECT_SPAWNER_RAD + position.y,
        );

        create_insect_spawner(
            commands,
            asset_server,
            texture_atlases,
            meshes,
            materials,
            spawner_pos,
        );
    }
}