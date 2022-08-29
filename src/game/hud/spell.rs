use bevy::prelude::*;

use crate::game::player::{player_shot::ShotType, basic_shot::BASIC_SHOT_WAND_PATH, PlayerShotsInfo, zap_spell::ZAP_IMAGE_PATH};

use super::CALIBRI_FONT_PATH;

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct PlayerShotInputNumber(pub usize);

#[derive(Component)]
pub struct SpellUiBorder;

#[derive(Component, Clone, Copy)]
pub struct SpellUiCooldown;

const BORDER_SELECTED_COLOUR: Color = Color::BLACK;
const BORDER_NON_SELECTED_COLOUR: Color = Color::DARK_GRAY;
const COOLDOWN_COLOUR: Color = Color::DARK_GRAY;
const BACKGROUND_COLOUR: Color = Color::BLACK;
const INPUT_TEXT_COLOR: Color = Color::WHITE;

pub fn create_spell_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    shot_type: ShotType,
    input_number: usize,
    selected: bool,
) {

    let image_handle: Handle<Image> = match shot_type {
        ShotType::Basic => {
            asset_server.load(BASIC_SHOT_WAND_PATH)
        }
        ShotType::Zap => {
            asset_server.load(ZAP_IMAGE_PATH)
        },
    };

    let border_colour = if selected {
        BORDER_SELECTED_COLOUR
    } else {
        BORDER_NON_SELECTED_COLOUR
    };
    
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(70.0), Val::Px(70.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(10.0 + (input_number as f32 - 1.0) * 90.0),
                top: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        color: border_colour.into(),
        ..default()
    })
    .insert(PlayerShotInputNumber(input_number))
    .insert(SpellUiBorder)
    .with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                position: UiRect { 
                    left: Val::Px(10.0), 
                    bottom: Val::Px(10.0), 
                    ..default() 
                },
                ..default()
            },
            color: BACKGROUND_COLOUR.into(),
            ..default()
        })
        .with_children(|parent| {

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(0.0)),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                color: COOLDOWN_COLOUR.into(),
                ..default()
            })
            .insert(SpellUiCooldown)
            .insert(PlayerShotInputNumber(input_number));

            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    input_number.to_string(), 
                    TextStyle {
                        font: asset_server.load(CALIBRI_FONT_PATH),
                        font_size: 30.0,
                        color: INPUT_TEXT_COLOR,
                    }
                ),
                style: Style {
                    size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(0.0),
                        top: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
    
            parent.spawn_bundle(ImageBundle {
                image: image_handle.into(),
                style: Style {
                    size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                    position: UiRect {
                        left: Val::Px(5.0),
                        bottom: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
        });
    });
}

pub fn update_ui_spell_borders(
    mut border_query: Query<(&mut UiColor, &PlayerShotInputNumber), With<SpellUiBorder>>,
    selected_shot: Res<PlayerShotsInfo>,
) {
    for (mut border_colour, input_num) in border_query.iter_mut() {
        *border_colour = if selected_shot.selected_shot_number == **input_num {
            BORDER_SELECTED_COLOUR
        } else {
            BORDER_NON_SELECTED_COLOUR
        }.into();
    }
}

pub fn update_ui_spell_cooldown(
    cooldown_query_ui: &mut Query<(&mut Style, &PlayerShotInputNumber), With<SpellUiCooldown>>,
    input_num: usize,
    cooldown_fraction: f32,
) {
    for (mut style, input_num_ui) in cooldown_query_ui.iter_mut() {
        if **input_num_ui == input_num {
            style.size.height = Val::Percent(cooldown_fraction * 100.0);
        }
    }
}