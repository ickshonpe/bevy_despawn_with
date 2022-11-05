use bevy::ecs::query::WorldQuery;
use bevy::ecs::system::Command;
use bevy::ecs::system::Despawn;
use bevy::prelude::*;
use std::marker::PhantomData;

use crate::DespawnBuffer;

struct RemoveAll<C, F = ()>
where
    C: Component,
    F: WorldQuery + 'static + Sync + Send,
{
    phantom: PhantomData<(C, F)>,
}

impl<C, F> Command for RemoveAll<C, F>
where
    C: Component,
    F: WorldQuery + 'static + Sync + Send,
{
    fn write(self, world: &mut bevy::prelude::World) {
        world.init_resource::<DespawnBuffer>();
        world.resource_scope(|world, mut buffer: Mut<DespawnBuffer>| {
            buffer.extend(world.query_filtered::<Entity, (With<C>, F)>().iter(world));
            for entity in buffer.drain(..) {
                world.entity_mut(entity).remove::<C>();
            }
        });
    }
}

pub trait RemoveAllCommandsExt {
    fn remove_all<C, F>(&mut self)
    where
        C: Component,
        F: WorldQuery + 'static + Sync + Send;
}

impl RemoveAllCommandsExt for Commands<'_, '_> {
    /// For every entity that has a component of type `MyComponent`
    /// and passes Filter `F`,
    /// remove its `MyComponent`.
    fn remove_all<C, F>(&mut self)
    where
        C: Component,
        F: WorldQuery + 'static + Sync + Send,
    {
        self.add(RemoveAll::<C, F> {
            phantom: PhantomData,
        });
    }
}
