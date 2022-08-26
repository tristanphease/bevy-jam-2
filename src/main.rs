use bevy::{prelude::*, sprite::Material2dPlugin};
use game::{
    camera::{camera_follow_player, setup_camera},
    game::setup_world,
    health_bar::{HealthBarMaterial, update_health_bars, update_health_bar_positions},
    input::{keyboard_input, mouse_input, ClickEvent},
    player::{setup_player, player_death::check_player_death, ShotSelected, player_shot::create_shot_on_click},
    shot::{setup_shots, collides_enemy, update_shots}, enemies::{insect_spawner::update_insect_spawners, insect_ai::move_insects, damage_player::{damage_player, update_damage_cooldowns}}, health::check_entity_death, waves::{waves::{check_wave_start, WaveInfo, StartWaveEvent, start_wave, EndWaveEvent}, insect_wave::check_insect_wave_end}, animate::animate_sprites,
};
use start_menu::{button_system, close_menu, setup_menu};

mod game;
mod start_menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    StartMenu,
    Game,
}

fn main() {
    App::new()
        .add_state(GameState::StartMenu)
        .add_plugins(DefaultPlugins)
        .add_event::<StartWaveEvent>()
        .add_event::<EndWaveEvent>()
        .add_event::<ClickEvent>()
        .init_resource::<Time>()
        .init_resource::<WaveInfo>()
        .init_resource::<ShotSelected>()
        .add_plugin(Material2dPlugin::<HealthBarMaterial>::default())
        .add_system_set(SystemSet::on_enter(GameState::StartMenu).with_system(setup_menu))
        .add_system_set(SystemSet::on_update(GameState::StartMenu).with_system(button_system))
        .add_system_set(SystemSet::on_exit(GameState::StartMenu).with_system(close_menu))
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(setup_camera)
                .with_system(setup_player)
                .with_system(setup_world)
                .with_system(setup_shots),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(camera_follow_player)
                .with_system(keyboard_input)
                .with_system(mouse_input)
                .with_system(update_shots)
                .with_system(collides_enemy)
                .with_system(check_player_death)
                .with_system(update_health_bars)
                .with_system(update_health_bar_positions)
                .with_system(move_insects)
                .with_system(update_insect_spawners)
                .with_system(check_entity_death)
                .with_system(check_wave_start)
                .with_system(start_wave)
                .with_system(damage_player)
                .with_system(update_damage_cooldowns)
                .with_system(check_insect_wave_end)
                .with_system(create_shot_on_click)
                .with_system(animate_sprites)
        )
        .run();
}
