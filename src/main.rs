use bevy::prelude::*;
use bevy_inspector_egui::quick::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
        ))

        .add_systems(Startup, (
            spawn_floor,
            spawn_camera,
            spawn_light,
            spawn_player,
        ))
        .run();
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pbr = PbrBundle {
        mesh: meshes.add(Cuboid::default().mesh()),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };

    commands.spawn(pbr);
}

fn spawn_light(
    mut commands: Commands,
) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 1_000_000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    commands.spawn(light);
}

fn spawn_camera(
    mut commands: Commands,
) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(camera);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let name = Name::new("floor");
    let pbr = PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
        material: materials.add(Color::DARK_GREEN),
        ..default()
    };

    commands.spawn((name, pbr));
}