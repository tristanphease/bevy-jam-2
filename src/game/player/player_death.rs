use bevy::prelude::*;

use crate::game::components::{Health, Player};

pub fn check_player_death(mut commands: Commands, player_query: Query<&Health, With<Player>>) {
    let player_health = player_query.single().current;

    if player_health <= 0.0 {
        //dead, change state here
        println!("player dead");
    }
}
