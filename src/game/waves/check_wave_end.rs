use bevy::{prelude::*, sprite::collide_aabb};

use crate::game::{components::{Player, Hitbox}, game::Cauldron};

use super::waves::{WaveInfo, EndWaveEvent};

pub fn check_deposit_cauldron(
    player_query: Query<(&GlobalTransform, &Hitbox), With<Player>>,
    cauldron_query: Query<(&GlobalTransform, &Hitbox), With<Cauldron>>,
    mut wave_info: ResMut<WaveInfo>,
    mut end_wave_event_writer: EventWriter<EndWaveEvent>,
) {
    let (player_trans, player_hitbox) = player_query.single();
    let (cauldron_trans, cauldron_hitbox) = cauldron_query.single();

    if collide_aabb::collide(
        player_trans.translation(), 
        **player_hitbox, 
        cauldron_trans.translation(), 
        **cauldron_hitbox,
    ).is_some() {
        if let Some(current_wave) = wave_info.get_current_wave() {
            if wave_info.get_progress() >= current_wave.drops_needed() {
                wave_info.end_wave();

                end_wave_event_writer.send(EndWaveEvent::new(
                    wave_info.num_waves_completed(),
                ));
            }
        }
    }
}