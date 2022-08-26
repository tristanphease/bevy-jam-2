use super::{components::{Health, Hitbox, Damage, CollidesEnemy}, player::basic_shot::ShotSpeed};
use bevy::{
    prelude::{shape::Circle, *},
    sprite::collide_aabb,
};

#[derive(Component)]
pub struct Shot {
    pub timer: Timer,
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
    let mesh_handle = meshes.add(Mesh::from(Circle::new(1.0)));
    let material_handle = materials.add(ColorMaterial::from(Color::RED));

    commands.insert_resource(ShotResource {
        mesh: mesh_handle,
        material: material_handle,
    });
}

pub fn update_shots(
    mut commands: Commands,
    mut shot_query: Query<(&mut Transform, &mut Shot, &ShotSpeed, Entity)>,
    time: Res<Time>,
) {
    for (mut trans, mut shot, shot_speed, entity) in shot_query.iter_mut() {
        let forward_amount = trans.local_x().normalize() * shot_speed.0 * time.delta_seconds();
        trans.translation += forward_amount;
        shot.timer.tick(time.delta());

        if shot.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

//handles shots doing damage
pub fn collides_enemy(
    mut commands: Commands,
    shot_query: Query<(&Transform, &Hitbox, Entity, &Damage), (With<Shot>, With<CollidesEnemy>)>,
    mut collides_shot_query: Query<(&Transform, &Hitbox, &mut Health), (Without<Shot>, With<CollidesEnemy>)>,
) {
    for (shot_trans, shot_hitbox, shot_entity, damage) in shot_query.iter() {
        for (object_trans, object_hitbox, mut object_health) in
            collides_shot_query.iter_mut()
        {
            if collide_aabb::collide(
                    shot_trans.translation,
                    shot_hitbox.0,
                    object_trans.translation,
                    object_hitbox.0,
                )
                .is_some()
            {
                commands.entity(shot_entity).despawn();
                object_health.current -= damage.0;

                break;
            }
        }
    }
}
