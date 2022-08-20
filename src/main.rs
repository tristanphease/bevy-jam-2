use bevy::prelude::*;
use game::{
    camera::{camera_follow_player, setup_camera},
    game::setup_world,
    input::keyboard_input,
    player::setup_player,
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
        .add_system_set(SystemSet::on_enter(GameState::StartMenu).with_system(setup_menu))
        .add_system_set(SystemSet::on_update(GameState::StartMenu).with_system(button_system))
        .add_system_set(SystemSet::on_exit(GameState::StartMenu).with_system(close_menu))
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(setup_camera)
                .with_system(setup_player)
                .with_system(setup_world),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(camera_follow_player)
                .with_system(keyboard_input),
        )
        .run();
}
