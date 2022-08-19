use bevy::prelude::*;

use crate::GameState;

//used for cleanup
#[derive(Component)]
pub struct MenuComponent;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default())
        .insert(MenuComponent);

    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(70.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).insert(MenuComponent)
    .with_children(|button| {
        button.spawn_bundle(TextBundle::from_section(
            "Start game", 
            TextStyle {
                font: asset_server.load("fonts/calibri.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.0, 0.0, 0.0),
            },
        )).insert(MenuComponent);
    });
}

pub fn button_system(
    mut app_state: ResMut<State<GameState>>,
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>)
    >,
) {
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                app_state.set(GameState::Game).unwrap();
            },
            _ => {},
        }
    }
}

pub fn close_menu(
    mut commands: Commands,
    menu_entity_query: Query<
        Entity, With<MenuComponent>,
    >,
) {
    for entity_id in menu_entity_query.iter() {
        commands.entity(entity_id).despawn();
    }
}