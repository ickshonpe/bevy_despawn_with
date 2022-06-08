use bevy::prelude::*;
use bevy_despawn_with::retain::*;

#[derive(Component)]
struct DespawnTimer(f32);

#[derive(Component)]
struct X(i32);

#[derive(Component)]
struct Y;

fn count(
    xs_query: Query<(), With<X>>,
    ys_query: Query<(), With<Y>>,
) {
    let x_n = xs_query.iter().count();
    let y_n = ys_query.iter().count();
    println!("\tnumber of entities with component X = {x_n}");
    println!("\tnumber of entities with component Y = {y_n}");
}

fn main() {
    App::new()
    .add_startup_system(|mut commands: Commands| {
        commands.spawn_batch((0..20).map(|n| (X(n),)));
        commands.spawn_batch((0..20).map(|n| (X(n), Y)));
    })
    .add_system(count.before("despawn"))
    .add_system(
        (|mut commands: Commands| {
            commands.retain::<&X, Without<Y>>(|x| { x.0 < 10 }); 
        }).label("despawn")
    )
    .add_system_to_stage(
        CoreStage::PostUpdate,
        count.after("despawn") 
    )
    .run();
}