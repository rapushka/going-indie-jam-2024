use bevy::prelude::*;
use bevy::scene::SceneInstance;
use bevy_third_person_camera::*;
use bevy_toon_shader::ToonShaderMaterial;

use crate::constants;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                move_player,
                customize_scene_materials,
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
    asset_server: Res<AssetServer>,
) {
    let scale = Vec3 { x: 0.5, y: 0.5, z: 0.5 };

    commands.spawn((
        Name::new("player"),
        Player,
        MovementSpeed(constants::PLAYER_MOVEMENT_SPEED),
        ThirdPersonCameraTarget,
        SceneBundle {
            scene: asset_server.load("models/Character.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(scale),
            ..default()
        },
        CustomizeMaterial,
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

            // rotate player to moving direction
            if direction.length_squared() > 0.0 {
                player_transform.look_to(-direction, Vec3::Y);
            }
        }
    }
}

// ---


#[derive(Component)]
struct CustomizeMaterial;

pub fn customize_scene_materials(
    unloaded_instances: Query<(Entity, &SceneInstance), With<CustomizeMaterial>>,
    handles: Query<(Entity, &Handle<StandardMaterial>)>,
    pbr_materials: Res<Assets<StandardMaterial>>,
    scene_manager: Res<SceneSpawner>,
    mut custom_materials: ResMut<Assets<ToonShaderMaterial>>,
    mut cmds: Commands,
) {
    for (entity, instance) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(entity).remove::<CustomizeMaterial>();
        }
        // Iterate over all entities in scene (once it's loaded)
        let handles = handles.iter_many(scene_manager.iter_instance_entities(**instance));
        for (entity, material_handle) in handles {
            let Some(material) = pbr_materials.get(material_handle) else { continue; };
            let custom = custom_materials.add(ToonShaderMaterial::default());
            cmds.entity(entity).insert(custom).remove::<Handle<StandardMaterial>>();
        }
    }
}