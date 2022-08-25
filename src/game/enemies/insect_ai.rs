use bevy::{prelude::*, math::Vec3Swizzles};
use rand::Rng;

use crate::game::components::Player;

use super::insect::Insect;

#[derive(Component)]
pub struct InsectAI {
    pub target_position: Vec2,
}

const INSECT_DISTANCE_NEW_POS: f32 = 40.0;
const INSECT_SPEED: f32 = 40.0;

pub fn move_insects(
    mut insect_query: Query<(&mut Transform, &mut InsectAI), With<Insect>>,
    player_query: Query<&Transform, (With<Player>, Without<Insect>)>,
    time: Res<Time>,
) {
    for (mut insect_trans, mut insect_ai) in insect_query.iter_mut() {
        let insect_pos = insect_trans.translation.xy();
        let mut target_pos = insect_ai.target_position;
        if f32::hypot(insect_pos.x - target_pos.x, insect_pos.y - target_pos.y) < INSECT_DISTANCE_NEW_POS {
            let player_pos = player_query.single().translation.xy();
            insect_ai.target_position = pick_position(insect_pos, player_pos);
            target_pos = insect_ai.target_position;
        }

        //move insect towards ai position
        let angle = f32::atan2(target_pos.y - insect_pos.y, target_pos.x - insect_pos.x);
        insect_trans.translation.x += f32::cos(angle) * INSECT_SPEED * time.delta_seconds();
        insect_trans.translation.y += f32::sin(angle) * INSECT_SPEED * time.delta_seconds();
    }
}

//ai variables
const RAND_POS_DISTANCE: f32 = 500.0;
const RAND_POS_VARIANCE: f32 = 200.0;
const INSECT_DIST_GAIN: f32 = 300.0;

fn pick_position(
    insect_pos: Vec2,
    player_pos: Vec2,
) -> Vec2 {
    let mut rng = rand::thread_rng();
    if f32::hypot(insect_pos.x - player_pos.x, insect_pos.y - player_pos.y) < RAND_POS_DISTANCE {
        //pick random point around player

        Vec2::new(
            player_pos.x + rng.gen::<f32>() * RAND_POS_VARIANCE,
            player_pos.y + rng.gen::<f32>() * RAND_POS_VARIANCE,
        )
    } else {
        //pick point a certain distance from the insect towards the player

        let angle = f32::atan2(player_pos.y - insect_pos.y, player_pos.x - insect_pos.x);

        Vec2::new(
            f32::cos(angle) * INSECT_DIST_GAIN,
            f32::sin(angle) * INSECT_DIST_GAIN,
        )
    }
}