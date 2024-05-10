use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

use crate::animations::*;
use crate::blender_workflow::BlenderWorkflowPlugin;
use crate::camera::*;
use crate::environment::EnvironmentPlugin;
use crate::player::*;
use crate::player::movement::{JumpForce, MovementSpeed};
use crate::ui::UiPlugin;

mod player;
mod camera;
mod environment;
mod constants;
mod animations;
mod ui;
mod blender_workflow;

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
    MainMenu,
    Gameplay,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    Undefined,
    Playing,
    Paused,
}

#[derive(Component)]
pub struct OnAppState(pub AppState);

#[derive(AssetCollection, Resource)]
struct LevelAssets {
    #[asset(path = "levels/level1.gltf")]
    level: Handle<Scene>,
}

fn main() {
    App::new()
        .configure_sets(Update, (Order::Input, Order::GameLogic, Order::Physics, Order::View).chain())
        .configure_sets(PostUpdate, (Order::Input, Order::GameLogic, Order::Physics, Order::View).chain())
        .init_state::<AppState>()
        .init_state::<GameState>()

        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::MainMenu)
                .load_collection::<LevelAssets>(),
        )

        .add_plugins((
            // dependencies
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            ThirdPersonCameraPlugin,
            BlenderWorkflowPlugin,

            // game
            CameraPlugin,
            PlayerPlugin,
            EnvironmentPlugin,
            AnimationsPlugin,
            UiPlugin,
        ))

        .add_systems(OnEnter(AppState::MainMenu), (
            // start_game,
            test_gltf_level,
        ))

        .add_systems(Update, (
            despawn_not_in_state,
        ))

        .run();
}

fn start_game(
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    next_app_state.set(AppState::MainMenu);
}

pub fn despawn_not_in_state(
    mut transitions: EventReader<StateTransitionEvent<AppState>>,
    mut entities: Query<(Entity, &OnAppState)>,
    mut commands: Commands,
) {
    for transition in transitions.read() {
        for (entity, on_state) in &mut entities {
            if on_state.0 != transition.after {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn test_gltf_level(
    mut commands: Commands,
    asset_server: Res<LevelAssets>,
) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.level.clone(),
            ..default()
        },
        Name::new("Level 1"),
        // OnAppState(AppState::Gameplay),
    ));
}

fn start_level(
    mut commands: Commands,
    assets: Res<LevelAssets>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        SceneBundle {
            scene: assets.level.clone(),
            ..default()
        },
        Name::new("Level1"),
    ));
}
