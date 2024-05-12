use std::collections::HashMap;
use bevy::prelude::*;

use crate::{AppState, constants, LevelAssets, OnAppState, ui};
use crate::ui::{Clicked, create};
use crate::ui::level_selection::levels::LoadLevelCommand;
use crate::ui::level_selection::spawn_points::SpawnPointsPlugin;

use super::gameplay_hud::pause::BackToMainMenuButton;

mod spawn_points;
mod levels;

#[derive(Component)]
pub struct StartLevelButton(u8);

pub struct LevelSelectionPlugin;

impl Plugin for LevelSelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SpawnPointsPlugin)

            .add_systems(OnEnter(AppState::LevelSelection), (
                build_level_selection,
            ))

            .add_systems(Update, (
                on_level_button_clicked,
            ).chain()
                .run_if(in_state(AppState::LevelSelection)))
        ;
    }
}

fn build_level_selection(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("level selection page"),
        ui::main_menu::MainMenu {},
        NodeBundle {
            style: constants::styles::LEVEL_SELECTION,
            z_index: ui::order::LEVEL_SELECTION,
            ..default()
        },
        OnAppState(AppState::LevelSelection),
    ))
        .with_children(|parent| {
            create::title(&asset_server, parent, "Select Level");

            // levels
            create::horizontal_layout(parent, |parent| {
                create::small_button(&asset_server, parent, "1", StartLevelButton(1));
                create::small_button(&asset_server, parent, "2", StartLevelButton(2));
                create::small_button(&asset_server, parent, "3", StartLevelButton(3));
            });

            create::button(&asset_server, parent, "Back", BackToMainMenuButton {});
        });
}

fn on_level_button_clicked(
    mut clicked_event: EventReader<Clicked>,
    buttons: Query<&StartLevelButton>,
    mut commands: Commands,
) {
    for e in clicked_event.read() {
        if let Ok(button) = buttons.get(e.0) {
            commands.add(LoadLevelCommand::new(button.0));
        }
    }
}