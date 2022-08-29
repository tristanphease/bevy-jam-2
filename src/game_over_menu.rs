use bevy::prelude::*;

use crate::{GameState, game::{waves::{waves::WaveInfo, digger_wave::DiggerResource}, player::PlayerShotsInfo, hud::CALIBRI_FONT_PATH}, GameResultResource, GameResult};

const WIN_TEXT: &str = "You Won!";
const LOSS_TEXT: &str = "You Lost.";

pub fn setup_game_over_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    game_result: Res<GameResultResource>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(TextBundle {
        style: Style {
            size: Size::new(Val::Px(300.0), Val::Px(70.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        text: Text::from_section(
            match game_result.result {
                GameResult::Win => WIN_TEXT,
                GameResult::Loss => LOSS_TEXT,
            },
            TextStyle { 
                font: asset_server.load(CALIBRI_FONT_PATH), 
                font_size: 40.0, 
                color: Color::BLACK, 
            }
        ),
        ..default()
    });

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(70.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor(Color::rgb(0.0, 0.0, 0.0)),
            ..default()
        })
        .with_children(|button| {
            button
                .spawn_bundle(TextBundle::from_section(
                    "Play Again",
                    TextStyle {
                        font: asset_server.load(CALIBRI_FONT_PATH),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                ));
        });
}

pub fn game_over_button_system(
    mut app_state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                app_state.set(GameState::StartMenu).unwrap();
            }
            _ => {}
        }
    }
}

pub fn close_game_over_menu(mut commands: Commands, menu_entity_query: Query<Entity>) {
    for entity_id in menu_entity_query.iter() {
        commands.entity(entity_id).despawn();
    }

    //also reset resources
    commands.insert_resource(WaveInfo::default());
    commands.insert_resource(PlayerShotsInfo::default());
    commands.insert_resource(DiggerResource::default());
}
