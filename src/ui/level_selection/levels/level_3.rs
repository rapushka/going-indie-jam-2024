use bevy::math::Vec3;
use bevy::prelude::*;
use commons::*;
use tutors::create::*;

use crate::*;
use crate::stars::create_star;
use crate::tutors::start_condition::{OnDebugViewActivated, OnHitInvisibleWall, OnLevelStarted};
use crate::tutors::waypoints::Waypoint;
use super::*;

pub fn load(
    root: &mut ChildBuilder,
    assets: &ResMut<MyAssets>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    create_spawn_point(root, 0, vec3(0.0, 1.0, 0.0));
    create_ground(root, assets, vec3(7.0, 0.0, 1.0), vec3(20.0, 1.0, 20.0));
    create_ground(root, assets, vec3(12.0, 0.0, 12.0), vec3(12.0, 20.0, 3.0));

    create_star(root, assets, vec3(7.0, 4.5, 1.0));

    create_invisible_ground(root, assets, vec3(7.0, 3.5, -3.0), vec3(1.0, 1.0, 1.0));
    create_invisible_ground(root, assets, vec3(7.0, 3.5, 6.0), vec3(1.0, 1.0, 1.0));
    create_invisible_ground(root, assets, vec3(17.0, 3.5, 1.0), vec3(1.0, 1.0, 1.0));
    create_invisible_ground(root, assets, vec3(20.0, 5.5, 1.0), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(25.0, 5.0, 1.5), vec3(1.0, 1.0, 1.0));
    create_ground(root, assets, vec3(22.5, 7.5, 5.5), vec3(1.0, 1.0, 1.0));
    create_ground(root, assets, vec3(20.5, 0.0, 7.5), vec3(0.1, 10.0, 0.1));

    create_spawn_point(root, 2, vec3(10.0, 100.0, 10.0));
    create_chunk(root, meshes, materials, 2, Color::GOLD, vec2(25.0, 25.0), vec2(10.0, 10.0));

    create_ground(root, assets, vec3(0.5, 3.0, -25.0), vec3(1_000.0, 1.0, 1.1));

    add_tutor::<OnLevelStarted>(
        root,
        vec![
            "This level isn't finished",
            "And it won't be.",
            "I ran out of time:^)\nsorry",
            "But you can still play around here, if you want so",
            "Anyway it's getting late",
            "Good night, and thank you for playing<3",
        ],
        vec![
            Waypoint::new(Vec3::new(0.0, 2.0, 5.0), Vec3::new(0.0, 2.0, 0.0))
        ],
    );
}

fn vec3(x: f32, y: f32, z: f32) -> Vec3 { Vec3::new(x, y, z) }

fn vec2(x: f32, y: f32) -> Vec2 { Vec2::new(x, y) }
