//! The implementation sucks as it allocates a Vec every time.
//! Could use a global static buffer or a resource?
//! Probably some built in way around it I don't know of.
//! Will immediately update the crate once I work it out.


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

pub trait DespawnWithExt {
    fn despawn_with<C>(&mut self) where C: Component;
    fn despawn_recursive_with<C>(&mut self) where C: Component;
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
}