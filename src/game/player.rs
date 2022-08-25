use bevy::prelude::*;

use super::{
    components::{Health, Hitbox, Player, ShotSpawnOffset},
    health_bar::{generate_health_bar, HealthBarMaterial, WithHealthBar},
};

const PLAYER_TEXTURE_SIZE: Vec2 = Vec2::new(106.0, 153.0);
const PLAYER_SIZE: Vec2 = Vec2::new(100.0, 150.0);

const PLAYER_START_POS: Vec2 = Vec2::new(200.0, 0.0);

const PLAYER_HEALTH: f32 = 100.0;

const PLAYER_SHOT_OFFSET: Vec2 = Vec2::new(-30.0, 60.0);

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<HealthBarMaterial>>,
) {
    let texture_handler = asset_server.load("images/wizard.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handler, PLAYER_TEXTURE_SIZE, 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

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
        .insert(Hitbox(PLAYER_SIZE))
        .insert(Health::new(PLAYER_HEALTH))
        .insert(WithHealthBar(health_bar))
        .insert(ShotSpawnOffset(PLAYER_SHOT_OFFSET));
}

pub fn check_player_death(mut commands: Commands, player_query: Query<&Health, With<Player>>) {
    let player_health = player_query.single().current;

    if player_health <= 0.0 {
        //dead, change state here
        println!("player dead");
    }
}