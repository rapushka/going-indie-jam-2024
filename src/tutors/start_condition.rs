use bevy::prelude::*;
use crate::player::despawn::KillPlayer;
use crate::player::movement::invisible_walls::HitInvisibleWall;
use crate::player::movement::IsGrounded;
use crate::player::Player;
use crate::tutors::{StartTutor, Tutor};

#[derive(Component, Default)]
pub struct OnLevelStarted;

#[derive(Component, Default)]
pub struct OnHitInvisibleWall;

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

pub(super) fn on_grounded_after_hit_invisible_wall(
    tutors: Query<Entity, (With<OnHitInvisibleWall>, With<Tutor>)>,
    mut event: EventWriter<StartTutor>,
    players: Query<&IsGrounded, (Changed<IsGrounded>, With<Player>, With<HitInvisibleWall>)>,
) {
    for is_grounded in players.iter() {
        if is_grounded.0 {
            for tutor in tutors.iter() {
                event.send(StartTutor(tutor));
            }
        }
    }
}

pub(super) fn on_player_kill_after_hit_invisible_wall(
    tutors: Query<Entity, (With<OnHitInvisibleWall>, With<Tutor>)>,
    mut event: EventWriter<StartTutor>,
    mut kill_player_event: EventReader<KillPlayer>,
    players: Query<&HitInvisibleWall>,
    test: Query<&Player>,
) {
    for e in kill_player_event.read() {
        info!("{}", test.get(e.0).is_ok());
        if let Ok(_) = players.get(e.0) {
            for tutor in tutors.iter() {
                event.send(StartTutor(tutor));
            }
        }
    }
}