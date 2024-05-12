use bevy::core::Name;
use bevy::ecs::system::Command;
use bevy::math::Vec3;
use bevy::prelude::*;

use crate::{AppState, MyAssets, OnAppState};

pub struct LoadLevel1Command;

impl Command for LoadLevel1Command {
    fn apply(self, world: &mut World) {
        let root = world.spawn((
            Name::new("level 1"),
            OnAppState(AppState::Gameplay),
        )).id();

        super::CreateGroundCommand {
            parent: Some(root),
            ..default()
        }.apply(world);
    }
}