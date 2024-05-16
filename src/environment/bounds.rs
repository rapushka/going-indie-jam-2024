use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_rapier3d::prelude::DebugRenderContext;

use crate::{AppState, constants};
pub use crate::environment::bounds::command::SpawnChunkCommand;

mod command;

const CHUNK_TRANSPARENCY: u8 = 100;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Chunk {
    pub index: u8,
}

impl Chunk {
    pub fn new(index: u8) -> Self { Self { index } }
}

pub struct BoundsPlugin;

impl Plugin for BoundsPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

fn create_bounds(
    mut commands: Commands,
) {
    // commands.add(SpawnChunkCommand {
    //     chunk_index: 0,
    //     color: Color::rgba_u8(255, 0, 0, CHUNK_TRANSPARENCY),
    //     position: Vec3::new(0.0, 0.0, -5.0),
    //     size: Vec3::new(20.0, 25.0, 10.0),
    // });
    // 
    // commands.add(SpawnChunkCommand {
    //     chunk_index: 1,
    //     color: Color::rgba_u8(0, 255, 0, CHUNK_TRANSPARENCY),
    //     position: Vec3::new(0.0, 0.0, 10.0),
    //     size: Vec3::new(20.0, 25.0, 20.0),
    // });
}

pub fn toggle_chunks_visibility(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    chunks: Query<(Entity, &Visibility), With<Chunk>>,
    mut rapier_debugger: ResMut<DebugRenderContext>,
) {
    if input.just_pressed(constants::controls::TOGGLE_DEBUG_VIEW) {
        for (chunk, visibility) in chunks.iter()
        {
            let visibility = if visibility == Visibility::Visible { Visibility::Hidden } else { Visibility::Visible };
            commands.entity(chunk).insert(visibility);
        }

        rapier_debugger.enabled = !rapier_debugger.enabled;
    }
}