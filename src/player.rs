use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;
use spawn::SpawnPlayer;

use crate::*;
use crate::player::despawn::DespawnPlugin;
use crate::player::movement::*;
use crate::player::respawn::RespawnPlugin;
use crate::player::spawn::SpawnPlugin;

pub mod movement;
pub mod spawn;
pub mod respawn;
pub mod despawn;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Jump>()

            .add_plugins((
                SpawnPlugin,
                RespawnPlugin,
                DespawnPlugin,
            ))

            .add_systems(Update, (
                input_movement,
                input_jump,
            ).in_set(Order::Input))

            .add_systems(Update, (
                update_grounded,
            ).in_set(Order::Physics))

            .add_systems(Update, (
                spawn_new_player,
            )
                .in_set(Order::GameLogic)
                .run_if(on_event::<SpawnPlayer>()))

            .add_systems(Update, (
                move_player,
                do_jump,
            )
                .in_set(Order::GameLogic)
                .run_if(in_state(GameState::Playing)))

            .add_systems(Update, (
                rotate_to_moving_direction,
            ).in_set(Order::View))
        ;
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_new_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_player_events: EventReader<SpawnPlayer>,
) {
    for spawn_player_event in spawn_player_events.read() {
        let half_size = Vec3 { x: 0.5, y: 0.5, z: 0.5 };

        commands.spawn((
            Name::new("player"),
            Player,
            MovementSpeed(constants::PLAYER_MOVEMENT_SPEED),
            JumpForce(100.0),
            ThirdPersonCameraTarget,
            SceneBundle {
                scene: asset_server.load("models/Character.gltf#Scene0"),
                transform: Transform::from_translation(spawn_player_event.position).with_scale(half_size),
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
}
