use bevy::prelude::*;

use super::components::Player;

const CAMERA_DIST: f32 = 1000.0;

pub fn setup_camera(mut commands: Commands) {
    //camera
    commands.spawn_bundle(Camera2dBundle::new_with_far(CAMERA_DIST));
}

pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_trans = player_query.single();
    let mut camera_trans = camera_query.single_mut();

    camera_trans.translation = Vec3 {
        x: player_trans.translation.x,
        y: player_trans.translation.y,
        z: camera_trans.translation.z,
    };
}
