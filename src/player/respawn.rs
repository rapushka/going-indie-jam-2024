use bevy::prelude::*;
use crate::*;
use crate::player::despawn::PlayerDead;
use crate::player::spawn::{SpawnPlayer, SpawnPoint, SpawnPointsMap};

pub struct RespawnPlugin;

impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                respawn_player,
            )
                .run_if(on_event::<PlayerDead>())
                .in_set(Order::GameLogic),
            )
        ;
    }
}

fn respawn_player(
    mut player_dead_event: EventReader<PlayerDead>,
    mut spawn_player_event: EventWriter<SpawnPlayer>,
    spawn_points: Res<SpawnPointsMap>,
    positions: Query<&Transform, With<SpawnPoint>>,
) {
    for e in player_dead_event.read() {
        if let Ok(spawn_point) = positions.get(spawn_points.get(e.chunk_index)) {
            spawn_player_event.send(SpawnPlayer { position: spawn_point.translation });
        } else {
            error!("spawn point {} is missing", e.chunk_index)
        }
    }
}
