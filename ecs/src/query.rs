use std::any::TypeId;

use bevy_utils::all_tuples;

use crate::prelude::*;
use crate::{
    component::{comp_id, Component},
    prelude::{unsafe_world_cell::UnsafeWorldCell, AccessTable, Entity, World},
};

#[derive(Debug)]
pub enum QueryError {
    EntityNotInQuery,
    ErrorFetchingData,
}

pub type QueryResult<T> = Result<T, QueryError>;

/// Represents data a query can fetch from the [`World`].
pub trait WorldQuery {
    type Data<'a>;

    fn get_data_from_world<'w>(
        world: UnsafeWorldCell<'w>,
        entity: Entity,
    ) -> QueryResult<Self::Data<'w>>;

    fn get_entites_matching_archetype<'w>(world: UnsafeWorldCell<'w>) -> Vec<Entity>;

    fn access_table() -> AccessTable;
}

pub struct Query<'w, Q: WorldQuery> {
    entites: Vec<Entity>,
    world: UnsafeWorldCell<'w>,
    _marker: std::marker::PhantomData<Q>,
}

impl<'w, Q: WorldQuery> Query<'w, Q> {
    pub fn get(&self, entity: Entity) -> QueryResult<Q::Data<'_>> {
        Q::get_data_from_world(self.world, entity)
    }
}

impl<'w, Q: WorldQuery> Query<'w, Q> {
    pub(crate) fn from_world(world: &'w World) -> Self {
        let world = UnsafeWorldCell::from_world(world);
        Self {
            entites: Q::get_entites_matching_archetype(world),
            world,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'w, Q: WorldQuery> IntoIterator for Query<'w, Q> {
    type Item = Q::Data<'w>;
    type IntoIter = QueryIter<'w, Q>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIter {
            entites: self.entites,
            current_entity: 0,
            world: self.world,
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct QueryIter<'w, Q: WorldQuery> {
    entites: Vec<Entity>,
    current_entity: usize,
    world: UnsafeWorldCell<'w>,
    _marker: std::marker::PhantomData<Q>,
}

impl<'w, Q: WorldQuery> ExactSizeIterator for QueryIter<'w, Q> {
    fn len(&self) -> usize {
        self.entites.len()
    }
}

impl<'w, Q: WorldQuery> Iterator for QueryIter<'w, Q> {
    type Item = Q::Data<'w>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entity >= self.entites.len() {
            return None;
        }
        let entity = self.entites[self.current_entity];
        self.current_entity += 1;
        Some(Q::get_data_from_world(self.world, entity).unwrap())
    }
}

impl WorldQuery for Entity {
    type Data<'a> = Entity;

    fn get_data_from_world<'w>(
        _world: UnsafeWorldCell<'w>,
        entity: Entity,
    ) -> QueryResult<Self::Data<'w>> {
        Ok(entity)
    }

    fn get_entites_matching_archetype<'w>(world: UnsafeWorldCell<'w>) -> Vec<Entity> {
        world.all_entities()
    }

    fn access_table() -> AccessTable {
        AccessTable::new()
    }
}

impl<C: Component> WorldQuery for &C {
    type Data<'w> = &'w C;

    fn get_data_from_world<'w>(
        world: UnsafeWorldCell<'w>,
        entity: Entity,
    ) -> QueryResult<Self::Data<'w>> {
        world
            .get_component::<C>(entity)
            .ok_or(QueryError::ErrorFetchingData)
    }

    fn get_entites_matching_archetype(world: UnsafeWorldCell<'_>) -> Vec<Entity> {
        world
            .component_storage(comp_id::<C>())
            .map_or(Vec::new(), |storage| storage.entites().copied().collect())
    }

    fn access_table() -> AccessTable {
        AccessTable::single(comp_id::<C>(), Access::Read)
    }
}

impl<C: Component> WorldQuery for &mut C {
    type Data<'w> = &'w mut C;

    fn get_data_from_world<'w>(
        world: UnsafeWorldCell<'w>,
        entity: Entity,
    ) -> QueryResult<Self::Data<'_>> {
        world
            .get_component_mut::<C>(entity)
            .ok_or(QueryError::ErrorFetchingData)
    }

    fn get_entites_matching_archetype(world: UnsafeWorldCell<'_>) -> Vec<Entity> {
        world
            .component_storage(comp_id::<C>())
            .map_or(Vec::new(), |storage| storage.entites().copied().collect())
    }

    fn access_table() -> AccessTable {
        AccessTable::single(comp_id::<C>(), Access::Write)
    }
}

macro_rules! impl_query_data_for_tuple {
    ($($name:ident),*) => {
        impl <$( $name: WorldQuery ),*> WorldQuery for ($( $name, )*) {
            type Data<'w> = ($( $name::Data<'w>, )*);

            #[allow(unused_variables)]
            fn get_data_from_world<'w>(world: UnsafeWorldCell<'w>, entity: Entity) -> QueryResult<Self::Data<'_>> {
                Ok(($( $name::get_data_from_world(world, entity)?, )*))
            }

            #[allow(unused_mut, non_snake_case)]
            fn get_entites_matching_archetype(world: UnsafeWorldCell<'_>) -> Vec<Entity> {
                let mut entites = world.all_entities();
                $(
                    let mut $name = $name::get_entites_matching_archetype(world);
                    $name.sort();
                )*
                $(
                    entites.retain(|e| $name.binary_search(e).is_ok());
                )*
                entites
            }

            #[allow(unused_mut)]
            fn access_table() -> AccessTable {
                let mut access_table = AccessTable::new();
                $(
                    access_table.merge($name::access_table());
                )*
                access_table
            }
        }
    };
}

all_tuples!(impl_query_data_for_tuple, 0, 15, Q);
