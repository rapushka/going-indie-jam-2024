use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::MyAssets;
use crate::stars::Star;

pub fn create_star(
    parent: &mut ChildBuilder,
    assets: &ResMut<MyAssets>,
    position: Vec3,
) {
    let sizes = Vec3::ONE * 1.0;

    parent.spawn((
        Name::new("star"),
        Star,
        SceneBundle {
            scene: assets.star.clone(),
            transform: Transform::from_translation(position).with_scale(sizes),
            ..default()
        },
        Collider::ball(sizes.x),
    ));
}