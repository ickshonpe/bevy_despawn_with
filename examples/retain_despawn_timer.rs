use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_despawn_with::retain::*;

#[derive(Component)]
struct DespawnTimer(f32);

fn spawn_timers(mut commands: Commands) {
    commands.spawn_batch((1..=10).map(|n| (DespawnTimer(n as f32),)));
}

fn update_despawn_timers(mut commands: Commands, time: Res<Time>) {
    let delta = time.delta_seconds();
    commands.retain::<&mut DespawnTimer, ()>(move |mut dt| {
        dt.0 -= delta;
        0. < dt.0 // despawns if false, that is once the time runs out
    });
}

fn update(
    mut t: Local<f32>,
    time: Res<Time>,
    query: Query<(), With<DespawnTimer>>,
    mut event_writer: EventWriter<AppExit>,
) {
    *t -= time.delta_seconds();
    if *t < 0. {
        *t = 1.0;
        println!("Remaining despawn timers: {}", query.iter().count());
    }

    if 13.0 < time.seconds_since_startup() {
        event_writer.send(AppExit);
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system(spawn_timers)
        .add_system(update_despawn_timers)
        .add_system(update)
        .run();
}
