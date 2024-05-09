use bevy::prelude::*;
use crate::{AppState, Order};
use crate::player::Player;

const DESPAWN_HEIGHT: f32 = -100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerDead>()

            .add_systems(Update, (
                kill_player_with_too_low_position,
            )
                .in_set(Order::GameLogic)
                .run_if(in_state(AppState::Gameplay)))

        ;
    }
}

#[derive(Event)]
pub struct PlayerDead {
    pub chunk_index: u8,
}

impl PlayerDead {
    pub fn out_of_bounce() -> Self { PlayerDead { chunk_index: 0 } }
}

fn kill_player_with_too_low_position(
    mut commands: Commands,
    players: Query<(Entity, &Transform), With<Player>>,
    mut kill_event: EventWriter<PlayerDead>,
) {
    for (player, transform) in players.iter() {
        if transform.translation.y <= DESPAWN_HEIGHT {
            kill_event.send(PlayerDead::out_of_bounce());
            commands.entity(player).despawn_recursive();
        }
    }
}