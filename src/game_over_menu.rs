use bevy::prelude::*;

use crate::GameState;

//used for cleanup
#[derive(Component)]
pub struct GameOverMenuComponent;

pub fn setup_game_over_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    _game_state: Res<State<GameState>>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(GameOverMenuComponent);

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(70.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor(Color::rgb(0.5, 0.5, 0.5)),
            ..default()
        })
        .insert(GameOverMenuComponent)
        .with_children(|button| {
            button
                .spawn_bundle(TextBundle::from_section(
                    "Play Again",
                    TextStyle {
                        font: asset_server.load("fonts/calibri.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                    },
                ))
                .insert(GameOverMenuComponent);
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

pub fn close_game_over_menu(mut commands: Commands, menu_entity_query: Query<Entity, With<GameOverMenuComponent>>) {
    for entity_id in menu_entity_query.iter() {
        commands.entity(entity_id).despawn();
    }
}
