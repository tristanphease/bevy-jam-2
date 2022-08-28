use bevy::{prelude::*, math::Vec3Swizzles};

use crate::game::{health_bar::{generate_health_bar, HealthBarMaterial, WithHealthBar}, components::{Health, Hitbox, CollidesEnemy, CollidesPlayer, Damage, DamageCooldown, AnimationTimer, DropsItemOnDeath, ItemDropType}, enemies::cat::Cat};

const CAT_IMAGE_PATH: &str = "images/cat.png";
const CAT_IMAGE_TEXTURE_SIZE: Vec2 = Vec2::new(166.0, 161.0);
const CAT_SIZE: Vec2 = Vec2::new(100.0, 100.0);
const CAT_HEALTH: f32 = 200.0;
const CAT_HIT_DAMAGE: f32 = 5.0;

pub const CAT_DROP_IMAGE_PATH: &str = "images/cat_tail.png";
pub const CAT_NUM: usize = 1;

pub fn create_cat(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<HealthBarMaterial>>,
    position: Vec2,
) {
    let texture_handler = asset_server.load(CAT_IMAGE_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handler, CAT_IMAGE_TEXTURE_SIZE, 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let cat_pos = Vec3::new(position.x, position.y, 1.0);

    let health_bar = generate_health_bar(commands, meshes, materials, cat_pos, CAT_SIZE.y/2.0);

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: Some(CAT_SIZE),
            ..default()
        },
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(cat_pos),
        ..default()
    })
    .insert(Health::new(CAT_HEALTH))
    .insert(WithHealthBar(health_bar))
    .insert(Hitbox(CAT_SIZE))
    .insert(CollidesEnemy)
    .insert(CollidesPlayer)
    .insert(Damage(CAT_HIT_DAMAGE))
    .insert(DamageCooldown { timer: Timer::from_seconds(2.0, true) })
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
    .insert(Cat {
        target_position: cat_pos.xy(),
        shoot_timer: Timer::from_seconds(1.0, true),
    })
    .insert(DropsItemOnDeath { drop: ItemDropType::CatTail });
}