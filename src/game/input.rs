use bevy::{prelude::*, math::Vec3Swizzles};

use super::{
    components::{Player, ShotSpawnOffset},
    shot::{create_shot, ShotResource}, game::{GAME_WIDTH, GAME_HEIGHT},
};

const SPEED: f32 = 200.0;

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_trans = player_query.single_mut();

    if keys.pressed(KeyCode::W) {
        player_trans.translation.y += SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::S) {
        player_trans.translation.y -= SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::A) {
        player_trans.translation.x -= SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::D) {
        player_trans.translation.x += SPEED * time.delta_seconds();
    }

    //clamp within game border
    //player_trans.translation.x = f32::clamp(player_trans.translation.x, -(GAME_WIDTH as f32)/2.0, (GAME_WIDTH as f32)/2.0);
    //player_trans.translation.y = f32::clamp(player_trans.translation.y, -(GAME_HEIGHT as f32)/2.0, (GAME_HEIGHT as f32)/2.0);
    player_trans.translation.x = f32::clamp(player_trans.translation.x, -(GAME_WIDTH as f32), GAME_WIDTH as f32);
    player_trans.translation.y = f32::clamp(player_trans.translation.y, -(GAME_HEIGHT as f32), GAME_HEIGHT as f32);
}

pub fn mouse_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    player_query: Query<(&Transform, &ShotSpawnOffset), With<Player>>,
    shot_res: Res<ShotResource>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(mouse_pos) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {

            let (player_trans, shot_offset) = player_query.get_single().unwrap();

            let x_diff = mouse_pos.x - window.width() / 2.0 - shot_offset.0.x;
            let y_diff = mouse_pos.y - window.height() / 2.0 - shot_offset.0.y;

            let angle = f32::atan2(y_diff, x_diff);

            

            create_shot(
                &mut commands,
                player_trans.translation.xy() + shot_offset.0,
                angle,
                shot_res.mesh.clone(),
                shot_res.material.clone(),
            );
        }
    }
}
