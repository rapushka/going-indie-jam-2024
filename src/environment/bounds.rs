use bevy::ecs::system::Command;
use bevy::prelude::*;

use crate::AppState;
use crate::environment::bounds::command::SpawnChunk;

mod command;

#[derive(Component)]
pub struct Chunk(u8);

pub struct BoundsPlugin;

impl Plugin for BoundsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Loading), (
                create_bounds,
            ))
        ;
    }
}

fn create_bounds(
    mut commands: Commands,
) {
    let color = Color::rgba_u8(255, 0, 0, 100);
    let position = Vec3::new(0.0, 0.0, -5.0);
    let size = Vec3::new(30.0, 20.0, 10.0);

    commands.add(SpawnChunk { chunk_index: 0, color, position, size })
}