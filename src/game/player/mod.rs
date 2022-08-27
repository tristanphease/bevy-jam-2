use bevy::prelude::*;

use self::{player_shot::{ShotType, add_player_shot, SHOT_TYPES}, basic_shot::BASIC_SHOT_WAND_PATH};

use super::{
    components::{Health, Hitbox, Player, ShotSpawnOffset},
    health_bar::{generate_health_bar, HealthBarMaterial, WithHealthBar},
};

const PLAYER_TEXTURE_SIZE: Vec2 = Vec2::new(106.0, 153.0);
const PLAYER_SIZE: Vec2 = Vec2::new(100.0, 150.0);
const PLAYER_HITBOX: Vec2 = Vec2::new(80.0, 110.0);
const PLAYER_START_POS: Vec2 = Vec2::new(200.0, 0.0);
const PLAYER_HEALTH: f32 = 100.0;
const PLAYER_SHOT_OFFSET: Vec2 = Vec2::new(-40.0, 70.0);

const WIZARD_IMAGE_PATH: &str = "images/wizard.png";

pub mod player_death;
pub mod player_shot;
pub mod basic_shot;

pub struct PlayerShotsInfo {
    pub selected_shot_type: ShotType,
    pub selected_shot_number: usize,
    pub shots_enabled: Vec<ShotType>,
}

impl Default for PlayerShotsInfo {
    fn default() -> Self {
        Self {
            selected_shot_type: ShotType::Basic,
            selected_shot_number: 1,
            shots_enabled: Vec::new(),
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<HealthBarMaterial>>,
    mut player_shots_info: ResMut<PlayerShotsInfo>,
    windows: Res<Windows>,
) {
    let texture_handler = asset_server.load(WIZARD_IMAGE_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handler, PLAYER_TEXTURE_SIZE, 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let basic_shot_image = asset_server.load(BASIC_SHOT_WAND_PATH);

    let player_pos = Vec3::new(
        PLAYER_START_POS.x,
        PLAYER_START_POS.y,
        3.0,
    );

    let health_bar = generate_health_bar(&mut commands, &mut meshes, &mut materials, player_pos, PLAYER_SIZE.y/2.0);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_translation(player_pos),
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(Player)
        .insert(Hitbox(PLAYER_HITBOX))
        .insert(Health::new(PLAYER_HEALTH))
        .insert(WithHealthBar(health_bar))
        .insert(ShotSpawnOffset(PLAYER_SHOT_OFFSET))
        .with_children(|player| {
            player.spawn_bundle(SpriteBundle {
                texture: basic_shot_image,
                transform: Transform::from_xyz(PLAYER_SHOT_OFFSET.x, PLAYER_SHOT_OFFSET.y, 0.5),
                ..default()
            });
        });
    
    //setup shot for player
    add_player_shot(
        &mut commands, 
        &asset_server, 
        &mut player_shots_info, 
        SHOT_TYPES[0].unwrap(), 
        1, 
    );
}
