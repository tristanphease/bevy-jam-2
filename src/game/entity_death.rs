use bevy::{prelude::*};

use super::{components::{Health, Player, DropsItemOnDeath, ItemDropType}, health_bar::WithHealthBar, drops::{golden_insect_wings::create_golden_insect_wings, digger_eyes::create_digger_eyes, cat_tail::create_cat_tail}};

pub fn check_entity_death(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Health, Option<&WithHealthBar>, Option<&DropsItemOnDeath>, &GlobalTransform), Without<Player>>,
) {
    for (entity, health, health_bar_option, drops_item_option, transform) in query.iter() {
        if health.current <= 0.0 {
            commands.entity(entity).despawn_recursive();

            if let Some(health_bar) = health_bar_option {
                commands.entity(health_bar.0).despawn();
            }

            if let Some(drops_item) = drops_item_option {
                let position = transform.translation();
                match drops_item.drop {
                    ItemDropType::GoldenInsectWings => create_golden_insect_wings(
                        &mut commands, 
                        &asset_server, 
                        position,
                    ),
                    ItemDropType::DiggerEyes => create_digger_eyes(
                        &mut commands, 
                        &asset_server, 
                        position,
                    ),
                    ItemDropType::CatTail => create_cat_tail(
                        &mut commands, 
                        &asset_server, 
                        position,
                    ),
                }
            }
        }
    }
}