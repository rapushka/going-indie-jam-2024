use bevy::math::Vec3;
use bevy::prelude::*;

use commons::*;

use crate::*;
use crate::stars::create_star;

use super::*;

pub fn load(
    root: &mut ChildBuilder,
    assets: &ResMut<MyAssets>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    create_spawn_point(root, 0, vec3(0.0, 1.0, 0.0));
    create_ground(root, assets, Vec3::ZERO, vec3(5.0, 1.0, 6.0));

    create_invisible_wall(root, vec3(5.0, 5.0, 0.0), vec3(0.25, 10.0, 10.0));

    create_spawn_point(root, 1, vec3(7.0, 8.5, 1.0));
    create_invisible_ground(root, assets, vec3(7.0, 3.5, 1.0), vec3(3.0, 1.0, 3.0));

    create_star(root, assets, vec3(-12.0, 5.0, 1.0));
    create_empty_ground(root, assets, vec3(-12.0, 3.0, 1.0), vec3(3.0, 1.0, 3.0));
    create_chunk(root, meshes, materials, 1, Color::GREEN, vec2(-12.0, 1.0), vec2(12.0, 12.0));

    create_star(root, assets, vec3(9.0, 4.5, 3.0));
    create_ground(root, assets, vec3(9.0, 2.0, 3.0), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(12.0, 3.0, 1.0), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(15.0, 2.75, -3.0), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(21.0, 5.0, -2.0), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(26.0, 7.0, 2.0), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(30.0, 10.0, 1.25), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(40.1, 12.5, 1.0), vec3(1.0, 1.0, 1.0));

    create_chunk(root, meshes, materials, 3, Color::BLUE, vec2(55.0, 11.0), vec2(32.0, 32.0));
    create_spawn_point(root, 3, vec3(45.0, 13.5, 1.0));
    create_ground(root, assets, vec3(45.0, 12.5, 1.0), vec3(3.0, 1.0, 3.0));

    create_ground(root, assets, vec3(45.0, 15.0, 7.5), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(50.5, 18.0, 17.5), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(55.0, 22.0, 15.0), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(65.0, 23.0, 14.5), vec3(1.0, 1.0, 1.0));

    create_ground(root, assets, vec3(75.0, 24.0, 14.5), vec3(1.0, 1.0, 1.0));

    create_empty_ground(root, assets, vec3(80.0, 27.5, 14.5), vec3(1.0, 1.0, 1.0));
    create_empty_ground(root, assets, vec3(85.0, 29.5, 14.5), vec3(1.0, 1.0, 1.0));
    create_empty_ground(root, assets, vec3(90.0, 31.0, 14.5), vec3(1.0, 1.0, 1.0));
    create_empty_ground(root, assets, vec3(95.0, 33.5, 14.5), vec3(1.0, 1.0, 1.0));
    create_empty_ground(root, assets, vec3(100.0, 35.5, 14.5), vec3(1.0, 1.0, 1.0));
    create_empty_ground(root, assets, vec3(105.0, 37.0, 14.5), vec3(1.0, 1.0, 1.0));
    create_empty_ground(root, assets, vec3(115.0, 39.0, 14.5), vec3(1.0, 1.0, 1.0));

    create_spawn_point(root, 2, vec3(10.0, 1_000.0, 10.0));
    create_chunk(root, meshes, materials, 2, Color::GOLD, vec2(100.0, 25.0), vec2(50.0, 50.0));
    create_star(root, assets, vec3(100.0, 40.0, 25.0));
    create_ground(root, assets, vec3(100.0, 37.5, 25.0), vec3(10.0, 1.0, 10.0));
}

fn vec3(x: f32, y: f32, z: f32) -> Vec3 { Vec3::new(x, y, z) }

fn vec2(x: f32, y: f32) -> Vec2 { Vec2::new(x, y) }
