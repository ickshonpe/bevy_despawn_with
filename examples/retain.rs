use bevy::prelude::*;
use bevy_despawn_with::retain::*;

#[derive(Component)]
struct DespawnTimer(f32);

#[derive(Component)]
struct X(i32);

#[derive(Component)]
struct Y;

fn spawn_entities(mut commands: Commands) {
    println!("Spawning 40 entities:");
    println!("\t20 with bundles (X(n),) for n in 0..20");
    println!("\tand 20 with bundles (X(n), Y) for n in 0..20");
    commands.spawn_batch((0..20).map(|n| (X(n),)));
    commands.spawn_batch((0..20).map(|n| (X(n), Y)));
}

fn count(xs_query: Query<(), With<X>>, ys_query: Query<(), With<Y>>) {
    let x_n = xs_query.iter().count();
    let y_n = ys_query.iter().count();
    println!();
    println!("Current entity count:");
    println!("\tnumber of entities with component X = {x_n}");
    println!("\tnumber of entities with component Y = {y_n}");
}

fn retain_entities(mut commands: Commands) {
    println!();
    println!("For each entity with an X component and without a Y component, retain that entity if and only if its inner value is less than 10.");
    commands.retain::<&X, Without<Y>>(|x| x.0 < 10);
}

fn main() {
    println!("Retain feature example\n");
    App::new()
        .add_startup_system(spawn_entities)
        .add_system(count)
        .add_system(retain_entities.after(count))
        .add_system_to_stage(CoreStage::PostUpdate, count)
        .run();
}
