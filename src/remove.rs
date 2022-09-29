use bevy::ecs::query::WorldQuery;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use std::marker::PhantomData;

use crate::BUFFER;

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
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(world.query_filtered::<Entity, (With<C>, F)>().iter(world));
        for entity in buffer.drain(..) {
            world.entity_mut(entity).remove::<C>();
        }
    }
}

pub trait RemoveAllCommandsExt {
    fn remove_all<C>(&mut self)
    where
        C: Component;

    fn remove_all_filtered<C, F>(&mut self)
    where
        C: Component,
        F: WorldQuery + 'static + Sync + Send;
}

impl RemoveAllCommandsExt for Commands<'_, '_> {
    fn remove_all<C>(&mut self)
    where
        C: Component,
    {
        self.add(RemoveAll::<C, ()> {
            phantom: PhantomData,
        });
    }

    fn remove_all_filtered<C, F>(&mut self)
    where
        C: Component,
        F: WorldQuery + 'static + Sync + Send,
    {
        self.add(RemoveAll::<C, F> {
            phantom: PhantomData,
        });
    }
}
