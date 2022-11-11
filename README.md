# Bevy Despawn With

[![crates.io](https://img.shields.io/crates/v/bevy_despawn_with)](https://crates.io/crates/bevy_despawn_with)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_despawn_with)
[![crates.io](https://img.shields.io/crates/d/bevy_despawn_with)](https://crates.io/crates/bevy_despawn_with)

This crate implements an extension trait on `Commands`, `DespawnAllCommandsExt` which has two methods `despawn_all` and `despawn_all_recursive` for despawning multiple entities:

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

Then despawn some things:

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

Supports Bevy 0.8