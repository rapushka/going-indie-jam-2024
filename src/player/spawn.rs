use std::collections::HashMap;
use bevy::app::{App, Plugin};
use bevy::prelude::{Component, Entity, Event, IntoSystemConfigs, OnEnter};
use bevy::math::Vec3;
use crate::*;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnPlayer>()
            .add_event::<Kill>()
            .add_event::<Died>()

            .add_systems(OnEnter(AppState::Gameplay), (
                load_spawn_points,
                spawn_play_on_first_spawn_point,
            )
                .chain()
                .in_set(Order::GameLogic))
        ;
    }
}

#[derive(Component)]
pub struct SpawnPoint(u8);

#[derive(Event)]
pub struct SpawnPlayer {
    pub position: Vec3,
}

#[derive(Event)]
pub struct Kill(Entity);

#[derive(Event)]
pub struct Died(Entity);

#[derive(Resource)]
pub struct SpawnPointsMap(pub HashMap<u8, Entity>);

impl SpawnPointsMap {
    pub fn get(&self, index: u8) -> Entity { self.0[&index] }
}

fn load_spawn_points(
    mut commands: Commands,
) {
    let mut map = HashMap::new();

    new_spawn_point(&mut commands, &mut map, 0, Vec3::ZERO);
    new_spawn_point(&mut commands, &mut map, 1, Vec3 { x: 5.0, y: 2.1, z: 5.0 });

    commands.insert_resource(SpawnPointsMap(map));
}

fn new_spawn_point(
    mut commands: &mut Commands,
    spawn_points: &mut HashMap<u8, Entity>,
    index: u8,
    position: Vec3,
) {
    spawn_points.insert(index, commands.spawn((
        Name::new(format!("spawn point {}", index)),
        Transform::from_translation(position),
        SpawnPoint(index),
    )).id());
}

fn spawn_play_on_first_spawn_point(
    spawn_points: Res<SpawnPointsMap>,
    positions: Query<&Transform, With<SpawnPoint>>,
    mut spawn_player_event: EventWriter<SpawnPlayer>,
) {
    if let Ok(point) = positions.get(spawn_points.get(0)) {
        spawn_player_event.send(SpawnPlayer { position: point.translation });
    } else {  
        error!("something wrong with the spawn point");
    } 
}