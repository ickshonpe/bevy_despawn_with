use std::marker::PhantomData;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use super::BUFFER;

#[derive(Component)]
pub struct Retain<C, P> 
where 
    P: Fn(&C) -> bool
{
    predicate: P,
    phantom: PhantomData<C>
}

#[derive(Component)]
pub struct RetainRecursive<C, P> 
where 
    P: Fn(&C) -> bool
{
    predicate: P,
    phantom: PhantomData<C>
}

#[derive(Component)]
pub struct RetainMut<C, P> 
where 
    P: Fn(&mut C) -> bool
{
    predicate: P,
    phantom: PhantomData<C>
}

#[derive(Component)]
pub struct RetainRecursiveMut<C, P> 
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
            if !(self.predicate)(&c) {
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
            if !(self.predicate)(&c) {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component)]
    struct DespawnTimer(f32);

    fn setup(mut commands: Commands) {
        for i in 0..10 {
            commands.spawn().insert(DespawnTimer(i as f32));
        }
    }

    fn update_timer(
        mut query: Query<&mut DespawnTimer>,
    ) {
        query.for_each_mut(|mut dt| {
            dt.0 -= 4.5;
        });
    }

    fn despawn_timer(
        mut commands: Commands,
    ) {
        commands.retain(|dt: &DespawnTimer| { 
            0. < dt.0
        });
    }

    fn despawn_recursive_timer(
        mut commands: Commands,
    ) {
        commands.retain_recursive(|dt: &DespawnTimer| { 
            0. < dt.0
        });
    }

    fn despawn_timer_mut(
        mut commands: Commands,
    ) {
        let delta = -4.5;
        commands.retain_mut(move |dt: &mut DespawnTimer| { 
            dt.0 -= delta;
            0. < dt.0
        });
    }

    fn despawn_timer_rec_mut(
        mut commands: Commands,        
    ) {
        let delta = -4.5;
        commands.retain_recursive_mut(move |dt: &mut DespawnTimer| { 
            dt.0 -= delta;
            0. < dt.0
        });
    }

    fn despawn_timer_system(
        mut commands: Commands,        
        time: Res<Time>,
    ) {
        let delta = time.delta_seconds();
        commands.retain_recursive_mut(move |dt: &mut DespawnTimer| { 
            dt.0 -= delta;
            0. < dt.0
        });
    }
}