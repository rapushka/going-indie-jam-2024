use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::stars::{Star, StarCollected};

#[derive(Resource, Default)]
pub struct LevelProgress {
    pub collected_stars: u8,
    pub total_stars: u8,
}

impl LevelProgress {
    fn reset(&mut self) {
        self.collected_stars = 0;
        self.total_stars = 0;
    }
}

pub struct LevelProgressPlugin;

impl Plugin for LevelProgressPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LevelProgress>()

            .add_systems(Update, (
                on_new_star_added,
                on_star_collected,
                check_level_completed,
            ).chain(),
            ) // TODO: run_if?
            .add_systems(OnExit(AppState::Gameplay), reset_progress)
        ;
    }
}

fn reset_progress(
    mut progress: ResMut<LevelProgress>,
) {
    progress.reset();
}

fn on_new_star_added(
    stars: Query<Entity, Added<Star>>,
    mut progress: ResMut<LevelProgress>,
) {
    for _ in stars.iter() {
        progress.total_stars += 1;
    }
}

fn on_star_collected(
    mut star_collected_event: EventReader<StarCollected>,
    mut progress: ResMut<LevelProgress>,
) {
    for _ in star_collected_event.read() {
        progress.collected_stars += 1;
    }
}

fn check_level_completed(
    progress: Res<LevelProgress>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if !progress.is_changed() {
        return;
    }

    if progress.total_stars > 0
        && progress.collected_stars >= progress.total_stars {
        next_game_state.set(GameState::GameOver);
    }
}