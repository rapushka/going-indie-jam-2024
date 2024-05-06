use bevy::prelude::*;
use bevy_toon_shader::*;
use std::f32::consts::PI;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ToonShaderPlugin)

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
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(2.0, 2.0, 2.0),
                rotation: Quat::from_euler(EulerRot::XYZ, -PI / 4., PI / 6., 0.),
                ..default()
            },
            ..default()
        },
        // PointLightBundle {
        //     point_light: PointLight {
        //         color: Color::hex("ffe500").unwrap(),
        //         intensity: 1_000_000.0,
        //         ..default()
        //     },
        //     transform: Transform::from_xyz(0.0, 5.0, 0.0),
        //     ..default()
        // },
        ToonShaderSun,
    ));
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ToonShaderMaterial>>,
) {
    let material = materials.add(ToonShaderMaterial {
        color: Color::hex("557d55").unwrap(),
        ambient_color: Color::hex("e39347").unwrap(),
        ..default()
    });
    // let material = materials.add(Color::hex("557d55").unwrap());

    // let modified_material = materials.get_mut(&material).unwrap();
    // modified_material.reflectance = 0.0;

    commands.spawn((
        Name::new("not floor"),
        MaterialMeshBundle {
            mesh: meshes.add(Sphere::default().mesh()),
            material: material.clone(),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("floor"),
        MaterialMeshBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
            material,
            ..default()
        },
    ));
}