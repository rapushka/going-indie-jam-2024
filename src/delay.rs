use bevy::prelude::*;

#[derive(Component)]
pub struct ShowAfterDelay(pub Timer);

pub struct DelayPlugin;

impl Plugin for DelayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                tick_show_after_delay_timers,
                show_after_time_out,
            ).chain())
        ;
    }
}

fn tick_show_after_delay_timers(
    time: Res<Time>,
    mut timers: Query<&mut ShowAfterDelay>,
) {
    for mut timer in timers.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn show_after_time_out(
    mut commands: Commands,
    mut timers: Query<(Entity, &ShowAfterDelay, &Visibility)>,
) {
    for (entity, timer, _visibility) in timers.iter_mut() {
        if !timer.0.finished() {
            continue;
        }

        commands.entity(entity)
            .insert(Visibility::Inherited)
            .remove::<ShowAfterDelay>()
        ;
    }
}