use bevy::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_rapier3d::prelude::*;
use bevy_registry_export::ExportRegistryPlugin;
use crate::environment::bounds::Chunk;
use crate::environment::*;
use crate::player::spawn::SpawnPoint;

pub struct BlenderWorkflowPlugin;

impl Plugin for BlenderWorkflowPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Chunk>()
            .register_type::<Ground>()
            .register_type::<SpawnPoint>()
            .register_type::<InvisibleWall>()
            .register_type::<ExitFromLevel>()
            .register_type::<BoxCollider>()

            .add_plugins((
                ExportRegistryPlugin::default(),
                ComponentsFromGltfPlugin::default(),
            ))

            .add_systems(Update, (
                add_box_colliders,
            ))
        ;
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct BoxCollider { // exists only cuz rapier's collider isn't reflected
    pub offset: Vec3,
    pub rotation: Quat,
    pub sizes: Vec3,
}

fn add_box_colliders(
    mut commands: Commands,
    entities: Query<(Entity, &BoxCollider), Added<BoxCollider>>,
) {
    for (e, box_collider) in entities.iter() {
        let sizes = box_collider.sizes;

        commands.entity(e).insert(Collider::compound(vec![(
            box_collider.offset,
            box_collider.rotation,
            Collider::cuboid(sizes.x, sizes.y, sizes.z),
        )]));
    }
}
