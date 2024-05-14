use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::environment::InvisibleWall;
use crate::player::Player;

#[derive(Component)]
pub struct HitInvisibleWall;

pub fn find_invisible_wall_colliding(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut players: Query<Entity, With<Player>>,
    walls: Query<Entity, With<InvisibleWall>>,
) {
    for player in players.iter_mut() {
        for wall in walls.iter() {
            if rapier_context.contact_pair(player, wall).is_some() {
                commands.entity(player)
                    .insert(HitInvisibleWall);
                return;
            }
        }
    }
}