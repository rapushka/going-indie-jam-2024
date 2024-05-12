use std::collections::HashMap;

use bevy::prelude::*;

use crate::{AppState, LevelLoadingOrder, Order};
use crate::player::spawn::{SpawnPoint, SpawnPointsMap};
use crate::ui::level_selection::spawn_points;

pub struct SpawnPointsPlugin;

impl Plugin for SpawnPointsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpawnPointsMap(HashMap::new()))

            .add_systems(Update, (
                collect_spawn_points,
                prepare_first_spawn_point,
            ).in_set(LevelLoadingOrder::Prepare)
                .run_if(in_state(AppState::LevelSelection)))

            .add_systems(OnExit(AppState::Gameplay), (
                cleanup_spawn_points,
            ))
        ;
    }
}

fn collect_spawn_points(
    mut spawn_points: ResMut<SpawnPointsMap>,
    query: Query<(Entity, &SpawnPoint), Added<SpawnPoint>>,
) {
    for (entity, spawn_point) in query.iter() {
        spawn_points.0.insert(spawn_point.chunk_index, entity);
    }
}

fn prepare_first_spawn_point(
    mut next_state: ResMut<NextState<AppState>>,
    spawn_points: Res<SpawnPointsMap>,
) {
    if let Some(_) = spawn_points.get(0) {
        next_state.set(AppState::Gameplay);
    }
}

fn cleanup_spawn_points(
    mut spawn_points: ResMut<SpawnPointsMap>,
) {
    spawn_points.0.clear();
}