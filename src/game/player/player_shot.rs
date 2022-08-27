use bevy::{prelude::*, math::Vec3Swizzles};

use crate::game::{waves::waves::{EndWaveEvent, WAVE_NUM}, input::ClickEvent, components::{ShotSpawnOffset, Player}};

use super::basic_shot::create_basic_shot;

const SHOT_TYPES: [Option<ShotType>; WAVE_NUM] = [
    Some(ShotType::Basic),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShotType {
    Basic,
}

#[derive(Component)]
pub struct PlayerShot;

//adds a shot to the player's arsenal
pub fn add_shot_wave_end(
    mut commands: Commands,
    mut end_wave_event_reader: EventReader<EndWaveEvent>,
) { 
    for end_wave_event in end_wave_event_reader.iter() {
        let shot_type = &SHOT_TYPES[end_wave_event.get_waves_complete()];

        if let Some(shot_type) = shot_type {

        }
    }
}

//creates a shot
pub fn create_shot_on_click(
    mut click_reader: EventReader<ClickEvent>,
    player_query: Query<(&Transform, &ShotSpawnOffset), With<Player>>,
    windows: Res<Windows>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for click_event in click_reader.iter() {
        let (player_trans, shot_offset) = player_query.get_single().unwrap();
        let window = windows.get_primary().unwrap();

        let x_diff = click_event.position.x - window.width() / 2.0 - shot_offset.0.x;
        let y_diff = click_event.position.y - window.height() / 2.0 - shot_offset.0.y;

        let angle = f32::atan2(y_diff, x_diff);
        let position = player_trans.translation.xy() + shot_offset.0;

        create_basic_shot(
            &mut commands, 
            &asset_server,
            &mut texture_atlases,
            position, 
            angle, 
        );
    }
}