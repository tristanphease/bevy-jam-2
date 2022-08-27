use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::game::{components::Player, enemies::digger::create_digger_warning};

use super::waves::{WaveInfo, WaveType};

pub const DIGGER_EYES_PATH: &str = "images/digger_eyes.png";
pub const NUM_DIGGER_EYES_NEEDED: usize = 10;

const MIN_DIGGER_TIMER: f32 = 0.2;
const MAX_DIGGER_TIMER: f32 = 3.0;

const DIGGER_VARIANCE: f32 = 300.0;

#[derive(Debug, Default)]
pub struct DiggerResource {
    pub next_digger_timer: Timer,
}

pub fn create_digger_system(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    player_query: Query<&Transform, With<Player>>,
    wave_info: Res<WaveInfo>,
    mut digger_resource: ResMut<DiggerResource>,
    time: Res<Time>,
) {
    if wave_info.wave_type_happening(WaveType::Diggers) {
        let player_pos = player_query.single().translation;
        
        digger_resource.next_digger_timer.tick(time.delta());

        if digger_resource.next_digger_timer.just_finished() {
            let mut rng = rand::thread_rng();
            let time_secs = rng.gen::<f32>() * (MAX_DIGGER_TIMER - MIN_DIGGER_TIMER) + MIN_DIGGER_TIMER;
            let time_duration = Duration::from_secs_f32(time_secs);
            digger_resource.next_digger_timer.set_duration(time_duration);

            let position = Vec3::new(
                player_pos.x + (rng.gen::<f32>() - 0.5) * DIGGER_VARIANCE,
                player_pos.y + (rng.gen::<f32>() - 0.5) * DIGGER_VARIANCE,
                1.0
            );

            create_digger_warning(
                &mut commands,
                &mut asset_server,
                &mut texture_atlases,
                position,
            );
        }
    }
}