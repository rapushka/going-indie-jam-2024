use bevy::core::Name;
use bevy::ecs::system::Command;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use crate::MyAssets;
use crate::extensions::Vec3Extensions;

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
        CreateGroundMeshCommand {
            parent: self.parent,
            position: self.position,
            sizes: self.sizes,
        }.apply(world);
    }
}

struct CreateGroundMeshCommand {
    pub parent: Option<Entity>,
    pub position: Vec3,
    pub sizes: Vec3,
}

impl Command for CreateGroundMeshCommand {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, assets: Mut<MyAssets>| {
            let position = self.position;
            let sizes = self.sizes;
            let offset_from_top = -0.2;
            world.spawn((
                Name::new("cube grass"),
                SceneBundle {
                    scene: assets.ground.clone(),
                    transform: Transform::from_translation(position).with_scale(sizes),
                    ..default()
                },
                create_collider(Vec3::new(0.0, offset_from_top, 0.0), sizes.add_y(offset_from_top)),
            ))
                .set_parent(self.parent.unwrap())
            ;
        });
    }
}

fn create_collider(offset: Vec3, sizes: Vec3) -> Collider {
    Collider::compound(vec![(
        Vec3::new(offset.x, offset.y, offset.z),
        Quat::IDENTITY,
        Collider::cuboid(sizes.x, sizes.y, sizes.z),
    )])
}