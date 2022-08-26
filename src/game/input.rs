use bevy::prelude::*;

use super::{
    components::Player,
    game::{GAME_WIDTH, GAME_HEIGHT},
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

pub struct ClickEvent {
    pub position: Vec2,
}

pub fn mouse_input(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut click_event_writer: EventWriter<ClickEvent>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(mouse_pos) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {

            click_event_writer.send(ClickEvent {
                position: mouse_pos,
            });
        }
    }
}
