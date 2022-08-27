use bevy::prelude::*;

use crate::game::waves::waves::WaveType;

use super::CALIBRI_FONT_PATH;

const OBJECTIVE_BORDER_COLOUR: Color = Color::ALICE_BLUE;

#[derive(Component)]
pub struct ObjectiveText {
    objective_num: usize,
}

pub fn create_objective_ui_start_wave(
    commands: &mut Commands,
    asset_server: &AssetServer,
    wave_type: WaveType,
    objective_num: usize,
) {
    let image_handle: Handle<Image> = asset_server.load(wave_type.get_objective_img_path());

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
            ..default()
        },
        color: OBJECTIVE_BORDER_COLOUR.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(ImageBundle {
            style: Style {
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
                    color: Color::BLACK
                }
            ),
            ..default()
        })
        .insert(ObjectiveText {
            objective_num,
        });
    });
}

pub fn update_objective_text(
    mut commands: Commands,
    mut query: Query<(&mut Text, &ObjectiveText)>,
) {
    
}