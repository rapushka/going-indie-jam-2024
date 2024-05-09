use bevy::app::AppExit;
use bevy::prelude::*;
use crate::{AppState, constants, GameState, ui};
use crate::constants::controls;
use crate::ui::{Clicked, create};

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
struct ContinueButton;

#[derive(Component)]
struct BackButton;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Paused), build_pause_menu)

            .add_systems(Update, (
                toggle_pause_state
            ).run_if(in_state(AppState::Gameplay)))

            .add_systems(Update, (
                on_continue_button_clicked,
                on_back_button_clicked,
            ).run_if(in_state(GameState::Paused)))

            .add_systems(OnExit(GameState::Paused), destroy_pause_menu)
        ;
    }
}

pub fn build_pause_menu(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        PauseMenu,
        NodeBundle {
            style: constants::styles::MAIN_MENU,
            z_index: ui::order::MAIN_MENU,
            ..default()
        },
    ))
        .with_children(|parent| {
            create::title(&asset_server, parent, "Paused");
            create::button(&asset_server, parent, "Continue", ContinueButton);
            create::button(&asset_server, parent, "Back to main menu", BackButton);
        });
}

pub fn destroy_pause_menu(
    menus: Query<Entity, With<PauseMenu>>,
    mut commands: Commands,
) {
    for menu in menus.iter() {
        commands.entity(menu).despawn_recursive();
    }
}

fn toggle_pause_state(
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

pub fn on_continue_button_clicked(
    buttons: Query<Entity, With<ContinueButton>>,
    mut event_reader: EventReader<Clicked>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read() {
        if let Ok(_) = buttons.get(event.0) {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn on_back_button_clicked(
    buttons: Query<Entity, With<BackButton>>,
    mut event_reader: EventReader<Clicked>,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read() {
        if let Ok(_) = buttons.get(event.0) {
            next_state.set(AppState::MainMenu);
            next_game_state.set(GameState::Undefined);
        }
    }
}
