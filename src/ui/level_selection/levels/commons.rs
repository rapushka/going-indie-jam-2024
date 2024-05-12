use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;

use crate::environment::{Ground, InvisibleWall};
use crate::extensions::Vec3Extensions;
use crate::MyAssets;
use crate::player::spawn::SpawnPoint;

const GRASS_HEIGHT: f32 = 0.2;

pub fn create_spawn_point(
    parent: &mut ChildBuilder,
    index: u8,
    position: Vec3,
) {
    parent.spawn((
        Name::new(format!("spawn point {index}")),
        SpawnPoint { chunk_index: index },
        Transform::from_translation(position),
    ));
}

pub fn create_invisible_wall(
    parent: &mut ChildBuilder,
    position: Vec3,
    sizes: Vec3,
) {
    parent.spawn((
        Name::new("invisible wall"),
        InvisibleWall,
        Transform::from_translation(position).with_scale(sizes),
        create_collider(position, sizes, Vec3::ONE),
    ));
}

pub fn create_ground(
    parent: &mut ChildBuilder,
    assets: &ResMut<MyAssets>,
    position: Vec3,
    sizes: Vec3,
) {
    create_ground_mesh(parent, &assets, position, sizes, |parent| {
        create_grass(parent, position, sizes);
    });
}

fn create_grass(
    parent: &mut ChildBuilder,
    position: Vec3,
    sizes: Vec3,
) {
    let position = position.add_y(sizes.y - GRASS_HEIGHT);

    parent.spawn((
        Name::new("ground"),
        Ground,
        Transform::from_translation(position),
        create_collider(position, sizes.set_y(GRASS_HEIGHT) * 0.99, Vec3::ONE),
    ));
}

fn create_ground_mesh(
    parent: &mut ChildBuilder,
    assets: &ResMut<MyAssets>,
    position: Vec3,
    sizes: Vec3,
    spawn_children: impl FnOnce(&mut ChildBuilder),
) {
    parent.spawn((
        Name::new("cube grass"),
        SceneBundle {
            scene: assets.ground.clone(),
            transform: Transform::from_translation(position).with_scale(sizes),
            ..default()
        },
        create_collider(Vec3::new(0.0, -GRASS_HEIGHT, 0.0), sizes.add_y(-GRASS_HEIGHT), sizes),
    ))
        .with_children(spawn_children)
    ;
}

fn create_collider(offset: Vec3, sizes: Vec3, transform_scale: Vec3) -> Collider {
    let sizes = sizes / transform_scale;

    Collider::compound(vec![(
        Vec3::new(offset.x, offset.y, offset.z),
        Quat::IDENTITY,
        Collider::cuboid(sizes.x, sizes.y, sizes.z),
    )])
}