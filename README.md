# Bevy Despawn With

This crate implements an extension trait on `Commands`, `DespawnWithCommandsExt` which has two generic helper functions `despawn_with` and `despawn_recursive_with` that allow you to despawn all entities with a specified component with a single statement.

## Version 0.5

```despawn_with_all``` and ```despawn_with_all_recursive``` added to DespawnWithCommandsExt that takes a tuple of up to five components:

```rust
    // despawns any entity which has
    // all of the components A, B, and C 
    commands.despawn_with_all::<(A, B, C)>();
```


## Version 0.3 Update

0.3 implements a second extension trait `RetainCommandsExt`, with four functions:
* `retain`
* `retain_mut`
* `retain_recursive`
* `retain_recursive_mut`

that are similar to Vec's retain.
Example of what is very probably a terrible anti-pattern:

```rust
use bevy_despawn_with::RetainCommandsExt;

#[derive(Component)]
struct DespawnTimer(f32);

fn update_despawn_timers(
    mut commands: Commands,
    time: Res<Time>,        
) {
    let delta = time.delta_seconds();
    commands.retain_recursive_mut(move |dt: &mut DespawnTimer| { 
        dt.0 -= delta;
        0. < dt.0   // despawns if false, that is once the time runs out
    });
}     
```


## Contrived Example

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

The `DespawnWithExt` makes this a little more ergonomic:

```rust
use bevy_despawn_with::DespawnWithExt;

fn despawn_system(mut commands: Commands) {
    // Despawn all entities with a MenuUiMarker component
    commands.despawn_with::<MenuUiMarker>();

    // Despawn all entities with a MenUiMarker component, 
    // and despawn those entities descendants.
    commands.despawn_recursive_with::<MenuUiMarker>();

    // .. second statement here does nothing of course as all
    // entities with MenuUiMarker are already despawned.
}

// so the despawn_all system in the menu screen example becomes:
fn despawn_all<C: Component>(
    mut commands: Commands,
) {
    commands.despawn_with::<C>(entity);
}
```

The descendants of entities despawned with `despawn_recursive_with` 
will be despawned regardless of whether they have the specified marker component.

## Usage

Add the following to your project's Cargo.toml `[dependencies]` section:

```toml
bevy_despawn_with = "0.5"
```
and you are ready to go.

## Notes

* Supports Bevy 0.6
