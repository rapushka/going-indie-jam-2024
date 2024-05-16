use bevy::prelude::*;
use crate::constants;

#[derive(Component)]
pub struct MoveTo(Vec3);

#[derive(Component)]
pub struct LookAt(Vec3);

pub(super) fn move_to(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Transform, &MoveTo)>,
    time: Res<Time>,
) {
    for (e, mut transform, move_to) in entities.iter_mut() {
        let position = transform.translation;
        let destination = move_to.0;
        let speed = constants::MOVE_TO_SPEED;

        let direction = (destination - position).normalize();
        let distance = position.distance(destination);
        let move_distance = speed * time.delta_seconds();

        if distance <= move_distance {
            transform.translation = destination;
            commands.entity(e).remove::<MoveTo>();
        }

        transform.translation = position + direction * move_distance;
    }
}

pub(super) fn look_at(
    mut entities: Query<(&mut Transform, &LookAt)>,
    time: Res<Time>,
) {
    for (mut transform, look_at) in entities.iter_mut() {
        let target_position = look_at.0;

        let current_position = transform.translation;

        let direction = target_position - current_position;

        let rotation = Quat::from_rotation_arc(Vec3::Z, direction.normalize());

        let rotation_speed = constants::LOOK_AT_SPEED;
        let rotation_amount = rotation_speed * time.delta_seconds();
        let interpolated_rotation = Quat::slerp(transform.rotation, rotation, rotation_amount);

        transform.rotation = interpolated_rotation;
    }
}