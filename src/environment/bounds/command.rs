use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use crate::environment::bounds::Chunk;
use crate::{AppState, OnAppState};

pub struct SpawnChunkCommand { // TODO: mb set parent
    pub chunk_index: u8,
    pub color: Color,
    pub position: Vec3,
    pub size: Vec3,
}

impl Command for SpawnChunkCommand {
    fn apply(self, world: &mut World) {
        let half_size = self.size * 0.5;

        let mesh = world.resource_mut::<Assets<Mesh>>().add(Cuboid::from_size(self.size));
        let material = world.resource_mut::<Assets<StandardMaterial>>().add(self.color);

        world.spawn((
            Name::new(format!("chunk {}", self.chunk_index)),
            Chunk::new(self.chunk_index),
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(self.position),
                ..default()
            },
            OnAppState(AppState::Gameplay),
            Aabb { center: self.position.into(), half_extents: half_size.into() },
        ));
    }
}