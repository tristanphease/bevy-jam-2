use bevy::{prelude::{*, shape::Circle}, sprite::Mesh2dHandle};
use rand::Rng;

use super::{waves::waves::EndWaveEvent, util::get_random};

#[derive(Component)]
pub struct PotionEffect;

#[derive(Component)]
pub struct Bubble {
    timer: Timer,
    speed: f32,
}

const BUBBLE_TIME_LENGTH: f32 = 1.0;

const BUBBLE_SPEED: Vec2 = Vec2::new(20.0, 50.0);

const BUBBLE_MIN_POS: Vec2 = Vec2::new(0.0, 80.0);
const BUBBLE_MAX_POS: Vec2 = Vec2::new(200.0, 170.0);

const BUBBLE_RAD: Vec2 = Vec2::new(10.0, 30.0);

pub fn update_cauldron(
    mut commands: Commands,
    mut cauldron_query: Query<(&mut Sprite, Entity), With<PotionEffect>>,
    mut end_wave_event_reader: EventReader<EndWaveEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (mut cauldron_sprite, cauldron_entity) = cauldron_query.single_mut();
    for _ in end_wave_event_reader.iter() {
        cauldron_sprite.color = random_colour();
    }

    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() < 0.1 {

        let mesh = meshes.add(Circle {
            radius: get_random(BUBBLE_RAD.x, BUBBLE_RAD.y),
            vertices: 16,
        }.into());

        let material = materials.add(ColorMaterial { 
            color: cauldron_sprite.color, 
            texture: None,
        });

        let pos = Vec3::new(
            get_random(BUBBLE_MIN_POS.x, BUBBLE_MAX_POS.x),
            get_random(BUBBLE_MIN_POS.y, BUBBLE_MAX_POS.y),
            1.0,
        );

        commands.entity(cauldron_entity)
            .add_children(|parent| {
                parent.spawn_bundle(ColorMesh2dBundle {
                    mesh: Mesh2dHandle(mesh),
                    material,
                    transform: Transform::from_translation(pos),
                    ..default()
                })
                .insert(Bubble {
                    timer: Timer::from_seconds(BUBBLE_TIME_LENGTH, false),
                    speed: get_random(BUBBLE_SPEED.x, BUBBLE_SPEED.y),
                });
            });
    }
}

pub fn update_bubbles(
    mut commands: Commands,
    mut bubble_query: Query<(&mut Transform, &mut Bubble, Entity)>,
    time: Res<Time>,
) {
    for (mut transform, mut bubble, entity) in bubble_query.iter_mut() {
        bubble.timer.tick(time.delta());

        transform.translation.y += bubble.speed * time.delta_seconds();

        if bubble.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn random_colour() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}