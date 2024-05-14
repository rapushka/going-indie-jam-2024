use bevy::prelude::*;
use bevy_text_animation::TextSimpleAnimator;
use crate::{AppState, constants, ui};
use crate::tutors::start_condition::*;

pub mod start_condition;
pub mod create;

#[derive(Component)]
pub struct Tutor {
    speeches: Vec<&'static str>,
}

#[derive(Event)]
pub struct StartTutor(pub Entity);

#[derive(Component)]
pub struct SpeechBubble;

pub struct TutorsPlugin;

impl Plugin for TutorsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StartTutor>()

            .add_systems(OnEnter(AppState::Loading), build_tutorial_speech)

            .add_systems(OnEnter(AppState::Gameplay), on_level_started)

            .add_systems(Update, (
                start_tutor,
            ))
        ;
    }
}

fn build_tutorial_speech(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("Speech Holder"),
        NodeBundle {
            style: constants::styles::SPEECH_BUBBLE,
            z_index: ui::order::TUTOR,
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.95).into(),
            ..default()
        },
        SpeechBubble,
    ))
        .insert(Visibility::Hidden)
        
        .with_children(|parent| {
            parent.spawn((
                Name::new("text"),
                SpeechBubble,
                ui::create::light_text_bundle(&asset_server, "", 32.0, ui::order::TUTOR_TEXT)
            ))
                .insert(Visibility::Hidden)
            ;
        })

    ;
}

fn start_tutor(
    mut commands: Commands,
    mut event: EventReader<StartTutor>,
    tutors: Query<&Tutor>,
    bubbles: Query<Entity, With<SpeechBubble>>,
) {
    for e in event.read() {
        if let Ok(tutor) = tutors.get(e.0) {
            for bubble in bubbles.iter() {
                commands.entity(bubble)
                    .insert(animated_text(tutor.speeches[0]))
                    .insert(Visibility::Inherited)
                ;
            }
        }
    }
}

fn animated_text(text: &str) -> TextSimpleAnimator {
    const SPEED: f32 = 50.0;
    TextSimpleAnimator::new(text, SPEED)
}