use crate::{component::Bundle, prelude::World};
use bevy_ptr::OwningPtr;
use std::ptr::NonNull;

/// This struct is used to identify each entity.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity {
    id: u32,
}

impl Entity {
    pub(crate) fn from_raw(raw: u32) -> Self {
        Self { id: raw }
    }
}

pub struct EntityWorldMut<'w> {
    world: &'w mut World,
    entity: Entity,
}

impl<'w> EntityWorldMut<'w> {
    pub fn from_world(world: &'w mut World, entity: Entity) -> Self {
        Self { world, entity }
    }

    pub fn id(&self) -> Entity {
        self.entity
    }

    /// insert a [`Bundle`] to the [`Entity`], if the entity already had a component in the bundle, replace it.
    pub fn insert<B: Bundle>(&mut self, bundle: B) -> &mut Self {
        for boxed_component in bundle.components() {
            self.world.register_component_from_desc(
                boxed_component.comp_id(),
                boxed_component.layout(),
                boxed_component.name(),
                boxed_component.drop_fn(),
            );
            let comp_id = boxed_component.comp_id();
            // Convert the box into a type-less pointer.
            let raw_pointer_to_component_on_the_heap = Box::into_raw(boxed_component) as *mut u8;
            // Create a new NonNull pointer.
            // SAFETY: We just defined the pointer
            let non_null_pointer_to_component_on_the_heap =
                NonNull::new(raw_pointer_to_component_on_the_heap).unwrap();
            // SAFETY: The components types are guarenteed to match.
            unsafe {
                self.world.components.get_mut(&comp_id).unwrap().insert(
                    self.entity,
                    OwningPtr::new(non_null_pointer_to_component_on_the_heap),
                );
            }
        }

        self
    }

    pub fn world_mut(self) -> &'w mut World {
        self.world
    }
}
