use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_inspector_egui::quick::*;

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
            ThirdPersonCameraPlugin,
            PlayerPlugin,
            CameraPlugin,
            EnvironmentPlugin,
        ))

        .run();
}

