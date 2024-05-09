use bevy::prelude::*;
use crate::{AppState, Order};
use crate::player::Player;

const DESPAWN_HEIGHT: f32 = -100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Kill>()
            .add_event::<Died>()

            .add_systems(Update, (
                kill_player_with_too_low_position,
            )
                .in_set(Order::GameLogic)
                .run_if(in_state(AppState::Gameplay)))
        ;
    }
}

#[derive(Event)]
pub struct Kill(Entity);

#[derive(Event)]
pub struct Died(Entity);

fn kill_player_with_too_low_position(
    players: Query<(Entity, &Transform), With<Player>>,
) {
    for (player, transform) in players.iter() {
        if transform.translation.y <= DESPAWN_HEIGHT {
            info!("despawn player");
        }
    }
}