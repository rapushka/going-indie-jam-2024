use bevy::prelude::*;
use bevy_rapier3d::na::Vector3;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::dynamics::RigidBodyAdditionalMassProps::Mass;
use bevy_rapier3d::rapier::dynamics::RigidBodyVelocity;
use bevy_third_person_camera::*;

use crate::constants;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                move_player,
                do_jump,
            ))
            .add_systems(FixedUpdate, (
                update_physics,
            ))
        ;
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MovementSpeed(f32);

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let scale = Vec3 { x: 0.5, y: 0.5, z: 0.5 };

    commands.spawn((
        bevy::core::Name::new("player"),
        Player,
        MovementSpeed(constants::PLAYER_MOVEMENT_SPEED),
        JumpForce(100.0),
        ThirdPersonCameraTarget,
        SceneBundle {
            scene: asset_server.load("models/Character.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(scale),
            ..default()
        },
        // physics
        // TODO: mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        Velocity {
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
        },
        RigidBody::Dynamic,
        Collider::capsule(Vec3::Y, Vec3::Y * 2.0, 1.0),
        KinematicCharacterController::default(),
        Ccd::enabled(),
        ExternalImpulse {
            impulse: Vec3::ZERO,
            torque_impulse: Vec3::ZERO,
        },
        GravityScale(2.0),
        ColliderMassProperties::Mass(10.0),
        Damping { // TODO: 0.0 is default
            linear_damping: 0.0,
            ..default()
        },
    ))
        .insert(LockedAxes::ROTATION_LOCKED)
    ;
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut players: Query<(&mut Transform, &MovementSpeed), With<Player>>,
    cameras: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let scaled_speed = constants::PLAYER_MOVEMENT_SPEED * time.delta_seconds();

    for (mut player_transform, player_speed) in players.iter_mut() {
        for camera in cameras.iter() {
            let mut direction = Vec3::ZERO;

            if input.pressed(KeyCode::KeyW) {
                direction += *camera.forward();
            }
            if input.pressed(KeyCode::KeyS) {
                direction += *camera.back();
            }
            if input.pressed(KeyCode::KeyA) {
                direction += *camera.left();
            }
            if input.pressed(KeyCode::KeyD) {
                direction += *camera.right();
            }

            direction.y = 0.0;

            direction = player_speed.0 * direction.normalize_or_zero();
            let movement = direction * scaled_speed;

            player_transform.translation += movement;

            // rotate player to moving direction
            if direction.length_squared() > 0.0 {
                player_transform.look_to(-direction, Vec3::Y);
            }
        }
    }
}

#[derive(Component)]
struct JumpForce(f32);

// TODO: to PhysicsPlugin?
fn update_physics(
    mut controllers: Query<&mut KinematicCharacterController>,
) {
    for mut controller in controllers.iter_mut() {
        // if let Some(mut transition) = controller.translation {
        //     transition.y += 0.1;
        //     controller.translation = Some(transition);
        // }
        // controller.translation = Some(Vec3::new(0.0, -0.25, 0.0));
    }
}

fn do_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ext_impulses: Query<(&JumpForce, &mut ExternalImpulse, &mut Velocity), With<Player>>,
) {
    // Apply impulses.
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (jump_force, mut ext_impulse, mut velocity) in ext_impulses.iter_mut() {
            velocity.linvel = Vec3::ZERO;
            ext_impulse.impulse = Vec3::new(0.0, jump_force.0, 0.0);
        }
    }
}

fn do_jump_(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&JumpForce, &mut Velocity), With<Player>>,
) {
    for (jump_force, mut velocity) in players.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            // rigid_body.linvel = Vec3::new(0.0, jump_force.0, 0.0).into();
            velocity.linvel = Vec3::new(0.0, jump_force.0, 0.0);
            // controller.translation = Some(Vec3::new(0.0, -0.1, 0.0));
        }
    }
}

fn apply_forces(mut ext_forces: Query<&mut ExternalForce>, mut ext_impulses: Query<&mut ExternalImpulse>) {
    // Apply forces.
    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force = Vec3::new(1000.0, 2000.0, 3000.0);
        ext_force.torque = Vec3::new(0.4, 0.5, 0.6);
    }

    // Apply impulses.
    for mut ext_impulse in ext_impulses.iter_mut() {
        ext_impulse.impulse = Vec3::new(100.0, 200.0, 300.0);
        ext_impulse.torque_impulse = Vec3::new(0.4, 0.5, 0.6);
    }
}
