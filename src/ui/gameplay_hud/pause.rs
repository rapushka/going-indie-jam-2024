use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::constants::controls;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                toggle_pause_visibility
            ).run_if(in_state(AppState::Gameplay)))
        ;
    }
}

fn toggle_pause_visibility(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(controls::PAUSE_KEY) {
        if *current_state.get() == GameState::Playing {
            next_state.set(GameState::Paused);
        } else if *current_state.get() == GameState::Paused {
            next_state.set(GameState::Playing);
        }
    }
}