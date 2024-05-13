use bevy::prelude::*;
use crate::{AppState, GameState, OnAppState, ui};
use pause::*;

pub(crate) mod pause;

pub struct GameplayHudPlugin;

impl Plugin for GameplayHudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PausePlugin)

            .add_systems(OnEnter(AppState::Gameplay), (
                build_hud,
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

fn build_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("HUD"),
        NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        },
        OnAppState(AppState::Gameplay),
    ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("stars count text"),
                ui::create::text_bundle(&asset_server, "stars: 0/0", 64.0),
            ));
        });
}