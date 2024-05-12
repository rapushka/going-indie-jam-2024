use bevy::ecs::system::Command;
use bevy::prelude::*;

use crate::AppState;
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