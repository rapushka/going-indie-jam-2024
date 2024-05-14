use bevy::core::Name;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{Bundle, Commands, Component};
use crate::AppState::Gameplay;
use crate::OnAppState;
use crate::tutors::Tutor;

pub fn add_tutor<SC>(
    parent: &mut ChildBuilder,
    speeches: Vec<&'static str>,
) where SC: Default + Component {
    parent.spawn((
        Name::new("tutor speech"),
        Tutor {
            speeches
        },
        SC::default(),
        OnAppState(Gameplay),
    ));
}