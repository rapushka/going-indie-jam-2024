use bevy::prelude::*;
use crate::{AppState, constants, ui};

#[derive(Component)]
pub struct OnLevelStarted;

#[derive(Component)]
pub struct OnGroundedAfterHitInvisibleWall;

#[derive(Component)]
pub struct OnDebugViewActivated;

#[derive(Component)]
pub struct Tutor {
    speeches: Vec<String>,
}

#[derive(Component)]
pub struct SpeechBubble;

pub struct TutorsPlugin;

impl Plugin for TutorsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Loading), build_tutorial_speech)
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
    ))
        .with_children(|parent| {
            parent.spawn((
                ui::create::light_text_bundle(&asset_server, "abc", 32.0, ui::order::TUTOR_TEXT),
                SpeechBubble,
            ));
        })
    // .insert(Visibility::Hidden)
    ;
}