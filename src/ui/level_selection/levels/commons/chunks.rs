use bevy::core::Name;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

use crate::environment::bounds::Chunk;
use crate::extensions::{Vec2Extensions, Vec3Extensions};
use crate::{AppState, OnAppState};

const CHUNK_TRANSPARENCY: f32 = 0.5;

pub fn create_chunk(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    index: u8,
    color: Color,
    position: Vec2,
    sizes: Vec2,
) {
    // commands.add(crate::environment::bounds::SpawnChunkCommand {
    //     chunk_index: index,
    //     color: color.with_a(CHUNK_TRANSPARENCY),
    //     position: position.as_flat().set_y(-10.0),
    //     size: sizes.as_flat().set_y(0.1),
    // });

    apply(
        parent,
        meshes,
        materials,
        index,
        color.with_a(CHUNK_TRANSPARENCY),
        position.as_flat().set_y(-10.0),
        sizes.as_flat().set_y(0.1),
    );
    apply(
        parent,
        meshes,
        materials,
        index,
        color.with_a(CHUNK_TRANSPARENCY),
        position.as_flat().set_y(-10.0),
        sizes.as_flat().set_y(0.1),
    );
}

fn apply(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk_index: u8,
    color: Color,
    position: Vec3,
    size: Vec3,
) {
    let half_size = size * 0.5;

    let mesh = meshes.add(Cuboid::from_size(size));
    let material = materials.add(color);

    parent.spawn((
        Name::new(format!("chunk {}", chunk_index)),
        Chunk::new(chunk_index),
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(position),
            ..default()
        },
        OnAppState(AppState::Gameplay),
        Aabb { center: position.into(), half_extents: half_size.into() },
    ));
}