use std::marker::PhantomData;

use crate::{
    component::ComponentStorage,
    prelude::{schedule::ScheduleLabel, *},
};

#[derive(Clone, Copy)]
pub struct UnsafeWorldCell<'w> {
    world_ptr: *mut World,
    _marker: PhantomData<(&'w World,)>,
}

impl<'w> UnsafeWorldCell<'w> {
    pub(crate) fn from_world(world: &'w World) -> Self {
        Self {
            world_ptr: world as *const World as *mut World,
            _marker: PhantomData,
        }
    }

    /// # Safety:
    /// The caller must ensure that the new shared reference doesn't conflict with any other exclusive references.
    pub unsafe fn world(&self) -> &'w World {
        unsafe { &*self.world_ptr }
    }

    /// # Safety:
    /// The caller must ensure that the new exclusive reference doesn't conflict with any other references.
    pub unsafe fn world_mut(&self) -> &'w mut World {
        unsafe { &mut *self.world_ptr }
    }

    pub fn all_entities(self) -> Vec<Entity> {
        unsafe { self.world() }.all_entities().collect()
    }

    pub fn get_component<C: Component>(self, entity: Entity) -> Option<&'w C> {
        unsafe { self.world() }.get_component::<C>(entity)
    }

    pub fn get_component_mut<C: Component>(self, entity: Entity) -> Option<&'w mut C> {
        unsafe { self.world_mut() }.get_component_mut::<C>(entity)
    }

    pub fn component_storage(self, comp_id: ComponentId) -> Option<&'w ComponentStorage> {
        unsafe { self.world() }.components.get(&comp_id)
    }

    pub fn get_resource<R: Resource>(self) -> ResQueryResult<'w, R> {
        unsafe { self.world() }.get_resource::<R>()
    }

    pub fn get_resource_mut<R: Resource>(self) -> ResMutQueryResult<'w, R> {
        unsafe { self.world_mut() }.get_resource_mut::<R>()
    }

    pub fn query<Q: WorldQuery>(self) -> Query<'w, Q> {
        Query::from_world(unsafe { self.world() })
    }

    pub fn systems_from_schedule<L: ScheduleLabel>(self) -> &'w mut Vec<BoxedSystem> {
        self.systems_from_schedule_label(L::PLACE)
    }

    pub fn systems_from_schedule_label(self, label: usize) -> &'w mut Vec<BoxedSystem> {
        unsafe { self.world_mut() }.systems.get_mut(&label).unwrap()
    }
}
