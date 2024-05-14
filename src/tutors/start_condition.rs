use bevy::prelude::*;
use crate::constants;
use crate::player::despawn::{KillPlayer, PlayerDead};
use crate::player::movement::invisible_walls::HitInvisibleWall;
use crate::player::movement::IsGrounded;
use crate::player::Player;
use crate::player::spawn::SpawnPlayer;
use crate::tutors::{StartTutor, Tutor};

#[derive(Component, Default)]
pub struct OnLevelStarted;

#[derive(Component, Default)]
pub struct OnHitInvisibleWall;

#[derive(Component, Default)]
pub struct OnDebugViewActivated;

#[derive(Component)]
pub struct OnSpawnAt(pub u8);

pub(super) fn on_respawn_at(
    mut dead_event: EventReader<PlayerDead>,
    tutors: Query<(Entity, &OnSpawnAt), (With<OnSpawnAt>, With<Tutor>)>,
    mut event: EventWriter<StartTutor>,
) {
    for e in dead_event.read() {
        for (tutor, observed_spawn_point) in tutors.iter() {
            if observed_spawn_point.0 == e.chunk_index {
                event.send(StartTutor(tutor));
            }
        }
    }
}

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
        if let Ok(_) = players.get(e.0) {
            for tutor in tutors.iter() {
                event.send(StartTutor(tutor));
            }
        }
    }
}

pub(super) fn on_debug_view_activated(
    input: Res<ButtonInput<KeyCode>>,
    tutors: Query<Entity, (With<OnDebugViewActivated>, With<Tutor>)>,
    mut event: EventWriter<StartTutor>,
) {
    if input.just_pressed(constants::controls::TOGGLE_DEBUG_VIEW) {
        for tutor in tutors.iter() {
            event.send(StartTutor(tutor));
        }
    }
}