use bevy::prelude::*;

use crate::game::{components::{Hitbox, CollidesEnemy, CollidesPlayer, Damage, AnimationTimer, Health, DropsItemOnDeath, ItemDropType}, health_bar::{generate_health_bar, HealthBarMaterial, WithHealthBar}};

const WARNING_TIME: f32 = 1.5;

const DIGGER_WARNING_PATH: &str = "images/warning_ground.png";
const DIGGER_WARNING_TEXTURE_SIZE: Vec2 = Vec2::new(200.0, 200.0);

const DIGGER_PATH: &str = "images/digger.png";
const DIGGER_TEXTURE_SIZE: Vec2 = Vec2::new(239.0, 112.0);
const DIGGER_SIZE: Vec2 = Vec2::new(100.0, 50.0);

const DIGGER_DAMAGE: f32 = 10.0;

//should die in one hit
const DIGGER_HEALTH: f32 = 0.1;

const DIGGER_STAGE_TIME: f32 = 0.5;

#[derive(Component, Deref, DerefMut)]
pub struct DiggerWarningTimer(pub Timer);

#[derive(Component)]
pub struct Digger {
    timer: Timer,
    digger_stage: usize,
}

pub fn create_digger_warning(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    position: Vec3,
) {
    let texture_handler = asset_server.load(DIGGER_WARNING_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handler, DIGGER_WARNING_TEXTURE_SIZE, 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(position)
            .with_scale(Vec3::new(0.5, 0.5, 1.0)),
        ..default()
    })
    .insert(DiggerWarningTimer(Timer::from_seconds(WARNING_TIME, false)))
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

pub fn update_digger_warnings(
    mut commands: Commands,
    mut query: Query<(&mut DiggerWarningTimer, Entity, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<HealthBarMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    for (mut timer, entity, trans) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            commands.entity(entity).despawn();
            
            create_digger(
                &mut commands, 
                trans.translation,
                &asset_server,
                &mut texture_atlases,
                &mut meshes,
                &mut materials,
            );
        }
    }
}

pub fn create_digger(
    commands: &mut Commands,
    position: Vec3,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<HealthBarMaterial>>,
) {
    let texture_handler = asset_server.load(DIGGER_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handler, DIGGER_TEXTURE_SIZE, 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let health_bar = generate_health_bar(commands, meshes, materials, position, DIGGER_TEXTURE_SIZE.y/2.0);

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: Some(DIGGER_SIZE),
            ..default()
        },
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(position),
        ..default()
    })
    .insert(Hitbox(DIGGER_SIZE))
    .insert(CollidesEnemy)
    .insert(CollidesPlayer)
    .insert(Damage(DIGGER_DAMAGE))
    .insert(Health::new(DIGGER_HEALTH))
    .insert(WithHealthBar(health_bar))
    .insert(Digger {
        timer: Timer::from_seconds(DIGGER_STAGE_TIME, true),
        digger_stage: 0,
    })
    .insert(DropsItemOnDeath { drop: ItemDropType::DiggerEyes });
}

pub fn update_diggers(
    mut commands: Commands,
    mut digger_query: Query<(&mut Digger, &mut TextureAtlasSprite, &WithHealthBar, Entity)>,
    time: Res<Time>,
) {
    for (mut digger, mut sprite, health_bar, entity) in digger_query.iter_mut() {
        digger.timer.tick(time.delta());

        if digger.timer.just_finished() {
            digger.digger_stage += 1;

            match digger.digger_stage {
                0 => {},
                index @ 1..=2 => {
                    sprite.index = index;
                },
                _ => {
                    commands.entity(entity).despawn();
                    commands.entity(health_bar.0).despawn();
                },
            }
        }
    }
}