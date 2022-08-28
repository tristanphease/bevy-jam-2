use bevy::prelude::*;

use crate::{game::components::{Health, Player}, GameState};

pub fn check_player_death(
    mut app_state: ResMut<State<GameState>>, 
    player_query: Query<&Health, With<Player>>
) {
    let player_health = player_query.single().current;

    if player_health <= 0.0 {
        //dead :(
        app_state.set(GameState::GameOver).unwrap();
    }
}
