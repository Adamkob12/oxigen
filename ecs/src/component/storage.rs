use crate::{entity::Entity, utils::BlobVec};
use bevy_ptr::{OwningPtr, Ptr, PtrMut};
use hashbrown::HashMap;

use super::{Component, ComponentDesc};

pub struct ComponentStorage {
    /// The pure data that holds the components.
    data: BlobVec,
    /// The map from entity to the row index of the component.
    entity_to_row: HashMap<Entity, usize>,
}

impl ComponentStorage {
    pub fn init(cdesc: ComponentDesc) -> Self {
        Self {
            // SAFETY: cdesc.drop() is valid for the types that will be inserted.
            data: unsafe { BlobVec::new(cdesc.layout, cdesc.drop, 1) },
            entity_to_row: HashMap::with_capacity(1),
        }
    }

    /// Get an iterator over the entities that have components stored in this [`ComponentStorage`].
    pub(crate) fn entites(&self) -> impl Iterator<Item = &Entity> {
        self.entity_to_row.keys()
    }

    /// # SAFETY:
    /// `value` needs to have the same [`Layout`](std::alloc::Layout) as the components
    /// stored here. Or, simply, the type of `val` needs to be the same as the type of components
    /// stored in this [`ComponentStorage`].
    ///
    /// If the [`Entity`] already had a registered component, override it.
    pub fn insert(&mut self, entity: Entity, value: OwningPtr<'_>) {
        if let Some(&index) = self.entity_to_row.get(&entity) {
            dbg!(index);
            unsafe { self.data.replace_unchecked(index, value) }
        } else {
            unsafe { self.data.push(value) }
            self.entity_to_row.insert(entity, self.data.len() - 1);
        }
    }

    /// Get (type erased) shared access to the component for the given [`Entity`]
    pub fn get(&self, entity: Entity) -> Option<Ptr<'_>> {
        let index = *self.entity_to_row.get(&entity)?;

        // Make sure the index is not greater than the length of the data.
        (index < self.data.len()).then(|| ())?;

        // SAFETY: We checked that the index isn't greater then the length of the data.
        unsafe { Some(self.data.get_unchecked(index)) }
    }

    /// Get (type erased) exclusive access to the component for the given [`Entity`]
    pub fn get_mut(&mut self, entity: Entity) -> Option<PtrMut<'_>> {
        let index = *self.entity_to_row.get(&entity)?;

        // Make sure the index is not greater than the length of the data.
        (index < self.data.len()).then(|| ())?;

        // SAFETY: We checked that the index isn't greater then the length of the data.
        unsafe { Some(self.data.get_unchecked_mut(index)) }
    }

    /// Get exclusive access to the component for the given [`Entity`]
    /// # SAFETY:
    /// `C` must be the same type as the components stored in this [`ComponentStorage`].
    pub unsafe fn get_mut_typed<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        self.get_mut(entity).map(|p| p.deref_mut::<C>())
    }

    /// Get shared access to the component for the given [`Entity`]
    /// # SAFETY:
    /// `C` must be the same type as the components stored in this [`ComponentStorage`].
    pub unsafe fn get_typed<C: Component>(&self, entity: Entity) -> Option<&C> {
        self.get(entity).map(|p| p.deref::<C>())
    }
}
