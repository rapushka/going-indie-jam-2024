use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::animations::*;
use crate::constants;
use crate::environment::Ground;
use crate::player::Player;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct JumpForce(pub f32);

#[derive(Component)]
pub struct SecondJumpLeft(pub bool);

#[derive(Component)]
pub struct IsGrounded(pub bool);

#[derive(Component)]
pub struct MoveDirection(pub Vec3);

pub fn read_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut MoveDirection, With<Player>>,
    cameras: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut direction) in players.iter_mut() {
        for camera in cameras.iter() {
            direction.0 = Vec3::ZERO;

            if input.pressed(KeyCode::KeyW) {
                direction.0 += *camera.forward();
            }
            if input.pressed(KeyCode::KeyS) {
                direction.0 += *camera.back();
            }
            if input.pressed(KeyCode::KeyA) {
                direction.0 += *camera.left();
            }
            if input.pressed(KeyCode::KeyD) {
                direction.0 += *camera.right();
            }

            direction.0.y = 0.0;
            direction.0 = direction.0.normalize_or_zero();
        }
    }
}

pub fn move_player(
    time: Res<Time>,
    mut players: Query<(&mut Transform, &MovementSpeed, &MoveDirection), With<Player>>,
) {
    let scaled_speed = constants::PLAYER_MOVEMENT_SPEED * time.delta_seconds();

    for (mut player_transform, player_speed, direction) in players.iter_mut() {
        let movement = direction.0 * player_speed.0 * scaled_speed;
        player_transform.translation += movement;
    }
}

pub fn rotate_to_moving_direction(
    mut players: Query<(&mut Transform, &MoveDirection), With<Player>>,
) {
    for (mut player_transform, direction) in players.iter_mut() {
        let is_moving = direction.0.length_squared() > 0.0;
        if is_moving {
            player_transform.look_to(-direction.0, Vec3::Y);
        }
    }
}

pub fn do_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&JumpForce, &mut ExternalImpulse, &mut Velocity, &IsGrounded, &mut SecondJumpLeft), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (jump_force, mut ext_impulse, mut velocity, is_grounded, mut double_jump) in players.iter_mut() {
            if is_grounded.0 {
                velocity.linvel = Vec3::ZERO;
                ext_impulse.impulse = Vec3::new(0.0, jump_force.0, 0.0);

                double_jump.0 = true;
            } else {
                if double_jump.0 {
                    velocity.linvel = Vec3::ZERO;
                    ext_impulse.impulse = Vec3::new(0.0, jump_force.0, 0.0);
                }

                double_jump.0 = false;
            }
        }
    }
}

pub fn update_grounded(
    rapier_context: Res<RapierContext>,
    mut players: Query<(Entity, &mut IsGrounded), With<Player>>,
    grounds: Query<Entity, With<Ground>>,
) {
    for (player, mut is_grounded) in players.iter_mut() {
        for ground in grounds.iter() {
            is_grounded.0 = rapier_context.contact_pair(player, ground).is_some();
        }
    }
}

