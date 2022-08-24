use std::time::Duration;

use super::components::{CollidesShot, Direction, Health, Hitbox, ShotType};
use bevy::{
    prelude::{shape::Circle, *},
    sprite::{collide_aabb::collide, MaterialMesh2dBundle, Mesh2dHandle},
};

//these can be changed to values on the shot in the future
const SHOT_SPEED: f32 = 500.0;
const SHOT_RADIUS: f32 = 20.0;
const SHOT_DAMAGE: f32 = 2.0;
const SHOT_TIME_LENGTH: f32 = 10.0;

#[derive(Component)]
pub struct Shot {
    timer: Timer,
    shot_type: ShotType,
}

#[derive(Default)]
pub struct ShotResource {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

pub fn setup_shots(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let mesh_handle = meshes.add(Mesh::from(Circle::new(SHOT_RADIUS)));
    let material_handle = materials.add(ColorMaterial::from(Color::RED));

    commands.insert_resource(ShotResource {
        mesh: mesh_handle,
        material: material_handle,
    });
}

pub fn create_shot(
    commands: &mut Commands,
    position: &Vec3,
    angle: f32,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) {
    commands
        .spawn()
        .insert_bundle(MaterialMesh2dBundle {
            transform: Transform {
                translation: *position,
                ..default()
            },
            mesh: Mesh2dHandle(mesh),
            material,
            ..default()
        })
        .insert(Shot {
            timer: Timer::new(Duration::from_secs_f32(SHOT_TIME_LENGTH), false),
            shot_type: ShotType::Enemy,
        })
        .insert(Direction(angle))
        .insert(Hitbox(Vec2::new(SHOT_RADIUS * 2.0, SHOT_RADIUS * 2.0)));
}

pub fn update_shots(
    mut commands: Commands,
    mut shot_query: Query<(&mut Transform, &Direction, &mut Shot, Entity)>,
    time: Res<Time>,
) {
    for (mut trans, dir, mut shot, entity) in shot_query.iter_mut() {
        trans.translation.x += SHOT_SPEED * f32::cos(dir.0) * time.delta_seconds();
        trans.translation.y += SHOT_SPEED * f32::sin(dir.0) * time.delta_seconds();
        shot.timer.tick(time.delta());

        if shot.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn shot_collide(
    mut commands: Commands,
    shot_query: Query<(&Transform, &Hitbox, Entity, &Shot), With<Shot>>,
    mut collides_shot_query: Query<(&Transform, &Hitbox, &mut Health, &CollidesShot)>,
) {
    for (shot_trans, shot_hitbox, shot_entity, shot) in shot_query.iter() {
        for (object_trans, object_hitbox, mut object_health, object_collides) in
            collides_shot_query.iter_mut()
        {
            if shot.shot_type == object_collides.0
                && collide(
                    shot_trans.translation,
                    shot_hitbox.0,
                    object_trans.translation,
                    object_hitbox.0,
                )
                .is_some()
            {
                commands.entity(shot_entity).despawn();
                object_health.current -= SHOT_DAMAGE;
            }
        }
    }
}
