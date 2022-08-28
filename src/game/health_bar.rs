use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::{PrimitiveTopology, AsBindGroup, ShaderRef}},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Material2d}, reflect::TypeUuid,
};

use super::components::Health;

#[derive(Component, Debug, Clone, Copy)]
pub struct WithHealthBar(pub Entity);

#[derive(Component)]
pub struct HealthBar {
    offset_y: f32,
}

pub fn generate_health_bar(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<HealthBarMaterial>>,
    position: Vec3,
    offset_y: f32,
) -> Entity {
    let (mesh_handle, material_handle) = get_health_bar_handles(meshes, materials);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh_handle),
            material: material_handle,
            transform: Transform::from_translation(position + Vec3::Y * offset_y).with_scale(Vec3::new(10.0, 10.0, 1.0)),
            ..default()
        })
        .insert(HealthBar { offset_y })
        .id()
}

pub fn update_health_bars(
    mut materials: ResMut<Assets<HealthBarMaterial>>,
    query: Query<(&Health, &WithHealthBar), Without<HealthBar>>,
    health_bar_query: Query<&Handle<HealthBarMaterial>, With<HealthBar>>,
) {
    for (health, health_bar_entity) in query.iter() {
        if let Ok(bar_material_handle) = health_bar_query.get(health_bar_entity.0) {
            let health_fraction = health.current / health.maximum;
            materials.get_mut(bar_material_handle).unwrap().set_amount(health_fraction);
        }
    }
}

pub fn update_health_bar_positions(
    query: Query<(&WithHealthBar, &Transform), Without<HealthBar>>,
    mut health_bar_query: Query<(&mut Transform, &HealthBar)>,
) {
    for (health_bar_entity, parent_transform) in query.iter() {
        if let Ok((mut bar_transform, health_bar)) = health_bar_query.get_mut(health_bar_entity.0) {
            bar_transform.translation = parent_transform.translation + Vec3::Y * health_bar.offset_y;
        }
    }
}

const BAR_WIDTH: i32 = 8;
const BAR_HEIGHT: i32 = 1;
const ANGLE_NUM: u32 = 8;
const BORDER_SIZE: f32 = 0.2;
const OUTER_RAD: f32 = BAR_HEIGHT as f32 - 0.1;
const INNER_RAD: f32 = BAR_HEIGHT as f32 - 0.1 - BORDER_SIZE;

pub fn get_health_bar_handles(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<HealthBarMaterial>>,
) -> (Handle<Mesh>, Handle<HealthBarMaterial>) {
    let mesh_handle = meshes.add(create_bar());
    let material_handle = materials.add(HealthBarMaterial::default());

    (mesh_handle, material_handle)
}

fn create_bar() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut vertices = Vec::new();
    let x_left = -BAR_WIDTH as f32 / 2.0 + 1.0;
    let y = 0.0;
    let start_angle = FRAC_PI_2;
    add_vertices(&mut vertices, x_left, y, OUTER_RAD, start_angle, true);
    add_vertices(&mut vertices, x_left, y, INNER_RAD, start_angle, true);

    let x_right = BAR_WIDTH as f32 / 2.0 - 1.0;
    add_vertices(&mut vertices, x_right, y, OUTER_RAD, start_angle, false);
    add_vertices(&mut vertices, x_right, y, INNER_RAD, start_angle, false);

    //2 special vertices for inside
    vertices.push([x_left, y, 0.0]);
    vertices.push([x_right, y, 0.0]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    let normals = vec![[0.0, 1.0, 0.0]; ANGLE_NUM as usize * 4 + 2];
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    let mut uvs = vec![[1.0, 1.0]; ANGLE_NUM as usize * 4];
    uvs.push([1.0 / (BAR_WIDTH as f32), 1.0]);
    uvs.push([((BAR_WIDTH as f32) - 1.0) / (BAR_WIDTH as f32), 1.0]);
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

    //add inside indices
    let left_inside_index = ANGLE_NUM * 4;
    for i in ANGLE_NUM..ANGLE_NUM + ANGLE_NUM - 1 {
        indices.extend([left_inside_index, i + 1, i]);
    }

    let right_inside_index = ANGLE_NUM * 4 + 1;
    for i in ANGLE_NUM*3..ANGLE_NUM*3 + ANGLE_NUM - 1 {
        indices.extend([right_inside_index, i + 1, i]);
    }

    add_quad_indices(&mut indices, [ANGLE_NUM * 3, right_inside_index, ANGLE_NUM * 2 - 1, left_inside_index]);
    add_quad_indices(&mut indices, [right_inside_index, ANGLE_NUM * 4 - 1, left_inside_index, ANGLE_NUM]);

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

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "84a33ce5-9d30-4855-89a6-828590468b1f"]
pub struct HealthBarMaterial {
    #[uniform(0)]
    pub amount: Vec2,

    #[uniform(0)]
    color: Color,
}

impl HealthBarMaterial {
    pub fn set_amount(&mut self, amount: f32) {
        self.amount.x = amount;
    }
}

impl Default for HealthBarMaterial {
    fn default() -> Self {
        Self { 
            amount: Vec2::new(1.0, BAR_WIDTH as f32),
            color: Color::RED,
        }
    }
}

impl Material2d for HealthBarMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/health_bar.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/health_bar.wgsl".into()
    }
}