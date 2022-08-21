use bevy::prelude::*;

use super::{components::Player, shot::{create_shot, ShotResource}};

const SPEED: f32 = 200.0;

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_trans = player_query.get_single_mut().expect("Couldn't find player");
    
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
}

pub fn mouse_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    player_query: Query<&Transform, With<Player>>,
    shot_res: Res<ShotResource>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(mouse_pos) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {
            let x_diff = mouse_pos.x - window.width()/2.0;
            let y_diff = mouse_pos.y - window.height()/2.0;

            let angle = f32::atan2(y_diff, x_diff);

            let player_trans = player_query.get_single().unwrap();

            create_shot(&mut commands, &player_trans.translation, angle, shot_res.mesh.clone(), shot_res.material.clone());
        }
    }
}
