use bevy::prelude::*;
use rand::Rng;

use crate::game::{components::{Player, AnimationTimer, CollidesPlayer, Damage, Hitbox}, player::basic_shot::ShotSpeed, shot::Shot};

const CAT_SPEED: f32 = 70.0;

const CAT_SHOT_VARIANCE: f32 = 200.0;
const CAT_MOVE_VARIANCE: f32 = 400.0;

const CAT_SHOT_IMAGE_PATH: &str = "images/cat_shot.png";
const CAT_SHOT_TEXTURE_SIZE: Vec2 = Vec2::new(200.0, 200.0);
const CAT_SHOT_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const CAT_SHOT_SPEED: f32 = 400.0;

const CAT_SHOT_LENGTH: f32 = 10.0;
const CAT_SHOT_DAMAGE: f32 = 10.0;

#[derive(Component)]
pub struct Cat {
    pub target_position: Vec2,
    pub shoot_timer: Timer,
}

pub fn update_cat(
    mut commands: Commands,
    mut cat_query: Query<(&mut Transform, &mut Cat), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    let player_pos = player_query.single().translation;

    for (mut cat_trans, mut cat) in cat_query.iter_mut() {
        let cat_pos = cat_trans.translation;
        cat.shoot_timer.tick(time.delta());

        let mut rng = rand::thread_rng();
        if cat.shoot_timer.just_finished() {
            let target_position = Vec2::new(
                player_pos.x + (rng.gen::<f32>() - 0.5) * CAT_SHOT_VARIANCE,
                player_pos.y + (rng.gen::<f32>() - 0.5) * CAT_SHOT_VARIANCE,
            );

            make_cat_shot(
                &mut commands, 
                cat_pos, 
                target_position, 
                &asset_server, 
                &mut texture_atlases
            );
        }

        let angle = f32::atan2(cat.target_position.y - cat_pos.y, cat.target_position.x - cat_pos.x);
        cat_trans.translation += Vec3::new(
            f32::cos(angle) * CAT_SPEED * time.delta_seconds(),
            f32::sin(angle) * CAT_SPEED * time.delta_seconds(),
            0.0
        );

        if f32::hypot(cat_trans.translation.x - cat.target_position.x, cat_trans.translation.y - cat.target_position.y) < 50.0 {
            cat.target_position += 
                Vec2::X * (rng.gen::<f32>() - 0.5) * CAT_MOVE_VARIANCE +
                Vec2::Y * (rng.gen::<f32>() - 0.5) * CAT_MOVE_VARIANCE;
        }
    }
}

pub fn make_cat_shot(
    commands: &mut Commands,
    cat_pos: Vec3,
    target_pos: Vec2,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let texture_handler = asset_server.load(CAT_SHOT_IMAGE_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handler, CAT_SHOT_TEXTURE_SIZE, 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let angle = f32::atan2(target_pos.y - cat_pos.y, target_pos.x - cat_pos.x);

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: Some(CAT_SHOT_SIZE),
            ..default()
        },
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(cat_pos)
            .with_rotation(Quat::from_rotation_z(angle)),
        ..default()
    })
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
    .insert(Shot {
        timer: Timer::from_seconds(CAT_SHOT_LENGTH, false),
    })
    .insert(CollidesPlayer)
    .insert(ShotSpeed(CAT_SHOT_SPEED))
    .insert(Damage(CAT_SHOT_DAMAGE))
    .insert(Hitbox(CAT_SHOT_SIZE));
}