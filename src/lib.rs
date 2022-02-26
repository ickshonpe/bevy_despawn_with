use std::marker::PhantomData;
use bevy::ecs::system::Command;
use bevy::prelude::*;

#[derive(Component)]
pub struct DespawnWith<C> {
    phantom_component: PhantomData<C>
}

pub struct DespawnRecursiveWith<C>
where
    C: Component
{
    phantom_component: PhantomData<C>
}

pub struct Retain<C, P> 
where 
    C: Component,
    P: Fn(&C) -> bool
{
    predicate: P,
    phantom: PhantomData<C>
}

pub struct RetainRecursive<C, P> 
where 
    C: Component,
    P: Fn(&C) -> bool
{
    predicate: P,
    phantom: PhantomData<C>
}



impl <C> Command for DespawnWith<C> 
where
    C: Component,
{
    fn write(self, world: &mut World) {
       let queue: Vec<Entity> = world.query::<(Entity, &C)>().iter(world).map(|(e, _)| e).collect();
        for entity in queue.into_iter() {
            world.despawn(entity);
        }
    }
}

impl <C> Command for DespawnRecursiveWith<C> 
where
    C: Component,
{
    fn write(self, world: &mut World) {
        let queue: Vec<Entity> = world.query::<(Entity, &C)>().iter(world).map(|(e, _)| e).collect();
        for entity in queue.into_iter() {
            despawn_with_children_recursive(world, entity);
        }
    }
}

impl <C, P> Command for Retain<C, P>
where
    C: Component,
    P: Fn(&C) -> bool + Component,
{
    fn write(self, world: &mut World) {
        let queue: Vec<Entity> = 
            world.query::<(Entity, &C)>().iter(world)
            .filter(|(_, c)| !(self.predicate)(c))
            .map(|(e, _)| e).collect();
        for entity in queue.into_iter() {
            world.despawn(entity);
        }
    }
}

impl <C, P> Command for RetainRecursive<C, P>
where
    C: Component,
    P: Fn(&C) -> bool + Component,
{
    fn write(self, world: &mut World) {
        let queue: Vec<Entity> =
            world.query::<(Entity, &C)>().iter(world)
            .filter(|(_, c)| !(self.predicate)(c))
            .map(|(e, _)| e).collect();
        for entity in queue.into_iter() {
            despawn_with_children_recursive(world, entity);
        }
    }
}

pub trait DespawnWithExt {
    fn despawn_with<C>(&mut self) where C: Component;
    fn despawn_recursive_with<C>(&mut self) where C: Component;
    fn retain<C, P>(&mut self, predicate: P)
    where
        C: Component,
        P: Fn(&C) -> bool + Component;
    fn retain_recursive<C, P>(&mut self, predicate: P)
    where
        C: Component,
        P: Fn(&C) -> bool + Component;
}

impl DespawnWithExt for Commands<'_, '_> {  
    /// Despawn all entities with component C.
    fn despawn_with<C: Component>(&mut self) {
        self.add(DespawnWith::<C> { phantom_component: PhantomData } );
    }

    /// Despawn all entities with component C, and despawn all their descendants regardless of whether they have C.
    fn despawn_recursive_with<C>(&mut self)
    where
        C: Component
    {
        self.add(DespawnRecursiveWith::<C> { phantom_component: PhantomData } );
    }

    /// Despawn all entities with a component C that fails to satisfy
    /// the given predicate.
    fn retain<C, P>(&mut self, predicate: P)
    where
        C: Component,
        P: Fn(&C) -> bool + Component 
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
        P: Fn(&C) -> bool + Component 
    {
        self.add(Retain::<C, P> { 
            predicate,
            phantom: PhantomData
        });
    }
}