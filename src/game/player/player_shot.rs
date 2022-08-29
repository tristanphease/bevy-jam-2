use bevy::{prelude::*, math::Vec3Swizzles};

use crate::game::{waves::waves::{EndWaveEvent, WAVE_NUM}, input::ClickEvent, components::{ShotSpawnOffset, Player}, hud::spell::{create_spell_ui, PlayerShotInputNumber, SpellUiCooldown, update_ui_spell_cooldown}};

use super::{basic_shot::create_basic_shot, PlayerShotsInfo, zap_spell::create_zap_spell};

pub const SHOT_TYPES: [Option<ShotType>; WAVE_NUM] = [
    Some(ShotType::Basic),
    None,
    Some(ShotType::Zap),
    None,
    None,
    None,
    None,
    None,
];

const BASIC_SHOT_COOLDOWN: f32 = 0.3;
const ZAP_COOLDOWN: f32 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShotType {
    Basic,
    Zap,
}

impl ShotType {
    fn cooldown(&self) -> f32 {
        match self {
            Self::Basic => BASIC_SHOT_COOLDOWN,
            Self::Zap => ZAP_COOLDOWN,
        }
    }
}

#[derive(Component)]
pub struct PlayerShot {
    cooldown_timer: Timer,
    shot_type: ShotType,
}

//adds a shot to the player's arsenal
pub fn add_shot_wave_end(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_shots_info: ResMut<PlayerShotsInfo>,
    mut end_wave_event_reader: EventReader<EndWaveEvent>,
) { 
    for end_wave_event in end_wave_event_reader.iter() {
        let shot_index = end_wave_event.get_waves_complete();
        let shot_type = &SHOT_TYPES[shot_index];

        if let Some(shot_type) = shot_type {
            //0 -> 1, 2 -> 2, 4 -> 3, 6 -> 4
            let input_number = shot_index / 2 + 1;

            add_player_shot(
                &mut commands, 
                &asset_server, 
                &mut player_shots_info, 
                *shot_type, 
                input_number,
            );
        }
    }
}

pub fn add_player_shot(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_shots_info: &mut ResMut<PlayerShotsInfo>,
    new_shot_type: ShotType,
    input_number: usize,
) {
    player_shots_info.shots_enabled.push(new_shot_type);

    create_spell_ui(commands, asset_server, new_shot_type, input_number, true);

    commands
        .spawn()
        .insert(PlayerShot {
            cooldown_timer: Timer::from_seconds(new_shot_type.cooldown(), false),
            shot_type: new_shot_type,
        })
        .insert(PlayerShotInputNumber(input_number));
}

//creates a player shot
pub fn create_shot_on_click(
    mut click_reader: EventReader<ClickEvent>,
    player_query: Query<(&Transform, &ShotSpawnOffset), With<Player>>,
    windows: Res<Windows>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    shot_selected: Res<PlayerShotsInfo>,
    mut player_shot_query: Query<&mut PlayerShot>,
) {
    for click_event in click_reader.iter() {
        for mut player_shot in player_shot_query.iter_mut() {
            if player_shot.shot_type == shot_selected.selected_shot_type {
                if player_shot.cooldown_timer.finished() {
                    let (player_trans, shot_offset) = player_query.get_single().unwrap();
                    let window = windows.get_primary().unwrap();

                    let x_diff = click_event.position.x - window.width() / 2.0 - shot_offset.0.x;
                    let y_diff = click_event.position.y - window.height() / 2.0 - shot_offset.0.y;

                    let angle = f32::atan2(y_diff, x_diff);
                    let position = player_trans.translation.xy() + shot_offset.0;

                    player_shot.cooldown_timer.reset();

                    match shot_selected.selected_shot_type {
                        ShotType::Basic => {
                            create_basic_shot(
                                &mut commands, 
                                &asset_server,
                                &mut texture_atlases,
                                position, 
                                angle, 
                            );
                        },
                        ShotType::Zap => {
                            create_zap_spell(
                                &mut commands, 
                                &asset_server, 
                                position, 
                                angle
                            );
                        },
                    }
                }
            }
        }
    }
}

pub fn update_player_shot_cooldowns(
    time: Res<Time>,
    mut query: Query<(&mut PlayerShot, &PlayerShotInputNumber)>,
    mut cooldown_query_ui: Query<(&mut Style, &PlayerShotInputNumber), With<SpellUiCooldown>>
) {
    for (mut player_shot, input_num) in query.iter_mut() {
        player_shot.cooldown_timer.tick(time.delta());

        let cooldown_timer_fraction = player_shot.cooldown_timer.percent();

        update_ui_spell_cooldown(
            &mut cooldown_query_ui,
            **input_num,
            cooldown_timer_fraction,
        );
    }
}