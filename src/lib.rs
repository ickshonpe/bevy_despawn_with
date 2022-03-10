pub mod retain;

use std::marker::PhantomData;
use std::sync::Mutex;
use bevy::ecs::system::Command;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
pub use retain::RetainCommandsExt;
use lazy_static::*;

lazy_static! {
    static ref BUFFER: Mutex<Vec<Entity>> = Mutex::new(vec![]);
}
pub struct DespawnWith<C> 
{
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
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(SystemState::<Query<Entity, With<C>>>::new(world).get(world).iter());
        for entity in buffer.drain(..) {
            world.despawn(entity);
        }
    }
}

pub struct DespawnWithAll<B> 
{
    phantom_component: PhantomData<B>
}

impl <A, B> Command for DespawnWithAll<(A, B)> 
where
    A: Component,
    B: Component,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(SystemState::<Query<Entity, (With<A>, With<B>)>>::new(world).get(world).iter());
        for entity in buffer.drain(..) {
            world.despawn(entity);
        }
    }
}

impl <A, B, C> Command for DespawnWithAll<(A, B, C)> 
where
    A: Component,
    B: Component,
    C: Component,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(SystemState::<Query<Entity, (With<A>, With<B>, With<C>)>>::new(world).get(world).iter());
        for entity in buffer.drain(..) {
            world.despawn(entity);
        }
    }
}

impl <A, B, C, D> Command for DespawnWithAll<(A, B, C, D)> 
where
    A: Component,
    B: Component,
    C: Component,
    D: Component,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(SystemState::<Query<Entity, (With<A>, With<B>, With<C>, With<D>)>>::new(world).get(world).iter());
        for entity in buffer.drain(..) {
            world.despawn(entity);
        }
    }
}

impl <A, B, C, D, E> Command for DespawnWithAll<(A, B, C, D, E)> 
where
    A: Component,
    B: Component,
    C: Component,
    D: Component,
    E: Component,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(SystemState::<Query<Entity, (With<A>, With<B>, With<C>, With<D>, With<E>)>>::new(world).get(world).iter());
        for entity in buffer.drain(..) {
            world.despawn(entity);
        }
    }
}


impl <C> Command for DespawnRecursiveWith<C> 
where
    C: Component,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(SystemState::<Query<Entity, With<C>>>::new(world).get(world).iter());
        for entity in buffer.drain(..) {
            despawn_with_children_recursive(world, entity);
        }
    }
}

pub struct DespawnWithAllRecursive<B> 
{
    phantom_component: PhantomData<B>
}

impl <A, B> Command for DespawnWithAllRecursive<(A, B)> 
where
    A: Component,
    B: Component,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(SystemState::<Query<Entity, (With<A>, With<B>)>>::new(world).get(world).iter());
        for entity in buffer.drain(..) {
            despawn_with_children_recursive(world, entity);
        }
    }
}

impl <A, B, C> Command for DespawnWithAllRecursive<(A, B, C)> 
where
    A: Component,
    B: Component,
    C: Component,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(SystemState::<Query<Entity, (With<A>, With<B>, With<C>)>>::new(world).get(world).iter());
        for entity in buffer.drain(..) {
            despawn_with_children_recursive(world, entity);
        }
    }
}

impl <A, B, C, D> Command for DespawnWithAllRecursive<(A, B, C, D)> 
where
    A: Component,
    B: Component,
    C: Component,
    D: Component,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(SystemState::<Query<Entity, (With<A>, With<B>, With<C>, With<D>)>>::new(world).get(world).iter());
        for entity in buffer.drain(..) {
            despawn_with_children_recursive(world, entity);
        }
    }
}


impl <A, B, C, D, E> Command for DespawnWithAllRecursive<(A, B, C, D, E)> 
where
    A: Component,
    B: Component,
    C: Component,
    D: Component,
    E: Component,
{
    fn write(self, world: &mut World) {
        let mut buffer = BUFFER.lock().unwrap();
        buffer.extend(SystemState::<Query<Entity, (With<A>, With<B>, With<C>, With<D>, With<E>)>>::new(world).get(world).iter());
        for entity in buffer.drain(..) {
            despawn_with_children_recursive(world, entity);
        }
    }
}

pub trait DespawnWithCommandsExt {
    fn despawn_with<C>(&mut self) where C: Component;
    fn despawn_with_all<X>(&mut self)
    where 
        DespawnWithAll<X>: Command;
    fn despawn_recursive_with<C>(&mut self) where C: Component;
    fn despawn_with_all_recursive<X>(&mut self)
    where 
        DespawnWithAllRecursive<X>: Command;
}

impl DespawnWithCommandsExt for Commands<'_, '_> {
    /// Despawn all entities with component C. 
    fn despawn_with<C: Component>(&mut self) {
        self.add(DespawnWith::<C> { phantom_component: PhantomData } );
    }

    /// Despawn all entities with component C.
    fn despawn_with_all<X>(&mut self)
    where 
        DespawnWithAll<X>: Command {
        self.add(DespawnWithAll::<X> { phantom_component: PhantomData } );
    }

    /// Despawn all entities with component C, and despawn all their descendants regardless of whether they have C.
    fn despawn_recursive_with<C>(&mut self)
    where
        C: Component
    {
        self.add(DespawnRecursiveWith::<C> { phantom_component: PhantomData } );
    }

    /// Despawn all entities with component C.
    fn despawn_with_all_recursive<X>(&mut self)
    where 
        DespawnWithAllRecursive<X>: Command {
        self.add(DespawnWithAllRecursive::<X> { phantom_component: PhantomData } );
    }
}
