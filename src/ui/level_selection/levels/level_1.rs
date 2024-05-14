use bevy::math::Vec3;
use bevy::prelude::*;
use commons::*;
use tutors::create::add_tutor;

use crate::*;
use crate::stars::create_star;
use crate::tutors::start_condition::{OnHitInvisibleWall, OnLevelStarted};
use super::*;

pub fn load(
    root: &mut ChildBuilder,
    assets: &ResMut<MyAssets>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    create_spawn_point(root, 0, Vec3::new(0.0, 0.1, 0.0));
    create_ground(root, assets, Vec3::ZERO, Vec3::new(3.0, 1.0, 3.0));

    create_star(root, assets, Vec3::new(7.0, 4.5, 1.0));
    create_ground(root, assets, Vec3::new(7.0, 2.0, 1.0), Vec3::new(3.0, 1.0, 3.0));

    create_star(root, assets, Vec3::new(3.0, 9.0, 9.0));
    create_ground(root, assets, Vec3::new(3.0, 6.5, 9.0), Vec3::new(3.0, 1.0, 3.0));

    create_invisible_wall(root, Vec3::new(-3.0, 16.5, 9.0), Vec3::new(0.25, 9.0, 10.0));
    create_spawn_point(root, 1, Vec3::new(-9.0, 8.1, 9.0));
    create_ground(root, assets, Vec3::new(-11.0, 8.0, 9.0), Vec3::new(4.0, 1.0, 3.0));

    create_star(root, assets, Vec3::new(-20.0, 10.5, 2.0));
    create_ground(root, assets, Vec3::new(-20.0, 8.0, 6.0), Vec3::new(3.0, 1.0, 6.0));

    create_chunk(root, meshes, materials, 1, Color::RED, vec2(-20.0, 6.0), vec2(12.0, 24.0));

    add_tutor::<OnLevelStarted>(root, vec!["I want all these stars so much!"]);
    add_tutor::<OnHitInvisibleWall>(root, vec!["Ouch"]);
}

fn vec3(x: f32, y: f32, z: f32) -> Vec3 { Vec3::new(x, y, z) }

fn vec2(x: f32, y: f32) -> Vec2 { Vec2::new(x, y) }
