use std::marker::PhantomData;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use super::BUFFER;

#[derive(Component)]
struct Retain<C, P> 
where 
    P: Fn(&C) -> bool
{
    predicate: P,
    phantom: PhantomData<C>
}

#[derive(Component)]
struct RetainRecursive<C, P> 
where 
    P: Fn(&C) -> bool
{
    predicate: P,
    phantom: PhantomData<C>
}

#[derive(Component)]
struct RetainMut<C, P> 
where 
    P: Fn(&mut C) -> bool
{
    predicate: P,
    phantom: PhantomData<C>
}

#[derive(Component)]
struct RetainRecursiveMut<C, P> 
where 
    P: Fn(&mut C) -> bool
{
    predicate: P,
    phantom: PhantomData<C>
}

impl <C, P> Command for Retain<C, P>
where
    C: Component,
    P: Fn(&C) -> bool + Send + Sync + 'static
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        for (e, c) in world.query::<(Entity, &C)>().iter(world) {
            if !(self.predicate)(c) {
                buffer.push(e);
            }
        }
        for entity in buffer.drain(..) {
            world.despawn(entity);
        }
    }
}

impl <C, P> Command for RetainRecursive<C, P>
where
    C: Component,
    P: Fn(&C) -> bool + Send + Sync + 'static,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        for (e, c) in world.query::<(Entity, &C)>().iter(world) {
            if !(self.predicate)(c) {
                buffer.push(e);
            }
        }
        for entity in buffer.drain(..) {
            despawn_with_children_recursive(world, entity);
        }
    }
}

impl <C, P> Command for RetainMut<C, P>
where
    C: Component,
    P: Fn(&mut C) -> bool + Send + Sync + 'static
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        for (e, mut c) in world.query::<(Entity, &mut C)>().iter_mut(world) {
            if !(self.predicate)(&mut c) {
                buffer.push(e);
            }
        }
        for entity in buffer.drain(..) {
            world.despawn(entity);
        }
    }
}

impl <C, P> Command for RetainRecursiveMut<C, P>
where
    C: Component,
    P: Fn(&mut C) -> bool + Send + Sync + 'static,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        for (e, mut c) in world.query::<(Entity, &mut C)>().iter_mut(world) {
            if !(self.predicate)(&mut c) {
                buffer.push(e);
            }
        }
        for entity in buffer.drain(..) {
            despawn_with_children_recursive(world, entity);
        }
    }
}

pub trait RetainCommandsExt {
    fn retain<C, P>(&mut self, predicate: P)
    where
        C: Component,
        P: Fn(&C) -> bool + Send + Sync + 'static;

    fn retain_recursive<C, P>(&mut self, predicate: P)
    where        
        C: Component,
        P: Fn(&C) -> bool + Send + Sync + 'static;

    fn retain_mut<C, P>(&mut self, predicate: P)
    where
        C: Component,
        P: Fn(&mut C) -> bool + Send + Sync + 'static;

    fn retain_recursive_mut<C, P>(&mut self, predicate: P)
    where        
        C: Component,
        P: Fn(&mut C) -> bool + Send + Sync + 'static;
}

impl <'w, 's> RetainCommandsExt for Commands<'w, 's> {
    /// Despawn all entities with a component C that fails to satisfy
    /// the given predicate.
    fn retain<C, P>(&mut self, predicate: P)
    where
        C: Component,
        P: Fn(&C) -> bool + Send + Sync + 'static
    {
        self.add(Retain::<C, P> { 
            predicate,
            phantom: PhantomData
        });
    }

    /// Recursively despawn all entities with a component C that fails to satisfy
    /// the given predicate.
    fn retain_recursive<C, P>(&mut self, predicate: P)
    where
        C: Component,
        P: Fn(&C) -> bool + Send + Sync + 'static 
    {
        self.add(RetainRecursive::<C, P> { 
            predicate,
            phantom: PhantomData
        });
    }

    /// Despawn all entities with a component C that fails to satisfy
    /// the given predicate.
    fn retain_mut<C, P>(&mut self, predicate: P)
    where
        C: Component,
        P: Fn(&mut C) -> bool + Send + Sync + 'static
    {
        self.add(RetainMut::<C, P> { 
            predicate,
            phantom: PhantomData
        });
    }

    /// Recursively despawn all entities with a component C that fails to satisfy
    /// the given predicate.
    fn retain_recursive_mut<C, P>(&mut self, predicate: P)
    where
        C: Component,
        P: Fn(&mut C) -> bool + Send + Sync + 'static 
    {
        self.add(RetainRecursiveMut::<C, P> { 
            predicate,
            phantom: PhantomData
        });
    }
}