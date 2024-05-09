use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use crate::environment::bounds::Chunk;

pub struct SpawnChunk {
    pub chunk_index: u8,
    pub color: Color,
    pub position: Vec3,
    pub size: Vec3,
}

impl Command for SpawnChunk {
    fn apply(self, world: &mut World) {
        let mesh;
        {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
            mesh = meshes.add(Cuboid::from_size(self.size));
        }
        let material;
        {
            let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();

            material = materials.add(self.color);
        }
        world.spawn((
            Name::new(format!("bounds {}", self.chunk_index)),
            Chunk(self.chunk_index),
            PbrBundle {
                mesh,
                material: material.clone(),
                transform: Transform::from_translation(self.position),
                ..default()
            },
            Aabb { center: self.position.into(), half_extents: self.size.into() },
        ));
    }
}
