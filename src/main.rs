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

fn main() {
    App::new()
        .configure_sets(Update, (Order::Input, Order::GameLogic, Order::Physics, Order::View).chain())

        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            ThirdPersonCameraPlugin,
            PlayerPlugin,
            CameraPlugin,
            EnvironmentPlugin,
            AnimationsPlugin,
        ))

        .run();
}

