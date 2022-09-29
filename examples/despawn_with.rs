use bevy::prelude::*;
use bevy_despawn_with::*;

// 1. spawn 30 entities
//  * 10 with A components
//  * 10 with B components
//  * 10 with A and B components
// 2. count the numbers of entities with each component
//      and print the result
// 3. despawn all entities with both A and B components
// 4. count again and print the result

#[derive(Component)]
struct A;

#[derive(Component)]
struct B;

fn count(query_a: Query<&A>, query_b: Query<&B>, query_ab: Query<(&A, &B)>) {
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
            // 1
            commands.spawn_batch((0..10).map(|_| (A, B)));
            commands.spawn_batch((0..10).map(|_| (A,)));
            commands.spawn_batch((0..10).map(|_| (B,)));
            println!("Spawned 30 entities:");
        })
        .add_system(count.before("despawn")) // 2
        .add_system(
            (|mut commands: Commands| {
                commands.despawn_all::<(With<A>, With<B>)>(); // 3
                println!();
                println!("After despawning all entities with both components A and B:");
            })
            .label("despawn"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            count.after("despawn"), // 4
        )
        .run();
}
