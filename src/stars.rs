use bevy::prelude::*;
pub use create::*;

mod create;

#[derive(Component)]
pub struct Star;

pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app
        ;
    }
}

