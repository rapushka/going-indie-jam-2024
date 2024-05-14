use bevy::prelude::*;
use bevy_text_animation::TextSimpleAnimator;

use crate::{AppState, constants, GameState, Order, ui};
use crate::tutors::start_condition::*;

pub mod start_condition;
pub mod create;

const SPEECH_SPEED: f32 = 50.0;

#[derive(Component)]
pub struct Tutor {
    speeches: Vec<&'static str>,
}

#[derive(Component)]
pub struct PlayingTutor {
    current_step: u8,
    speeches: Vec<&'static str>,
}

impl PlayingTutor {
    pub fn get_current(&self) -> Option<&'static str> { self.speeches.get(self.current_step as usize).copied() }
}

#[derive(Event)]
pub struct StartTutor(pub Entity);

#[derive(Event)]
pub struct PlayNextTutorStep(pub Entity);

#[derive(Component)]
pub struct SpeechBubble;

pub struct TutorsPlugin;

impl Plugin for TutorsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StartTutor>()
            .add_event::<PlayNextTutorStep>()

            .add_systems(OnEnter(AppState::Loading), build_tutorial_speech)

            .add_systems(OnEnter(AppState::Gameplay), on_level_started)

            .add_systems(Update, (
                on_grounded_after_hit_invisible_wall,
                on_player_kill_after_hit_invisible_wall,
            ).in_set(Order::Tutor))

            .add_systems(Update, (
                start_tutor,
                skip_tutor.run_if(in_state(GameState::Tutor)),
                play_next_tutor_step,
                set_tutor_visibility,
            )
                .chain()
                .run_if(in_state(AppState::Gameplay)),
            )
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
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for e in event.read() {
        if let Ok(tutor) = tutors.get(e.0) {
            for bubble in bubbles.iter() {
                commands.entity(bubble)
                    .insert(PlayingTutor {
                        current_step: 0,
                        speeches: tutor.speeches.clone(),
                    })
                ;
            }

            next_game_state.set(GameState::Tutor);
            commands.entity(e.0).despawn_recursive();
        }
    }
}

fn skip_tutor(
    input: Res<ButtonInput<KeyCode>>,
    mut bubbles: Query<&mut PlayingTutor, With<SpeechBubble>>,
) {
    let key_to_skip_tutor = vec![KeyCode::Space, KeyCode::Escape];

    if key_to_skip_tutor.iter().any(|k| input.just_pressed(*k)) {
        for mut bubble in bubbles.iter_mut() {
            bubble.current_step += 1;
            // event.send(PlayNextTutorStep(bubble));
        }
    }
}

fn play_next_tutor_step(
    mut commands: Commands,
    bubbles: Query<(Entity, &PlayingTutor), (Changed<PlayingTutor>, With<SpeechBubble>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (bubble, playing_tutor) in bubbles.iter() {
        let mut bubble_entity = commands.entity(bubble);

        if let Some(next_step) = playing_tutor.get_current() {
            bubble_entity.insert(TextSimpleAnimator::new(next_step, SPEECH_SPEED));

            next_game_state.set(GameState::Tutor);
        } else {
            bubble_entity.remove::<TextSimpleAnimator>();

            next_game_state.set(GameState::Playing);
        }
    }
}

fn set_tutor_visibility(
    mut commands: Commands,
    tutors: Query<Entity, With<SpeechBubble>>,
    mut game_state_transition: EventReader<StateTransitionEvent<GameState>>,
) {
    for e in game_state_transition.read() {
        let visibility = if e.after == GameState::Tutor { Visibility::Inherited } else { Visibility::Hidden };
        for tutor in tutors.iter() {
            commands.entity(tutor).insert(visibility);
        }
    }
}

