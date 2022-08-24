use bevy::prelude::*;

use crate::game::{components::Player, game::{GAME_WIDTH, GAME_HEIGHT}, health_bar::HealthBarMaterial};

use super::insect_wave::start_insect_wave;


const WAVE_SPOT_NUM: usize = 8;
const WAVE_SPOTS: [Vec2; WAVE_SPOT_NUM] = get_wave_spots();
const PLAYER_DISTANCE_WAVE_START: f32 = 500.0;

const WAVES: [WaveType; WAVE_SPOT_NUM] = [WaveType::Insects; WAVE_SPOT_NUM];

#[derive(Debug, Clone, Copy)]
enum WaveType {
    Insects,
}

pub struct StartWaveEvent {
    wave_type: WaveType,
    wave_position: Vec2,
}

pub struct WaveInfo {
    current_wave: Option<usize>,
    waves_completed: [bool; WAVE_SPOT_NUM],
}

impl WaveInfo {
    pub fn start_wave(&mut self, index: usize) {
        self.current_wave = Some(index);
    }

    pub fn end_wave(&mut self) {
        if let Some(wave_index) = self.current_wave {
            self.waves_completed[wave_index] = true;
        }
    }

    pub fn num_waves_completed(&self) -> usize {
        self.waves_completed.iter().filter(|x| **x).count()
    }

    pub fn wave_happening(&self) -> bool {
        self.current_wave.is_some()
    }

    pub fn completed_wave(&self, index: usize) -> bool {
        self.waves_completed[index]
    }
}

impl Default for WaveInfo {
    fn default() -> Self {
        Self {
            current_wave: None,
            waves_completed: [false; WAVE_SPOT_NUM],
        }
    }
}

pub fn check_wave_start(
    mut wave_info: ResMut<WaveInfo>,
    player_query: Query<&Transform, With<Player>>,
    mut wave_start_event_writer: EventWriter<StartWaveEvent>,
) {
    let player_pos = player_query.single().translation;

    if !wave_info.wave_happening() {
        for (wave_index, wave_pos) in WAVE_SPOTS.iter().enumerate() {
            if !wave_info.completed_wave(wave_index) &&
                f32::hypot(wave_pos.x - player_pos.x, wave_pos.y - player_pos.y) < PLAYER_DISTANCE_WAVE_START 
            {
                //start wave
                wave_info.start_wave(wave_index);
                let wave_type = WAVES[wave_index];
                wave_start_event_writer.send(
                    StartWaveEvent {
                        wave_type,
                        wave_position: *wave_pos,
                    }
                );

                dbg!(WAVE_SPOTS, player_pos, wave_pos);
                println!("started wave {wave_index}");

                break;
            }
        }
    }
}

pub fn start_wave(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<HealthBarMaterial>>,
    mut start_wave_event_reader: EventReader<StartWaveEvent>,
) {
    for event in start_wave_event_reader.iter() {
        match event.wave_type {
            WaveType::Insects => {
                start_insect_wave(&mut commands, &asset_server, &mut texture_atlases, &mut meshes, &mut materials, event.wave_position);
            }
        }
    }
}

const fn get_wave_spots() -> [Vec2; 8] {
    let mut spots = [Vec2::new(0.0, 0.0); WAVE_SPOT_NUM];

    let mut index = 0;
    let mut x = -1;
    let mut y = -1;
    //can't use for loop in const fn :(
    while index < WAVE_SPOT_NUM {
        if !(x == 0 && y == 0) {
            //also can't do floating point arithmetic, this is fine though
            spots[index] = Vec2::new(((x * GAME_WIDTH as i32) / 2) as f32, ((y * GAME_HEIGHT as i32) / 2) as f32);
            index += 1;
        }
        x += 1;
        if x > 1 {
            x = -1;
            y += 1;
        }
    }
    
    spots
}