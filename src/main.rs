use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_inspector_egui::quick::*;
use bevy_rapier3d::prelude::*;

use crate::camera::*;
use crate::environment::EnvironmentPlugin;
use crate::player::*;

mod player;
mod camera;
mod environment;
mod constants;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            ThirdPersonCameraPlugin,
            PlayerPlugin,
            CameraPlugin,
            EnvironmentPlugin,
        ))

        .run();
}

