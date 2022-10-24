use crate::DespawnBuffer;
use bevy::ecs::query::Fetch;
use bevy::ecs::query::WorldQuery;
use bevy::ecs::query::WorldQueryGats;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use std::marker::PhantomData;

struct Retain<P, Q, F = ()>
where
    P: 'static + Sync + Send,
    P: for<'w> FnMut(<<Q as WorldQueryGats<'w>>::Fetch as Fetch<'w>>::Item) -> bool,
    Q: WorldQuery + 'static + Sync + Send,
    F: WorldQuery + 'static + Sync + Send,
{
    predicate: P,
    phantom: PhantomData<(F, Q)>,
}

impl<P, Q, F> Command for Retain<P, Q, F>
where
    P: 'static + Sync + Send,
    P: for<'w> FnMut(<<Q as WorldQueryGats<'w>>::Fetch as Fetch<'w>>::Item) -> bool,
    Q: WorldQuery + 'static + Sync + Send,
    F: WorldQuery + 'static + Sync + Send,
{
    fn write(mut self, world: &mut World) {
        if !world.contains_resource::<DespawnBuffer>() {
            world.insert_resource(DespawnBuffer::default());
        }
        world.resource_scope(|world, mut buffer: Mut<DespawnBuffer>| {
            for (e, c) in world.query_filtered::<(Entity, Q), F>().iter_mut(world) {
                if !(self.predicate)(c) {
                    buffer.push(e);
                }
            }
            for entity in buffer.drain(..) {
                world.despawn(entity);
            }
        });
    }
}

struct RetainRecursive<P, Q, F = ()>
where
    P: 'static + Sync + Send,
    P: FnMut(<<Q as WorldQueryGats<'_>>::Fetch as Fetch<'_>>::Item) -> bool,
    Q: WorldQuery + 'static + Sync + Send,
    F: WorldQuery + 'static + Sync + Send,
{
    predicate: P,
    phantom: PhantomData<(F, Q)>,
}

impl<P, Q, F> Command for RetainRecursive<P, Q, F>
where
    P: 'static + Sync + Send,
    P: FnMut(<<Q as WorldQueryGats<'_>>::Fetch as Fetch<'_>>::Item) -> bool,
    Q: WorldQuery + 'static + Sync + Send,
    F: WorldQuery + 'static + Sync + Send,
{
    fn write(mut self, world: &mut World) {
        if !world.contains_resource::<DespawnBuffer>() {
            world.insert_resource(DespawnBuffer::default());
        }
        world.resource_scope(|world, mut buffer: Mut<DespawnBuffer>| {
            for (e, c) in world.query_filtered::<(Entity, Q), F>().iter_mut(world) {
                if !(self.predicate)(c) {
                    buffer.push(e);
                }
            }
            for entity in buffer.drain(..) {
                despawn_with_children_recursive(world, entity);
            }
        });
    }
}

pub trait RetainCommandsExt<P> {
    fn retain<Q, F>(&mut self, predicate: P)
    where
        P: 'static + Sync + Send,
        P: for<'w> FnMut(<<Q as WorldQueryGats<'w>>::Fetch as Fetch<'w>>::Item) -> bool,
        Q: WorldQuery + 'static + Sync + Send,
        F: WorldQuery + 'static + Sync + Send;

    fn retain_recursive<Q, F>(&mut self, predicate: P)
    where
        P: 'static + Sync + Send,
        P: FnMut(<<Q as WorldQueryGats<'_>>::Fetch as Fetch<'_>>::Item) -> bool,
        Q: WorldQuery + 'static + Sync + Send,
        F: WorldQuery + 'static + Sync + Send;
}

impl<P> RetainCommandsExt<P> for Commands<'_, '_> {
    /// Queries the World with a component accessor `Q` and filter `F`,
    /// Calls P with each query result, P may mutate the components.
    /// If P returns false, the respective entity is despawned.
    ///
    /// Remember that Commands are not applied immediately, nothing will be done until the next stage boundary.
    fn retain<Q, F>(&mut self, predicate: P)
    where
        P: 'static + Sync + Send,
        P: for<'w> FnMut(<<Q as WorldQueryGats<'w>>::Fetch as Fetch<'w>>::Item) -> bool,
        Q: WorldQuery + 'static + Sync + Send,
        F: WorldQuery + 'static + Sync + Send,
    {
        self.add(Retain::<P, Q, F> {
            predicate,
            phantom: PhantomData,
        });
    }

    /// Queries the World with a component accessor `Q` and filter `F`,
    /// Calls P with each query result, P may mutate the components.
    /// If P returns false, the respective entity and all its descandents are despawned.
    ///
    /// Remember that Commands are not applied immediately, nothing will be done until the next stage boundary.
    fn retain_recursive<Q, F>(&mut self, predicate: P)
    where
        P: 'static + Sync + Send,
        P: FnMut(<<Q as WorldQueryGats<'_>>::Fetch as Fetch<'_>>::Item) -> bool,
        Q: WorldQuery + 'static + Sync + Send,
        F: WorldQuery + 'static + Sync + Send,
    {
        self.add(RetainRecursive::<P, Q, F> {
            predicate,
            phantom: PhantomData,
        });
    }
}
