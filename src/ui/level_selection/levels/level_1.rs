use bevy::math::Vec3;
use bevy::prelude::*;

use crate::*;
use super::*;

pub fn load(
    parent: &mut ChildBuilder,
    assets: &ResMut<MyAssets>,
) {
    commons::create_ground(parent, assets, Vec3::ZERO, Vec3::ONE);
}