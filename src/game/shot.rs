use bevy::{prelude::{*, shape::Circle}, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use super::components::Direction;

const SHOT_SPEED: f32 = 500.0;
const SHOT_RADIUS: f32 = 20.0;

#[derive(Component)]
pub struct Shot;

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
    commands.spawn()
        .insert_bundle(MaterialMesh2dBundle {
            transform: Transform { 
                translation: *position,
                ..default()
            },
            mesh: Mesh2dHandle(mesh),
            material,
            ..default()
        })
        .insert(Shot)
        .insert(Direction(angle));
}

pub fn move_shots(
    mut shot_query: Query<(&mut Transform, &Direction), With<Shot>>,
    time: Res<Time>,
) {
    for (mut trans, dir) in shot_query.iter_mut() {
        trans.translation.x += SHOT_SPEED * f32::cos(dir.0) * time.delta_seconds();
        trans.translation.y += SHOT_SPEED * f32::sin(dir.0) * time.delta_seconds();
    }
}

pub fn shot_collide(

) {

}