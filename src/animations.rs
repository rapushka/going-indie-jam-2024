use std::collections::HashMap;
use std::time::Duration;
use bevy::prelude::*;
use crate::Order;
use crate::player::*;
use crate::player::movement::*;

// Keys
pub const IDLE: i32 = 0;
pub const DEATH: i32 = 1;
pub const JUMP: i32 = 2;
pub const JUMP_IDLE: i32 = 3;
pub const JUMP_LAND: i32 = 4;
pub const RUN: i32 = 5;
pub const WALK: i32 = 6;

#[derive(Resource)]
pub struct Animations(pub(crate) HashMap<i32, Handle<AnimationClip>>);

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_animations)

            .add_systems(Update, (
                idle_player_on_spawned,
                play_run_animation,
            ).in_set(Order::View))
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
    mut animators: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut animator in &mut animators {
        animator.play(animations.0[&IDLE].clone_weak()).repeat();
    }
}

fn play_run_animation(
    animations: Res<Animations>,
    mut animators: Query<&mut AnimationPlayer>,
    mut players: Query<&MoveDirection, With<Player>>,
) {
    for direction in players.iter() {
        for mut animator in &mut animators {
            let is_moving = direction.0.length_squared() > 0.0;
            let key = if is_moving { &RUN } else { &IDLE };

            animator.play_with_transition(
                animations.0[&key].clone_weak(),
                Duration::from_millis(250), // TODO: huh?
            )
                .repeat();
        }
    }
}
