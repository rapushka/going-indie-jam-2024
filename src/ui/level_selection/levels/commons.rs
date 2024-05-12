use bevy::core::Name;
use bevy::ecs::system::Command;
use bevy::math::Vec3;
use bevy::prelude::*;
use crate::{AppState, MyAssets, OnAppState};

pub struct CreateGroundCommand {
    pub parent: Option<Entity>,
    pub position: Vec3,
    pub sizes: Vec3,
}

impl Default for CreateGroundCommand {
    fn default() -> Self {
        Self {
            sizes: Vec3::ONE,
            position: Vec3::ZERO,
            parent: None,
        }
    }
}

impl Command for CreateGroundCommand {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, assets: Mut<MyAssets>| {
            let mut e = world.spawn((
                Name::new("ground"),
                SceneBundle {
                    scene: assets.ground.clone(),
                    transform: Transform::from_translation(self.position).with_scale(self.sizes),
                    ..default()
                },
            ));

            if let Some(parent) = self.parent {
                e.set_parent(parent);
            }
        });
    }
}