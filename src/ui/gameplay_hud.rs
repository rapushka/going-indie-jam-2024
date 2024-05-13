use bevy::prelude::*;
use crate::{AppState, GameState, OnAppState, ui};
use pause::*;
use crate::level_progress::{LevelProgress, LevelProgressPlugin};
use crate::stars::StarsText;

pub mod pause;

pub struct GameplayHudPlugin;

impl Plugin for GameplayHudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PausePlugin)

            .add_systems(OnEnter(AppState::Gameplay), (
                build_hud,
                start_gameplay,
            ))

            .add_systems(Update, (
                update_stars_text,
            )
                .run_if(in_state(AppState::Gameplay)))
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
                padding: UiRect::all(Val::Px(25.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::RowReverse,
                ..default()
            },
            ..default()
        },
        OnAppState(AppState::Gameplay),
    ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("stars count text"),
                StarsText,
                ui::create::text_bundle(&asset_server, "stars: 0/0", 64.0),
            ));
        });
}

fn update_stars_text(
    level_progress: Res<LevelProgress>,
    mut texts: Query<&mut Text, With<StarsText>>,
) {
    if !level_progress.is_changed() {
        return;
    }

    for mut text in texts.iter_mut() {
        let collected = level_progress.collected_stars;
        let total = level_progress.total_stars;
        text.sections[0].value = format!("stars {}/{}", collected, total);
    }
}