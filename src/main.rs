use bevy::{prelude::*, sprite::Material2dPlugin};
use game::{
    camera::{camera_follow_player, setup_camera},
    game::{setup_world, despawn_all},
    health_bar::{HealthBarMaterial, update_health_bars, update_health_bar_positions},
    input::{keyboard_input, mouse_input, ClickEvent},
    player::{setup_player, player_death::check_player_death, PlayerShotsInfo, player_shot::{create_shot_on_click, update_player_shot_cooldowns, add_shot_wave_end}},
    shot::{collides_enemy, update_shots}, enemies::{insect_spawner::update_insect_spawners, insect_ai::move_insects, damage_player::{damage_player, update_damage_cooldowns}, digger::{update_digger_warnings, update_diggers}, dragon_ai::update_dragons, cat::update_cat}, entity_death::{check_entity_death}, waves::{waves::{check_wave_start, WaveInfo, StartWaveEvent, start_wave, EndWaveEvent}, digger_wave::{DiggerResource, create_digger_system}, check_wave_end::check_deposit_cauldron}, animate::animate_sprites, hud::{spell::update_ui_spell_borders, objective::{update_objective_text, delete_objective_on_wave_end}}, drops::check_player_collect::check_player_drops, cauldron::{update_cauldron, update_bubbles},
};
use game_over_menu::{setup_game_over_menu, game_over_button_system, close_game_over_menu};
use start_menu::{button_system, close_menu, setup_menu};

mod game;
mod start_menu;
mod game_over_menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    StartMenu,
    Game,
    GameOver,
}

fn main() {
    App::new()
        .add_state(GameState::StartMenu)
        .add_plugins(DefaultPlugins)
        .add_event::<StartWaveEvent>()
        .add_event::<EndWaveEvent>()
        .add_event::<ClickEvent>()
        .insert_resource(ClearColor(Color::GRAY))
        .init_resource::<Time>()
        .init_resource::<WaveInfo>()
        .init_resource::<PlayerShotsInfo>()
        .init_resource::<DiggerResource>()
        .add_plugin(Material2dPlugin::<HealthBarMaterial>::default())
        .add_system_set(SystemSet::on_enter(GameState::StartMenu).with_system(setup_menu))
        .add_system_set(SystemSet::on_update(GameState::StartMenu).with_system(button_system))
        .add_system_set(SystemSet::on_exit(GameState::StartMenu).with_system(close_menu))
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(setup_camera)
                .with_system(setup_player)
                .with_system(setup_world)
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
                .with_system(create_shot_on_click)
                .with_system(animate_sprites)
                .with_system(update_ui_spell_borders)
                .with_system(update_player_shot_cooldowns)
                .with_system(check_player_drops)
                .with_system(update_objective_text)
                .with_system(create_digger_system)
                .with_system(update_digger_warnings)
                .with_system(update_diggers)
                .with_system(check_deposit_cauldron)
                .with_system(delete_objective_on_wave_end)
                .with_system(add_shot_wave_end)
                .with_system(update_dragons)
                .with_system(update_cat)
                .with_system(update_cauldron)
                .with_system(update_bubbles)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Game)
                .with_system(despawn_all)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(setup_game_over_menu)
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver).with_system(game_over_button_system)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver).with_system(close_game_over_menu)
        )
        .run();
}
