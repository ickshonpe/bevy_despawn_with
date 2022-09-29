use bevy::prelude::*;
use bevy_despawn_with::*;

#[derive(Component)]
struct A;

#[derive(Component)]
struct B;

fn count(query_a: Query<&A>, query_b: Query<&B>, query_ab: Query<(&A, &B)>) {
    let a_n = query_a.iter().count();
    let b_n = query_b.iter().count();
    let ab_n = query_ab.iter().count();
    println!();
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
        })
        .add_system(count.before("remove")) // 2
        .add_system(
            (|mut commands: Commands| {
                println!();
                println!("Remove all A components from entities with B components");
                commands.remove_all_filtered::<A, (Changed<A>, With<B>)>(); // 3
            })
            .label("remove"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            count.after("remove"), // 4
        )
        .run();
}
