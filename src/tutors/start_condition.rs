use bevy::prelude::*;
use crate::tutors::Tutor;

#[derive(Component, Default)]
pub struct OnLevelStarted;

#[derive(Component, Default)]
pub struct OnGroundedAfterHitInvisibleWall;

#[derive(Component, Default)]
pub struct OnDebugViewActivated;

pub(super) fn on_level_started(
    tutors: Query<Entity, (With<OnLevelStarted>, With<Tutor>)>,
) {
    for tutor in tutors.iter() {
        
    }
}