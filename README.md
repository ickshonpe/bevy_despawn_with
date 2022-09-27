# Bevy Despawn With

This crate implements an extension trait on `Commands`, `DespawnAllCommandsExt` which has two methods `despawn_all` and `despawn_all_recursive` that allow you to despawn all entities that satisfy a given query filter with a single statement.

## Version 0.9

Supports Bevy 0.8

## Version 0.8

Revamped `RetainCommandsExt`, now only has two methods
* `retain`
* `retain_recursive`

Similar to Vec's retain, but for queries.
Example of what is very probably a terrible anti-pattern:

```rust
use bevy_despawn_with::retain::*;

#[derive(Component)]
struct DespawnTimer(f32);

fn update_despawn_timers(
    mut commands: Commands,
    time: Res<Time>,        
) {
    let delta = time.delta_seconds();
    commands.retain::<&mut DespawnTimer, ()>(move |despawn_timer| { 
        dt.0 -= delta;
        0. < dt.0   // despawns if false, that is once the time runs out
    });
}     
```
Remember that Commands are not applied immediately. The DespawnTimer components won't be updated until the next stage boundary.

Feature-gated for all its silliness, to enable ```retain``` use:
```toml
bevy_despawn_with = { version = "0.8", features = ["retain"] }
```

## Version 0.7

Massive API improvements 
* `despawn_with` and `despawn_with_recursive` renamed to `despawn_all` and `despawn_all_recursive`.
* The methods take a query filter instead of a component tuple. For example:

    ```rust
    commands.despawn_all::<(With<People>, With<Shoes>, Without<Laces>)>();
    ```
    despawns all people wearing shoes without laces.

* There are no seperate `despawn_with_all` methods any more.

No longer uses SystemState. Performance should be much better (assumed, not benchmarked).

## Version 0.6

Adds Bevy 0.7 support.

#
## Contrived Example & Motivation

Marker components are a common pattern in Bevy:
```rust
#[derive(Component)]
struct MenuUiMarker;

fn spawn_menu(mut commands: Commands) {
    commands.spawn_bundle(NodeBundle {
        ..Default::default()
    })
    .insert(MenuUiMarker);
    // .. and so on.
    // The developer spawns a bunch of UI entities and then 
    // inserts a MenuUiMarker component for each one.
}

fn despawn_all<C: Component>(
    mut commands: Commands,
    query: Query<Entity, With<C>>,
) {
    query.for_each(|entity| {
        commands.despawn(entity);
    });
}

pub struct MenuScreenPlugin;

impl Plugin for MenuScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MenuScreen)
            .with_system(spawn_menu)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MenuScreen)
            .with_system(despawn_all::<MenuUiMarker>)
        )
        // ... 
        // rest of whatever MenuScreenPlugin needs to work
        ;
    }
}
```

`DespawnAllCommandsExt` makes this a little more ergonomic:

```rust
use bevy_despawn_with::*;

fn despawn_system(mut commands: Commands) {

    // Despawn all entities with a MenuUiMarker component
    commands.despawn_all::<With<MenuUiMarker>>();

    // Despawn all entities without a SomeOtherMarker component, 
    // and despawn those entities descendants.
    commands.despawn_all_recursive::<Without<SomeOtherMarker>>();


    // Methods can take any query filter.
    // The following despawns any entity with a MenuUiMarker 
    // component, without a SomeOtherMarker component, 
    // and/or a changed GlobalTransform.
    commands.despawn_all::<(Or<
        With<MenUiMarker>, 
        Without<SomeOtherMarker>, 
        Changed<GlobalTransform>
    )>();
}
```
so if we want we could replace the despawn_all system in the menu screen example with:
```rust
app.add_system_set(
    SystemSet::on_exit(AppState::MenuScreen)
    .with_system(|mut commands: Commands| 
        commands.despawn_all::<With<MenuUiMarker>>()
    )
);
```

The descendants of entities despawned with `despawn_all_recursive` 
will be despawned regardless of whether they satisfy the query filter or not.
#

## Usage

Add the following to your project's Cargo.toml `[dependencies]` section:

```toml
bevy_despawn_with = "0.9"
```
and you are ready to go.

#

## Examples

```
cargo run --example despawn_with
cargo run --example despawn_without
cargo run --example retain --features retain
cargo run --example despawn_timer --features retain
```