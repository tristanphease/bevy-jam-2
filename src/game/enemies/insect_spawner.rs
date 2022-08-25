use bevy::prelude::*;
use rand::Rng;

use crate::game::{health_bar::{HealthBarMaterial, generate_health_bar, WithHealthBar}, components::{Hitbox, Health, Spawner}};

use super::insect::{spawn_insect, Insect};

const INSECT_SPAWNER_IMAGE_PATH: &str = "images/insect_spawner.png";
const INSECT_SPAWNER_TEXTURE_SIZE: Vec2 = Vec2::new(224.0, 257.0);
const INSECT_SPAWNER_SIZE: Vec2 = Vec2::new(200.0, 220.0);
const INSECT_SPAWNER_HEALTH: f32 = 100.0;
const SPAWNER_RANDOM_OFFSET: f32 = 100.0;

const MAX_INSECT_NUM: usize = 10;

#[derive(Component)]
pub struct InsectSpawner;

pub fn create_insect_spawner(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<HealthBarMaterial>>,
    pos: Vec2,
) {
    let texture_handler = asset_server.load(INSECT_SPAWNER_IMAGE_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handler, INSECT_SPAWNER_TEXTURE_SIZE, 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let health_bar = generate_health_bar(commands, meshes, materials, INSECT_SPAWNER_SIZE.y/2.0);

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: Some(INSECT_SPAWNER_SIZE),
            ..default()
        },
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0))
            .with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>())),
        ..default()
    })
    .insert(Hitbox(INSECT_SPAWNER_SIZE))
    .insert(Health::new(INSECT_SPAWNER_HEALTH))
    .insert(InsectSpawner)
    .insert(Spawner {
        timer: Timer::from_seconds(10.0, true),
    })
    .insert(WithHealthBar(health_bar));
}

pub fn update_insect_spawners(
    mut commands: Commands,
    mut query: Query<(&mut Spawner, &Transform)>,
    mut asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<HealthBarMaterial>>,
    time: Res<Time>,
    insect_query: Query<&Insect>,
) {
    for (mut spawner, trans) in query.iter_mut() {
        spawner.timer.tick(time.delta());

        if spawner.timer.finished() && insect_query.iter().count() < MAX_INSECT_NUM {
            let mut rng = rand::thread_rng();
            let x = trans.translation.x + (rng.gen::<f32>() - 0.5) * SPAWNER_RANDOM_OFFSET;
            let y = trans.translation.y + (rng.gen::<f32>() - 0.5) * SPAWNER_RANDOM_OFFSET;
            spawn_insect(&mut commands, &mut asset_server, &mut texture_atlases, &mut meshes, &mut materials, x, y);
        }
    }
}

