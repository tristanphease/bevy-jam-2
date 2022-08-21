use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub fn get_health_bar(commands: &mut Commands, health_bar: Res<HealthBarHandles>) -> Entity {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(health_bar.mesh_handle.clone()),
            material: health_bar.material_handle.clone(),
            transform: Transform::from_xyz(0.0, -10.0, 1.0).with_scale(Vec3::new(50.0, 50.0, 1.0)),
            ..default()
        })
        .id()
}

const BAR_WIDTH: i32 = 4;
const BAR_HEIGHT: i32 = 1;
const ANGLE_NUM: u32 = 6;
const OUTER_RAD: f32 = BAR_HEIGHT as f32 - 0.1;
const INNER_RAD: f32 = BAR_HEIGHT as f32 - 0.2;

#[derive(Default)]
pub struct HealthBarHandles {
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
}

pub fn setup_health_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh_handle = meshes.add(create_bar());
    let material_handle = materials.add(ColorMaterial::from(Color::ORANGE));

    commands.insert_resource(HealthBarHandles {
        mesh_handle,
        material_handle,
    });
}

fn create_bar() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut vertices = Vec::new();
    let x = -BAR_WIDTH as f32 / 2.0 + 1.0;
    let y = 0.0;
    let start_angle = FRAC_PI_2;
    add_vertices(&mut vertices, x, y, OUTER_RAD, start_angle, true);
    add_vertices(&mut vertices, x, y, INNER_RAD, start_angle, true);

    let x = BAR_WIDTH as f32 / 2.0 - 1.0;
    add_vertices(&mut vertices, x, y, OUTER_RAD, start_angle, false);
    add_vertices(&mut vertices, x, y, INNER_RAD, start_angle, false);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    let normals = vec![[0.0, 1.0, 0.0]; ANGLE_NUM as usize * 4];
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    let uvs = vec![[1.0, 1.0]; ANGLE_NUM as usize * 4];
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    let mut indices = Vec::new();

    for i in 0..ANGLE_NUM - 1 {
        add_quad_indices(&mut indices, [i, i + 1, i + ANGLE_NUM, i + ANGLE_NUM + 1]);
    }

    add_quad_indices(
        &mut indices,
        [0, ANGLE_NUM, ANGLE_NUM * 3 - 1, ANGLE_NUM * 4 - 1],
    );

    for i in ANGLE_NUM * 2..ANGLE_NUM * 2 + ANGLE_NUM - 1 {
        add_quad_indices(&mut indices, [i, i + 1, i + ANGLE_NUM, i + ANGLE_NUM + 1]);
    }

    add_quad_indices(
        &mut indices,
        [
            ANGLE_NUM - 1,
            ANGLE_NUM * 2,
            ANGLE_NUM * 2 - 1,
            ANGLE_NUM * 3,
        ],
    );

    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}

fn add_vertices(
    vertices: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    radius: f32,
    start_angle: f32,
    direction: bool,
) {
    let mult = if direction { 1.0 } else { -1.0 };
    for i in 0..ANGLE_NUM {
        let angle = start_angle + mult * i as f32 * PI / (ANGLE_NUM - 1) as f32;
        vertices.push([
            x as f32 + radius * angle.cos(),
            y as f32 + radius * angle.sin() * -mult,
            0.0,
        ]);
    }
}

fn add_quad_indices(indices: &mut Vec<u32>, points: [u32; 4]) {
    indices.extend([points[0], points[2], points[1]].iter());
    indices.extend([points[1], points[2], points[3]].iter());
}
