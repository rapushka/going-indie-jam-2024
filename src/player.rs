use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

use crate::{constants, Order};
use crate::player::movement::*;

pub mod movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Jump>()

            .add_systems(Startup, spawn_player)

            .add_systems(Update, (
                read_movement,
                read_jump_input,
            ).in_set(Order::Input))

            .add_systems(Update, (
                update_grounded,
            ).in_set(Order::Physics))

            .add_systems(Update, (
                move_player,
                do_jump,
            ).in_set(Order::GameLogic))

            .add_systems(Update, (
                rotate_to_moving_direction,
            ).in_set(Order::View))
        ;
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let scale = Vec3 { x: 0.5, y: 0.5, z: 0.5 };

    commands.spawn((
        Name::new("player"),
        Player,
        MovementSpeed(constants::PLAYER_MOVEMENT_SPEED),
        JumpForce(100.0),
        ThirdPersonCameraTarget,
        SceneBundle {
            scene: asset_server.load("models/Character.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(scale),
            ..default()
        },
        SecondJumpLeft(true),
        IsGrounded(false),

        // physics
        KinematicCharacterController::default(),
        RigidBody::Dynamic,
        Collider::capsule(Vec3::Y, Vec3::Y * 2.0, 1.0),
        GravityScale(2.0),
        ColliderMassProperties::Mass(10.0),
        Velocity::default(),
        ExternalImpulse::default(),
    ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(MoveDirection(Vec3::ZERO))
    ;
}