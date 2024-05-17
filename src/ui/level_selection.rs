use std::collections::HashMap;
use bevy::prelude::*;
use bevy_editor_pls::egui::ImageData::Color;

use crate::{AppState, constants, LevelAssets, MyAssets, OnAppState, ui};
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
    assets: ResMut<MyAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for e in clicked_event.read() {
        if let Ok(button) = buttons.get(e.0) {
            commands.spawn((
                Name::new(format!("level {}", button.0)),
                OnAppState(AppState::Gameplay),
                Transform::default(),
                GlobalTransform::default(),
                InheritedVisibility::default(),
            ))
                .with_children(|parent| {
                    match button.0 {
                        1 => levels::level_1::load(parent, &assets, &mut meshes, &mut materials),
                        2 => levels::level_2::load(parent, &assets, &mut meshes, &mut materials),
                        3 => levels::level_3::load(parent, &assets, &mut meshes, &mut materials),
                        _ => error!("Not implemented yet"),
                    }
                });
        }
    }
}