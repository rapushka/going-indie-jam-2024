use bevy::audio::Volume;
use bevy::prelude::*;
use crate::AppState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Loading), play_music)
        ;
    }
}

fn play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("music"),
        AudioBundle {
            source: asset_server.load("audio/music.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.25)),
        },
    ));
}
