use bevy::prelude::*;
use crate::Order;
use crate::tutors::waypoints::move_to::*;

pub mod move_to;

pub struct Waypoint {
    pub position: Vec3,
    pub look_at: Vec3,
}

impl Waypoint {
    pub fn new(position: Vec3, look_at: Vec3) -> Self { Self { position, look_at } }
}

pub(super) struct WaypointsPlugin;

impl Plugin for WaypointsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                move_to,
                look_at,
            ).in_set(Order::GameLogic))
        ;
    }
}