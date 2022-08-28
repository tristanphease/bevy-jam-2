use std::time::Duration;

use bevy::prelude::*;

use crate::game::components::{Player, Hitbox, AnimationTimer, CollidesPlayer, DamageCooldown, Damage};

const DRAGON_SPEED: f32 = 100.0;
const SHOOTING_TIME: f32 = 2.0;
const MOVING_TIME: f32 = 5.0;

const DRAGON_FIRE_IMAGE_PATH: &str = "images/fire.png";
const DRAGON_FIRE_TEXTURE_SIZE: Vec2 = Vec2::new(200.0, 200.0);
const DRAGON_FIRE_SIZE: Vec2 = Vec2::new(200.0, 200.0);

const DRAGON_FIRE_COOLDOWN_TIME: f32 = 1.0;
const DRAGON_FIRE_DAMAGE: f32 = 20.0;
const DRAGON_FIRE_OFFSET: Vec2 = Vec2::new(-170.0, 0.0);

#[derive(Debug, Clone, Copy)]
pub enum DragonState {
    Moving,
    Shooting,
}

#[derive(Component)]
pub struct Dragon {
    pub state: DragonState,
    pub timer: Timer,
}

pub fn update_dragons(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Dragon, Entity), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    let player_pos = player_query.single().translation;
    for (mut dragon_trans, mut dragon_info, dragon_entity) in query.iter_mut() {
        let dragon_pos = dragon_trans.translation;
        dragon_info.timer.tick(time.delta());
        match dragon_info.state {
            DragonState::Moving => {
                let angle = f32::atan2(dragon_pos.y - player_pos.y, dragon_pos.x - player_pos.x);
                dragon_trans.rotation = Quat::from_rotation_z(angle);

                let forward = -dragon_trans.local_x();
                dragon_trans.translation += forward * DRAGON_SPEED * time.delta_seconds();

                if dragon_info.timer.just_finished() {
                    dragon_info.state = DragonState::Shooting;
                    dragon_info.timer.reset();
                    dragon_info.timer.set_duration(Duration::from_secs_f32(SHOOTING_TIME));

                    create_dragon_fire(
                        &mut commands,
                        dragon_entity,
                        &asset_server,
                        &mut texture_atlases,
                    );
                }
            }
            DragonState::Shooting => {
                if dragon_info.timer.just_finished() {
                    dragon_info.state = DragonState::Moving;
                    dragon_info.timer.reset();
                    dragon_info.timer.set_duration(Duration::from_secs_f32(MOVING_TIME));

                    delete_dragon_fire(
                        &mut commands,
                        dragon_entity,
                    );
                }
            }
        }
    }
}

fn create_dragon_fire(
    commands: &mut Commands,
    dragon: Entity,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let texture_handler = asset_server.load(DRAGON_FIRE_IMAGE_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handler, DRAGON_FIRE_TEXTURE_SIZE, 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.entity(dragon)
        .with_children(|dragon| {
            dragon.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(DRAGON_FIRE_SIZE),
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_translation(Vec3::new(DRAGON_FIRE_OFFSET.x, DRAGON_FIRE_OFFSET.y, 3.0)),
                ..default()
            })
            .insert(Hitbox(DRAGON_FIRE_SIZE))
            .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
            .insert(CollidesPlayer)
            .insert(DamageCooldown { timer: Timer::from_seconds(DRAGON_FIRE_COOLDOWN_TIME, false) })
            .insert(Damage(DRAGON_FIRE_DAMAGE));
        });
}

fn delete_dragon_fire(
    commands: &mut Commands,
    dragon: Entity,
) {
    commands.entity(dragon).despawn_descendants();
}