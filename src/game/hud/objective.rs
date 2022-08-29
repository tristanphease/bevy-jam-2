use bevy::prelude::*;

use crate::game::waves::waves::{WaveType, WaveInfo, EndWaveEvent};

use super::CALIBRI_FONT_PATH;

const OBJECTIVE_BORDER_COLOUR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const BACKGROUND_COLOUR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const OBJECTIVE_TEXT_COLOUR: Color = Color::BLACK;

#[derive(Component)]
pub struct ObjectiveText {
    objective_num: usize,
}

#[derive(Component)]
pub struct ObjectiveNode;

pub fn create_objective_ui_start_wave(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    wave_type: WaveType,
) {
    let image_handle: Handle<Image> = asset_server.load(wave_type.get_objective_img_path());

    let objective_num = wave_type.drops_needed();
    let objective_string = format!("{0}/{1}", 0, objective_num);

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(70.0), Val::Px(70.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            },
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        color: OBJECTIVE_BORDER_COLOUR.into(),
        ..default()
    })
    .insert(ObjectiveNode)
    .with_children(|parent| {

        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                ..default()
            },
            color: BACKGROUND_COLOUR.into(),
            ..default()
        });

        parent.spawn_bundle(ImageBundle {
            style: Style {
                size: Size {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0), 
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            image: image_handle.into(),
            ..default()
        });

        parent.spawn_bundle(TextBundle {
            text: Text::from_section(
                objective_string, 
                TextStyle { 
                    font: asset_server.load(CALIBRI_FONT_PATH), 
                    font_size: 30.0, 
                    color: OBJECTIVE_TEXT_COLOUR,
                }
            ),
            style: Style {
                position: UiRect {
                    left: Val::Px(15.0),
                    bottom: Val::Px(-20.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(ObjectiveText {
            objective_num,
        });
    });
}

pub fn update_objective_text(
    mut query: Query<(&mut Text, &ObjectiveText)>,
    wave_info: Res<WaveInfo>,
) {
    for (mut text, objective) in query.iter_mut() {
        let progress = wave_info.get_progress();
        text.sections[0].value = format!("{0}/{1}", progress, objective.objective_num);
    }
}

//on deposit
pub fn delete_objective_on_wave_end(
    mut commands: Commands,
    query: Query<Entity, With<ObjectiveNode>>,
    mut end_wave_reader: EventReader<EndWaveEvent>,
) {
    for _ in end_wave_reader.iter() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
    
}