use std::time::Duration;
use bevy::prelude::*;
use bevy::utils::info;
use bevy_rapier3d::prelude::*;

use crate::{animations, constants};
use crate::environment::Ground;
use crate::player::Player;
use crate::animations::*;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct JumpForce(pub f32);

#[derive(Component)]
pub struct SecondJumpLeft(pub bool);

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
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(Entity, &JumpForce, &mut ExternalImpulse, &mut Velocity, &mut SecondJumpLeft), With<Player>>,
    grounds: Query<Entity, With<Ground>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (player, jump_force, mut ext_impulse, mut velocity, mut double_jump) in players.iter_mut() {
            for ground in grounds.iter() {
                if let Some(_contact_pair) = rapier_context.contact_pair(player, ground) {
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
}

