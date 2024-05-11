use bevy::prelude::*;
use crate::{AppState, GameState};
use pause::*;

pub(crate) mod pause;

pub struct GameplayHudPlugin;

impl Plugin for GameplayHudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PausePlugin)

            .add_systems(OnEnter(AppState::Gameplay), (
                start_gameplay,
            ))
        ;
    }
}

fn start_gameplay(
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_game_state.set(GameState::Playing);
}