# Bevy Despawn With

This crate implements an extension trait on `Commands`, `DespawnAllCommandsExt` which has two methods `despawn_all` and `despawn_all_recursive` that despawn all entities satisfying a given query filter:

```rust
fn despawn_system(
    mut commands: Commands,
) {
    commands.despawn_all::<(With<People>, With<Shoes>, Without<Laces>)>();
}
```

## Usage

Add the dependency to your project's Cargo.toml `[dependencies]` section:

```toml
bevy_despawn_with = "0.13"
```

```rust
use bevy_despawn_with::DespawnAllCommandsExt;

fn despawn_system(mut commands: Commands) {
    // Despawn all entities with a MenuUiMarker component
    commands.despawn_all::<With<MenuUiMarker>>();

    // Despawn all entities without a SomeOtherMarker component, 
    // and despawn those entities descendants.
    commands.despawn_all_recursive::<Without<SomeOtherMarker>>();

    // Despawn all entities with a MenuUiMarker component, or with a changed GlobalTransform.
    commands.despawn_all::<Or<(With<MenUiMarker>, Changed<GlobalTransform>)>>();
}
```

## Optional Features

```toml
bevy_despawn_with = { version = "0.13", features = ["retain, remove"] }
```

## Examples

```
cargo run --example despawn_with
cargo run --example despawn_without
cargo run --example retain --features retain
cargo run --example retain_despawn_timer --features retain
cargo run --example remove_all
```

## Notes

Versions >= 0.9 support Bevy 0.8, 0.6 to 0.8 support Bevy 0.7, versions < 0.5 support Bevy 0.5.