use bevy::prelude::*;
use crate::tutors::{StartTutor, Tutor};

#[derive(Component, Default)]
pub struct OnLevelStarted;

#[derive(Component, Default)]
pub struct OnGroundedAfterHitInvisibleWall;

#[derive(Component, Default)]
pub struct OnDebugViewActivated;

pub(super) fn on_level_started(
    tutors: Query<Entity, (With<OnLevelStarted>, With<Tutor>)>,
    mut event: EventWriter<StartTutor>,
) {
    for tutor in tutors.iter() {
        event.send(StartTutor(tutor));
    }
}