use bevy::prelude::*;

use crate::game::{components::Player, game::{GAME_WIDTH, GAME_HEIGHT}, health_bar::HealthBarMaterial, hud::objective::create_objective_ui_start_wave};

use super::{insect_wave::{start_insect_wave, GOLDEN_WINGS_PATH, INSECT_SPAWNER_NUM}, digger_wave::{DIGGER_EYES_PATH, NUM_DIGGER_EYES_NEEDED, DiggerResource}, dragon_wave::{DRAGON_COAL_PATH, NUM_DRAGONS_SPAWN, start_dragon_wave}, cat_wave::{CAT_DROP_IMAGE_PATH, CAT_NUM, create_cat}};

pub const WAVE_NUM: usize = 8;
const WAVE_SPOTS: [Vec2; WAVE_NUM] = get_wave_spots();
const PLAYER_DISTANCE_WAVE_START: f32 = 500.0;

const WAVES: [WaveType; WAVE_NUM] = [
    WaveType::Insects,
    WaveType::Diggers,
    WaveType::Dragons,
    WaveType::Cat,
    WaveType::Insects,
    WaveType::Insects,
    WaveType::Insects,
    WaveType::Insects,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveType {
    Insects,
    Diggers,
    Dragons,
    Cat,
}

impl WaveType {
    pub fn get_objective_img_path(&self) -> &str {
        match self {
            Self::Insects => GOLDEN_WINGS_PATH,
            Self::Diggers => DIGGER_EYES_PATH,
            Self::Dragons => DRAGON_COAL_PATH,
            Self::Cat => CAT_DROP_IMAGE_PATH,
        }
    }

    pub fn drops_needed(&self) -> usize {
        match self {
            Self::Insects => INSECT_SPAWNER_NUM,
            Self::Diggers => NUM_DIGGER_EYES_NEEDED,
            Self::Dragons => NUM_DRAGONS_SPAWN,
            Self::Cat => CAT_NUM,
        }
    }
}

pub struct StartWaveEvent {
    wave_type: WaveType,
    wave_position: Vec2,
}

pub struct EndWaveEvent {
    waves_complete: usize,
}

impl EndWaveEvent {
    pub fn new(waves_complete: usize) -> Self {
        Self { 
            waves_complete
        }
    }

    pub fn get_waves_complete(&self) -> usize {
        self.waves_complete
    }
}

#[derive(Debug)]
pub struct WaveInfo {
    current_wave: Option<usize>,
    current_wave_progress: usize,
    waves_completed: [bool; WAVE_NUM],
}

impl WaveInfo {
    pub fn start_wave(&mut self, index: usize) {
        self.current_wave = Some(index);
    }

    pub fn end_wave(&mut self) {
        if let Some(wave_index) = self.current_wave {
            self.current_wave = None;
            self.current_wave_progress = 0;
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

    pub fn wave_type_happening(&self, wave_type: WaveType) -> bool {
        if let Some(wave_index) = self.current_wave {
            return WAVES[wave_index] == wave_type;
        }
        false
    }

    pub fn add_drop(&mut self) {
        self.current_wave_progress += 1;
    }

    pub fn get_progress(&self) -> usize {
        self.current_wave_progress
    }

    pub fn get_current_wave(&self) -> Option<WaveType> {
        self.current_wave.map(|index| WAVES[index])
    }
}

impl Default for WaveInfo {
    fn default() -> Self {
        Self {
            current_wave: None,
            waves_completed: [false; WAVE_NUM],
            current_wave_progress: 0,
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
                let new_wave_index = wave_info.num_waves_completed();
                //start wave
                wave_info.start_wave(new_wave_index);
                let wave_type = WAVES[new_wave_index];
                wave_start_event_writer.send(
                    StartWaveEvent {
                        wave_type,
                        wave_position: *wave_pos,
                    }
                );

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
            },
            WaveType::Diggers => {
                commands.insert_resource(DiggerResource {
                    next_digger_timer: Timer::from_seconds(1.0, true),
                });
            },
            WaveType::Dragons => {
                start_dragon_wave(&mut commands, &asset_server, &mut texture_atlases, &mut meshes, &mut materials, event.wave_position);
            },
            WaveType::Cat => {
                create_cat(&mut commands, &asset_server, &mut texture_atlases, &mut meshes, &mut materials, event.wave_position);
            }
        }

        create_objective_ui_start_wave(
            &mut commands,
            &asset_server,
            event.wave_type,
        );
    }
}

const fn get_wave_spots() -> [Vec2; 8] {
    let mut spots = [Vec2::new(0.0, 0.0); WAVE_NUM];

    let mut index = 0;
    let mut x = -1;
    let mut y = -1;
    //can't use for loop in const fn :(
    while index < WAVE_NUM {
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