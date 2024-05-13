use bevy::prelude::*;
use bevy_rapier3d::plugin::RapierContext;

pub use create::*;

use crate::{AppState, Order};
use crate::player::Player;

mod create;

#[derive(Component)]
pub struct Star;

pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, (
                collect_stars,
            )
                .in_set(Order::GameLogic)
                .run_if(in_state(AppState::Gameplay)))
        ;
    }
}

fn collect_stars(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    players: Query<Entity, With<Player>>,
    stars: Query<Entity, With<Star>>,
) {
    for player in players.iter() {
        for star in stars.iter() {
            if rapier_context.contact_pair(player, star).is_some() {
                commands.entity(star).despawn_recursive();
            }
        }
    }
}

