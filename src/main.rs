use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_inspector_egui::quick::*;
use bevy_rapier3d::prelude::*;
use crate::animations::*;

use crate::camera::*;
use crate::environment::EnvironmentPlugin;
use crate::player::*;

mod player;
mod camera;
mod environment;
mod constants;
mod animations;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Order {
    Input,
    Physics,
    GameLogic,
    View,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
    #[default]
    Loading,
    Gameplay,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    Undefined,
    Playing,
    Paused,
}

fn main() {
    App::new()
        .configure_sets(Update, (Order::Input, Order::GameLogic, Order::Physics, Order::View).chain())
        .configure_sets(PostUpdate, (Order::Input, Order::GameLogic, Order::Physics, Order::View).chain())
        .init_state::<AppState>()
        .init_state::<GameState>()

        .add_plugins((
            // dependencies
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            ThirdPersonCameraPlugin,

            // game
            CameraPlugin,
            PlayerPlugin,
            EnvironmentPlugin,
            AnimationsPlugin,
        ))

        .add_systems(OnEnter(AppState::Loading), (
            start_game,
        ))

        .run();
}

fn start_game(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    // next_app_state.set(AppState::Gameplay);
    // next_game_state.set(GameState::Playing);
}

