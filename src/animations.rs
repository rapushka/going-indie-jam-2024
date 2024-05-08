use std::collections::HashMap;
use bevy::prelude::*;
use crate::animations::animation_key::*;

pub mod animation_key {
    pub const IDLE: i32 = 0;
    pub const DEATH: i32 = 1;
    pub const JUMP: i32 = 2;
    pub const JUMP_IDLE: i32 = 3;
    pub const JUMP_LAND: i32 = 4;
    pub const RUN: i32 = 5;
    pub const WALK: i32 = 6;
}

#[derive(Resource)]
struct Animations(HashMap<i32, Handle<AnimationClip>>);

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_animations)

            .add_systems(Update, (
                idle_player_on_spawned,
            ))
        ;
    }
}

fn load_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut animations = HashMap::new();
    animations.insert(IDLE, asset_server.load("models/Character.gltf#Animation3"));
    animations.insert(DEATH, asset_server.load("models/Character.gltf#Animation0"));
    animations.insert(JUMP, asset_server.load("models/Character.gltf#Animation6"));
    animations.insert(JUMP_IDLE, asset_server.load("models/Character.gltf#Animation7"));
    animations.insert(JUMP_LAND, asset_server.load("models/Character.gltf#Animation8"));
    animations.insert(RUN, asset_server.load("models/Character.gltf#Animation11"));
    animations.insert(WALK, asset_server.load("models/Character.gltf#Animation14"));

    commands.insert_resource(Animations(animations));
}

fn idle_player_on_spawned(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[&IDLE].clone_weak()).repeat();
    }
}
