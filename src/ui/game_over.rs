use bevy::prelude::*;
use crate::{AppState, constants, GameState, OnAppState, ui};
use crate::ui::gameplay_hud::pause::BackToMainMenuButton;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), build_game_over_screen)
        ;
    }
}

fn build_game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const TITLE: &'static str = "Congratulation! You've completed the level!";

    commands.spawn((
        Name::new("Game Over Screen"),
        NodeBundle {
            style: constants::styles::MAIN_MENU,
            ..default()
        },
        OnAppState(AppState::Gameplay),
    ))
        .with_children(|parent| {
            ui::create::text(&asset_server, TITLE, parent, 64.0);
            ui::create::button(&asset_server, parent, "Back To Main Menu", BackToMainMenuButton);
        });
}