use bevy::prelude::*;
use crate::constants;

#[derive(Component)]
pub struct MoveTo(pub Vec3);

#[derive(Component)]
pub struct LookAt(pub Vec3);

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
) {
    for (mut transform, look_at) in entities.iter_mut() {
        transform.look_at(look_at.0, Vec3::Y);
    }
}