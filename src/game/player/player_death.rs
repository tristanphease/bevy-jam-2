use bevy::prelude::*;

use crate::{game::components::{Health, Player}, GameState, GameResultResource, GameResult};

pub fn check_player_death(
    mut commands: Commands,
    mut app_state: ResMut<State<GameState>>, 
    player_query: Query<&Health, With<Player>>
) {
    let player_health = player_query.single().current;

    if player_health <= 0.0 {
        //dead :(
        commands.insert_resource(GameResultResource {
            result: GameResult::Loss,
        });
        app_state.set(GameState::GameOver).unwrap();
    }
}
