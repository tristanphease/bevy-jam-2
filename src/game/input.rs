use bevy::prelude::*;

use super::components::Player;

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
