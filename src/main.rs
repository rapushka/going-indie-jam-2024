use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy_asset_loader::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_inspector_egui::quick::*;
use bevy_rapier3d::prelude::*;
use bevy_text_animation::TextAnimatorPlugin;
use bevy_third_person_camera::*;

use crate::animations::*;
use crate::blender_workflow::BlenderWorkflowPlugin;
use crate::camera::*;
use crate::delay::DelayPlugin;
use crate::environment::EnvironmentPlugin;
use crate::level_progress::LevelProgressPlugin;
use crate::player::*;
use crate::player::movement::{JumpForce, MovementSpeed};
use crate::stars::StarsPlugin;
use crate::tutors::TutorsPlugin;
use crate::ui::UiPlugin;

mod player;
mod camera;
mod environment;
mod constants;
mod animations;
mod ui;
mod blender_workflow;
mod extensions;
mod stars;
mod level_progress;
mod delay;
mod tutors;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Order {
    Input,
    Physics,
    GameLogic,
    View,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum LevelLoadingOrder {
    Prepare,
    Playing,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    LevelSelection,
    Gameplay,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    Undefined,
    Playing,
    Paused,
    GameOver, // i mean it's level completed
}

#[derive(Component)]
pub struct OnAppState(pub AppState);

#[derive(AssetCollection, Resource)]
struct LevelAssets {
    #[asset(paths("levels/level1.gltf#Scene0"), collection(typed))]
    levels: Vec<Handle<Scene>>,
}

#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(path = "models/Ground.gltf#Scene0")]
    ground: Handle<Scene>,

    #[asset(path = "models/Star.gltf#Scene0")]
    star: Handle<Scene>,
}

fn main() {
    App::new()
        .configure_sets(Update, (Order::Input, Order::GameLogic, Order::Physics, Order::View).chain())
        .configure_sets(OnEnter(AppState::Gameplay), (LevelLoadingOrder::Prepare, LevelLoadingOrder::Playing).chain())
        .configure_sets(PostUpdate, (Order::Input, Order::GameLogic, Order::Physics, Order::View).chain())

        .init_state::<AppState>()
        .init_state::<GameState>()

        // .insert_resource(WinitSettings::desktop_app())

        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::MainMenu)
                .load_collection::<LevelAssets>()
                .load_collection::<MyAssets>()
        )

        // dependencies
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            ThirdPersonCameraPlugin,
            BlenderWorkflowPlugin,
            EditorPlugin::default(),
            TextAnimatorPlugin,
        ))

        // game
        .add_plugins((
            CameraPlugin,
            PlayerPlugin,
            EnvironmentPlugin,
            AnimationsPlugin,
            UiPlugin,
            StarsPlugin,
            LevelProgressPlugin,
            DelayPlugin,
            TutorsPlugin,
        ))

        .add_systems(PreStartup, show_loading_curtain)

        .add_systems(OnExit(AppState::Loading), || { info!("=== Game Started ===") })

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

fn show_loading_curtain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("loading curtain"),
        NodeBundle {
            background_color: Color::BLACK.into(),
            ..default()
        },
        OnAppState(AppState::Loading),
    ))
        .with_children(|parent| { ui::create::text(&asset_server, "Loading...", parent, 16.0); });
}