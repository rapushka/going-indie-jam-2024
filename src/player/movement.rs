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

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut players: Query<(&mut Transform, &MovementSpeed), With<Player>>,
    cameras: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    animations: Res<Animations>,
    mut animators: Query<&mut AnimationPlayer>,
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
            let is_moving = direction.length_squared() > 0.0;
            if is_moving {
                player_transform.look_to(-direction, Vec3::Y);
            }

            // animation
            for mut animator in &mut animators {
                let key = if is_moving { &RUN } else { &IDLE };

                play_animation(&animations, &mut animator, &key);
            }
        }
    }
}

fn play_animation(
    animations: &Res<Animations>,
    animator: &mut Mut<AnimationPlayer>,
    key: &i32,
) {
    animator.play_with_transition(
        animations.0[key].clone_weak(),
        Duration::from_millis(250), // TODO: huh?
    )
        .repeat();
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

