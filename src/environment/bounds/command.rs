use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use crate::environment::bounds::Chunk;

pub struct SpawnChunkCommand {
    pub chunk_index: u8,
    pub color: Color,
    pub position: Vec3,
    pub size: Vec3,
}

impl Command for SpawnChunkCommand {
    fn apply(self, world: &mut World) {
        let mesh = world.resource_mut::<Assets<Mesh>>().add(Cuboid::from_size(self.size));
        let material = world.resource_mut::<Assets<StandardMaterial>>().add(self.color);

        world.spawn((
            Name::new(format!("chunk {}", self.chunk_index)),
            Chunk(self.chunk_index),
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(self.position),
                ..default()
            },
            Aabb { center: self.position.into(), half_extents: self.size.into() },
        ));
    }
}