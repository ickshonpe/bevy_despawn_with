use bevy::prelude::*;
use bevy_despawn_with::*;

#[derive(Component)]
struct A;

#[derive(Component)]
struct B;

fn count(
    query_a: Query<&A>,
    query_b: Query<&B>,
    query_ab: Query<(&A, &B)>
) {
    let a_n = query_a.iter().count();
    let b_n = query_b.iter().count();
    let ab_n = query_ab.iter().count();
    println!("\tnumber of entities with component A = {a_n}");
    println!("\tnumber of entities with component B = {b_n}");
    println!("\tnumber of entities with both components A and B = {ab_n}");
}

fn main() {
    App::new()
    .add_startup_system(|mut commands: Commands| {
        commands.spawn_batch((0..10).map(|_| (A, B)));
        commands.spawn_batch((0..10).map(|_| (A,)));
        commands.spawn_batch((0..10).map(|_| (B,)));
        println!("Spawned 30 entities:");
    })
    .add_system(count.before("despawn")) 
    .add_system(
        (|mut commands: Commands| {
            commands.despawn_all::<Or<(Without<A>, Without<B>)>>(); 
            println!();
            println!("After despawning all entities without both A and B components:");
        }).label("despawn")
    )
    .add_system_to_stage(
        CoreStage::PostUpdate,
        count.after("despawn") 
    )
    .run();

}