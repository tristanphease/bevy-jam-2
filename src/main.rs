use bevy::{prelude::*, sprite::Material2dPlugin};
use game::{
    camera::{camera_follow_player, setup_camera},
    game::setup_world,
    health_bar::{setup_health_bar, HealthBarMaterial},
    input::{keyboard_input, mouse_input},
    player::{check_player_death, setup_player},
    shot::{setup_shots, shot_collide, update_shots},
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
        .init_resource::<Time>()
        .add_plugin(Material2dPlugin::<HealthBarMaterial>::default())
        .add_startup_system(setup_health_bar)
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
                .with_system(shot_collide)
                .with_system(check_player_death),
        )
        .run();
}
