use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::constants;
use crate::player::Player;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct JumpForce(pub f32);

pub fn move_player(
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

pub fn do_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ext_impulses: Query<(&JumpForce, &mut ExternalImpulse, &mut Velocity), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (jump_force, mut ext_impulse, mut velocity) in ext_impulses.iter_mut() {
            velocity.linvel = Vec3::ZERO;
            ext_impulse.impulse = Vec3::new(0.0, jump_force.0, 0.0);
        }
    }
}
