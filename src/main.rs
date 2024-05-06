use bevy::prelude::*;
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
            PlayerPlugin,
            CameraPlugin,
            EnvironmentPlugin,
        ))

        .run();
}

