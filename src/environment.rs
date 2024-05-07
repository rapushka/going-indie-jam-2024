use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 1_000.0,
            })

            .insert_resource(ClearColor(Color::hex("80aaa7").unwrap()))

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
                color: Color::hex("ffe500").unwrap(),
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
    let material = materials.add(Color::hex("557d55").unwrap());

    let modified_material = materials.get_mut(&material).unwrap();
    modified_material.reflectance = 0.0;

    commands.spawn((
        Name::new("floor"),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
            material: material,
            ..default()
        },
        // Physics
        Collider::compound(vec![(
            Vec3::new(0.0, -0.1, 0.0),
            Quat::IDENTITY,
            Collider::cuboid(7.5, 0.1, 7.5)
        )]),
    ));
}