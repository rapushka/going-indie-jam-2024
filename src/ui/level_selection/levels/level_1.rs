use bevy::math::Vec3;
use bevy::prelude::*;

use crate::*;
use super::*;

pub fn load(
    parent: &mut ChildBuilder,
    assets: &ResMut<MyAssets>,
) {
    commons::create_ground(parent, assets, Vec3::ZERO, Vec3::new(2.0, 1.0, 2.0));

    commons::create_spawn_point(parent, 0, Vec3::new(0.0, 0.1, 0.0));
}