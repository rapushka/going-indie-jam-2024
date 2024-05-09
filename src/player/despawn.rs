use bevy::math::bounding::Aabb3d;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

use crate::{AppState, Order};
use crate::environment::bounds::Chunk;
use crate::player::Player;

const DESPAWN_HEIGHT: f32 = -10.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<KillPlayer>()
            .add_event::<PlayerDead>()

            .add_systems(Update, (
                player_suicide,
                kill_player_with_too_low_position,
                handle_player_death,
            ).chain()
                .in_set(Order::GameLogic)
                .run_if(in_state(AppState::Gameplay)))

        ;
    }
}

#[derive(Event)]
pub struct KillPlayer(Entity);

#[derive(Event)]
pub struct PlayerDead {
    pub chunk_index: u8,
}

impl PlayerDead {
    pub fn out_of_bounce() -> Self { PlayerDead { chunk_index: 0 } }
}

fn player_suicide(
    input: Res<ButtonInput<KeyCode>>,
    players: Query<Entity, With<Player>>,
    mut kill_player_event: EventWriter<KillPlayer>,
) {
    for (player) in players.iter() {
        if input.just_pressed(KeyCode::KeyL) {
            kill_player_event.send(KillPlayer(player));
        }
    }
}

fn kill_player_with_too_low_position(
    players: Query<(Entity, &Transform), With<Player>>,
    mut kill_player_event: EventWriter<KillPlayer>,
) {
    for (player, transform) in players.iter() {
        if transform.translation.y <= DESPAWN_HEIGHT {
            kill_player_event.send(KillPlayer(player));
        }
    }
}

fn handle_player_death(
    mut commands: Commands,
    mut kill_player_event: EventReader<KillPlayer>,
    players: Query<(Entity, &Transform), With<Player>>,
    chunks: Query<(&Chunk, &Aabb)>,
    mut death_event: EventWriter<PlayerDead>,
) {
    for e in kill_player_event.read() {
        if let Ok((player, transform)) = players.get(e.0) {
            let mut chunk_index = 0;
            for (chunk, bounds) in chunks.iter() {
                if bounds.contains(transform.translation) {
                    chunk_index = chunk.0;
                    break;
                }
            }

            death_event.send(PlayerDead { chunk_index });
            commands.entity(player).despawn_recursive();
        }
    }
}

pub trait AabbExtensions {
    fn contains(&self, point: Vec3) -> bool;
}

impl AabbExtensions for Aabb {
    fn contains(&self, point: Vec3) -> bool {
        let min = self.min();
        let max = self.max();

        point.x >= min.x && point.x <= max.x &&
            point.y >= min.y && point.y <= max.y &&
            point.z >= min.z && point.z <= max.z
    }
}