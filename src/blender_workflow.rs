use bevy::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_registry_export::ExportRegistryPlugin;
use crate::environment::bounds::Chunk;
use crate::environment::Ground;
use crate::player::spawn::SpawnPoint;

pub struct BlenderWorkflowPlugin;

impl Plugin for BlenderWorkflowPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Chunk>()
            .register_type::<Ground>()
            .register_type::<SpawnPoint>()
            
            .add_plugins((
                ExportRegistryPlugin::default(),
                ComponentsFromGltfPlugin::default(),
            ))
        ;
    }
}