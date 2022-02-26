# Bevy Despawn With

This crate implements an extension trait on `Commands`, `DespawnWithExt` which has two generic helper functions `despawn_with` and `despawn_recursive_with` that allow you to despawn all entities with a specified component with a single statement.

It's kind of a trivial crate but, despite the use of Marker components everywhere, I haven't seen this idea of using an extension trait out in the wild. So I thought I'd share it, as it seems quite neat.

## Contrived Example

Common pattern in Bevy is giving entities marker Components to easily despawn groups of entities. 

```rust
#[derive(Component)]
struct MenuUiMarker;

fn spawn_menu(mut commands: Commands) {
    commands.spawn_bundle(NodeBundle {
        ..Default::default()
    })
    .insert(MenuUiElement);
    // .. 
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
        // rest of whatever TitleScreenPlugin needs to work
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

// so you can replace the system in the menu screen example with:

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
bevy_despawn_with = "0.1"
```
and you are ready to go.

## Notes

* Supports Bevy 0.6
* Not optimised, allocates a Vec every time. 

    Only easy way I know how to avoid the allocation is to use unsafe.

    Will update the crate once I find
    a better alternative.

## Todo
*  A `despawn_with_all` function that can take 
more than one marker component, or maybe the existing functions could be made to accept a tuple of components.