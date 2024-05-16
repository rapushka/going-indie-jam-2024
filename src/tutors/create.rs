use bevy::core::Name;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Component;

use crate::AppState::Gameplay;
use crate::OnAppState;
use crate::tutors::start_condition::OnSpawnAt;
use crate::tutors::Tutor;
use crate::tutors::waypoints::Waypoint;

pub fn add_tutor<SC>(
    parent: &mut ChildBuilder,
    speeches: Vec<&'static str>,
    waypoints: Vec<Waypoint>,
) where SC: Default + Component {
    parent.spawn((
        Name::new("tutor speech"),
        Tutor { speeches, waypoints },
        SC::default(),
        OnAppState(Gameplay),
    ));
}

pub fn add_tutor_on_respawn_at(
    parent: &mut ChildBuilder,
    spawn_point: u8,
    speeches: Vec<&'static str>,
    waypoints: Vec<Waypoint>,
) {
    parent.spawn((
        Name::new("tutor speech"),
        Tutor { speeches, waypoints },
        OnSpawnAt(spawn_point),
        OnAppState(Gameplay),
    ));
}