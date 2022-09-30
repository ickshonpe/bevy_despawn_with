pub mod remove;
#[cfg(feature = "retain")]
pub mod retain;

use bevy::ecs::query::WorldQuery;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use lazy_static::*;
use std::marker::PhantomData;
use std::sync::Mutex;

pub use remove::*;

lazy_static! {
    static ref BUFFER: Mutex<Vec<Entity>> = Mutex::new(vec![]);
}

struct DespawnAll<F: WorldQuery>
where
    F: 'static + Sync + Send,
{
    phantom: PhantomData<F>,
}

struct DespawnAllRecursive<F: WorldQuery>
where
    F: 'static + Sync + Send,
{
    phantom: PhantomData<F>,
}

impl<F: WorldQuery> Command for DespawnAll<F>
where
    F: Sync + Send,
{
    fn write(self, world: &mut bevy::prelude::World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(world.query_filtered::<Entity, F>().iter(world));
        for entity in buffer.drain(..) {
            world.despawn(entity);
        }
    }
}

impl<F: WorldQuery> Command for DespawnAllRecursive<F>
where
    F: Sync + Send,
{
    fn write(self, world: &mut bevy::prelude::World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(world.query_filtered::<Entity, F>().iter(world));
        for entity in buffer.drain(..) {
            despawn_with_children_recursive(world, entity);
        }
    }
}

pub trait DespawnAllCommandsExt {
    fn despawn_all<F>(&mut self)
    where
        F: WorldQuery + 'static + Sync + Send;

    fn despawn_all_recursive<F>(&mut self)
    where
        F: WorldQuery + 'static + Sync + Send;
}

impl DespawnAllCommandsExt for Commands<'_, '_> {
    /// Despawn all entities that are selected by the query filter `F`.
    fn despawn_all<F>(&mut self)
    where
        F: WorldQuery + 'static + Sync + Send,
    {
        self.add(DespawnAll::<F> {
            phantom: PhantomData,
        });
    }

    /// Despawn all entities that are selected by the query filter `F` and their descendants.
    ///
    /// The descendants of entities despawned with `despawn_all_recursive`
    /// will be despawned regardless of whether they
    /// satisfy the query filter `F` or not.
    fn despawn_all_recursive<F>(&mut self)
    where
        F: WorldQuery + 'static + Sync + Send,
    {
        self.add(DespawnAllRecursive::<F> {
            phantom: PhantomData,
        });
    }
}
