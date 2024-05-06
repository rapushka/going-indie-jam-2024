use bevy::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_floor,
                spawn_light,
            ))
        ;
    }
}

fn spawn_light(
    mut commands: Commands,
) {
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1_000_000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
    ));
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("floor"),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
            material: materials.add(Color::DARK_GREEN),
            ..default()
        },
    ));
}