use bevy::prelude::*;
use start_menu::{setup_menu, button_system, close_menu};

mod start_menu;
mod game;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    StartMenu,
    Game,
}

fn main() {
    App::new()
        .add_state(GameState::StartMenu)
        .add_plugins(DefaultPlugins)
        .add_system_set(
            SystemSet::on_enter(GameState::StartMenu)
                .with_system(setup_menu)
        )
        .add_system_set(
            SystemSet::on_update(GameState::StartMenu)
                .with_system(button_system)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::StartMenu)
                .with_system(close_menu)
        )
        .run();
}

