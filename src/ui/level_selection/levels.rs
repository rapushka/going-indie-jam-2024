use bevy::asset::AssetContainer;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_rapier3d::na::DimAdd;

use crate::ui::level_selection::levels::level_1::LoadLevel1Command;
pub use commons::*;

mod commons;

mod level_1;

pub struct LoadLevelCommand {
    pub level_number: u8,
}

impl LoadLevelCommand {
    pub fn new(level_number: u8) -> Self { Self { level_number } }
}

impl Command for LoadLevelCommand {
    fn apply(self, world: &mut World) {
        LoadLevel1Command.apply(world);
    }
}

pub trait LevelLoader {
    fn load(spawn_children: impl FnOnce(&mut WorldChildBuilder));
}