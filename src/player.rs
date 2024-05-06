use bevy::prelude::*;
use bevy_inspector_egui::egui::FontFamily::Name;
use bevy_third_person_camera::*;

use crate::constants;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                move_player,
            ))
        ;
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MovementSpeed(f32);

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        bevy::core::Name::new("player"),
        Player,
        MovementSpeed(constants::PLAYER_MOVEMENT_SPEED),
        PbrBundle {
            mesh: meshes.add(Cuboid::default().mesh()),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        ThirdPersonCameraTarget,
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut players: Query<(&mut Transform, &MovementSpeed), With<Player>>,
    cameras: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let scaled_speed = constants::PLAYER_MOVEMENT_SPEED * time.delta_seconds();

    for (mut player_transform, player_speed) in players.iter_mut() {
        for camera in cameras.iter() {
            let mut direction = Vec3::ZERO;

            if input.pressed(KeyCode::KeyW) {
                direction += *camera.forward();
            }
            if input.pressed(KeyCode::KeyS) {
                direction += *camera.back();
            }
            if input.pressed(KeyCode::KeyA) {
                direction += *camera.left();
            }
            if input.pressed(KeyCode::KeyD) {
                direction += *camera.right();
            }

            direction.y = 0.0;

            direction = player_speed.0 * direction.normalize_or_zero();
            let movement = direction * scaled_speed;

            player_transform.translation += movement;
        }
    }
}