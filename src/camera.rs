use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_third_person_camera::camera::*;
use crate::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera) // TODO: remove this one

            // .add_systems(OnEnter(AppState::Gameplay), spawn_camera)
        ;
    }
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        ThirdPersonCamera {
            cursor_lock_active: false,
            cursor_lock_key: KeyCode::Escape,
            mouse_sensitivity: 5.0,
            zoom: Zoom::new(3.0, 10.0),
            offset_enabled: true,
            offset: Offset::new(0.0, 1.0),
            ..default()
        },
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}