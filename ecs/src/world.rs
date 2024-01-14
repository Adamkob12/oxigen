pub(crate) mod access_table;
pub(crate) mod unsafe_world_cell;

use std::any::TypeId;

use bevy_ptr::OwningPtr;
use bevy_utils::{HashMap, HashSet};

use crate::component::{Bundle, Component, ComponentDesc, ComponentId, ComponentStorage};
use crate::entity::{Entity, EntityWorldMut};
use crate::prelude::schedule::{ScheduleLabel, SCHEDULE_MAX_PLACE};
use crate::prelude::*;
use crate::query::Query;
use crate::resource::ResTable;

pub use access_table::AccessTable;
pub use unsafe_world_cell::UnsafeWorldCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Read,
    Write,
}

/// The World
pub struct World {
    pub(crate) resources: ResTable,
    pub(crate) components: HashMap<ComponentId, ComponentStorage>,
    systems: HashMap<usize, Vec<BoxedSystem>>,
    schedule_labels: Vec<usize>,
    entites: HashSet<Entity>,
}

impl World {
    /// Create a new world.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a resource based off a default value.
    pub fn init_resource<R>(&mut self)
    where
        R: Resource + Default,
    {
        self.resources.insert_resource(R::default());
    }

    /// Insert a resource based off a given value.
    pub fn insert_resource<R: Resource>(&mut self, res: R) {
        self.resources.insert_resource(res);
    }

    /// Get a [`Res`] smart pointer (shared access) to the resource.
    /// Return [`ResQueryError`] corresponding to the error, if encountered one.
    pub fn get_resource<R: Resource>(&self) -> ResQueryResult<R> {
        self.resources.get_resource::<R>()
    }

    /// Get a [`ResMut`] smart pointer (exclusive access) to the resource.
    /// Return [`ResQueryError`] corresponding to the error, if encountered one.
    pub fn get_resource_mut<R: Resource>(&self) -> ResMutQueryResult<R> {
        self.resources.get_resource_mut::<R>()
    }

    /// Get a direct exclusive reference to the resource.
    pub fn get_pure_resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.resources.get_direct_resource_mut()
    }

    /// Register a [`Component`]. If the component alredy exists, the method will not do anything.
    pub fn register_component<C: Component>(&mut self) {
        if !self.components.contains_key(&std::any::TypeId::of::<C>()) {
            self.components.insert(
                std::any::TypeId::of::<C>(),
                ComponentStorage::init(ComponentDesc::new::<C>()),
            );
        }
    }

    pub fn register_untyped_component(&mut self, comp: &dyn Component) {
        if !self.components.contains_key(&comp.comp_id()) {
            self.components.insert(
                comp.comp_id(),
                ComponentStorage::init(ComponentDesc::from_desc(
                    comp.comp_id(),
                    comp.layout(),
                    comp.name(),
                    comp.drop_fn(),
                )),
            );
        }
    }

    pub(crate) fn register_component_from_desc(
        &mut self,
        comp_id: ComponentId,
        layout: std::alloc::Layout,
        name: &'static str,
        drop_fn: Option<unsafe fn(OwningPtr<'_>)>,
    ) {
        if !self.components.contains_key(&comp_id) {
            self.components.insert(
                comp_id,
                ComponentStorage::init(ComponentDesc::from_desc(comp_id, layout, name, drop_fn)),
            );
        }
    }

    fn new_entity(&mut self) -> Entity {
        let mut entity = Entity::from_raw(rand::random::<u32>());
        while self.entites.contains(&entity) {
            entity = Entity::from_raw(rand::random::<u32>());
        }
        self.entites.insert(entity);
        entity
    }

    /// Spawn an [`Entity`] without any components.
    pub fn spawn_empty(&mut self) -> EntityWorldMut<'_> {
        let new_entity = self.new_entity();
        EntityWorldMut::from_world(self, new_entity)
    }

    /// Spawn an [`Entity`] with a bundle.
    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut<'_> {
        let new_entity = self.new_entity();
        let mut entity_world_mut = EntityWorldMut::from_world(self, new_entity);
        entity_world_mut.insert(bundle);
        entity_world_mut
    }

    /// Get the [`EntityWorldMut`] of an entity.
    pub fn entity(&mut self, entity: Entity) -> EntityWorldMut<'_> {
        assert!(
            self.entites.contains(&entity),
            "Can't get entity that doesn't exist {:?}",
            entity
        );
        EntityWorldMut::from_world(self, entity)
    }

    /// Get shared access to the component of an entity.
    pub fn get_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        // SAFETY: C is guaranteed to be the correct type for the ComponentStorage.
        unsafe {
            self.components
                .get(&ComponentId::of::<C>())?
                .get_typed(entity)
        }
    }

    /// Get exclusive access to the component of an entity.
    pub fn get_component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        // SAFETY: C is guaranteed to be the correct type for the ComponentStorage.
        unsafe {
            self.components
                .get_mut(&ComponentId::of::<C>())?
                .get_mut_typed(entity)
        }
    }

    pub fn all_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entites.iter().copied()
    }

    pub fn query<Q: WorldQuery>(&self) -> Query<Q> {
        Query::from_world(self)
    }

    /// Run the system.
    pub fn run_system<M>(&mut self, sys: impl IntoSystem<M>) {
        let mut sys = IntoSystem::into_system(sys);
        if !sys.check_conflict() {
            sys.run(self);
        } else {
            panic!("System {} has conflicting access", sys.name());
        }
    }

    /// Run a boxed system.
    pub fn run_boxed_system(&mut self, mut sys: BoxedSystem) {
        if !sys.check_conflict() {
            sys.run(self);
        } else {
            panic!("System {} has conflicting access", sys.name());
        }
    }

    /// Run a boxed system without checking for conflicts in the params.
    pub unsafe fn run_boxed_system_unchecked(&mut self, mut sys: BoxedSystem) {
        sys.run(self);
    }

    /// Run all the systems in the [`SystemSet`]
    pub fn run_systems<M>(&mut self, sys_set: impl IntoSystemSet<M>) {
        let sys_set = IntoSystemSet::into_system_set(sys_set);
        for sys in sys_set.systems() {
            self.run_boxed_system(sys);
        }
    }

    /// Run all the systems in the [`SystemSet`] without checking for conflicts in the params.
    pub unsafe fn run_systems_unchecked<M>(&mut self, sys_set: impl IntoSystemSet<M>) {
        let sys_set = IntoSystemSet::into_system_set(sys_set);
        for sys in sys_set.systems() {
            self.run_boxed_system_unchecked(sys);
        }
    }

    /// Run the system without checking for conflicts in the params.
    pub unsafe fn run_system_unchecked<M>(&mut self, sys: impl IntoSystem<M>) {
        IntoSystem::into_system(sys).run(self)
    }
}

impl World {
    pub fn config_schedule_label<L>(&mut self)
    where
        L: ScheduleLabel,
    {
        let label = L::PLACE;
        self.schedule_labels.sort();
        if self.schedule_labels.binary_search(&label).is_err() {
            self.schedule_labels.push(label);
            self.systems.insert(label, Vec::new());
            self.schedule_labels.sort();
        }
    }
    pub fn add_systems<M, L>(&mut self, sys_set: impl IntoSystemSet<M>)
    where
        L: ScheduleLabel,
    {
        let label = L::PLACE;
        self.config_schedule_label::<L>();
        self.systems
            .get_mut(&label)
            .unwrap()
            .extend(IntoSystemSet::into_system_set(sys_set).systems());
    }

    pub fn run_schedule<L>(&mut self)
    where
        L: ScheduleLabel,
    {
        let label = L::PLACE;
        self.run_schedule_value(label);
    }

    pub(crate) fn run_schedule_value(&mut self, label: usize) {
        if !self.schedule_labels.contains(&label) {
            panic!("Can't run uninitialized schedule label {}", label);
        }
        let uwc = UnsafeWorldCell::from_world(self);
        for sys in uwc.systems_from_schedule_label(label) {
            // SAFETY: Systems can't access the systems themselves (kinda).
            unsafe {
                sys.run_unsafe(uwc);
            }
        }
    }

    pub fn run_startup_labels(&mut self) {
        let startup_labels = self
            .schedule_labels
            .iter()
            .copied()
            .filter(|l| *l < SCHEDULE_MAX_PLACE)
            .collect::<Vec<_>>();

        for startup_label in startup_labels {
            self.run_schedule_value(startup_label);
        }
    }

    pub fn run<F>(&mut self, stop_condition: Option<F>)
    where
        F: Fn(&World) -> bool,
    {
        self.schedule_labels.sort();
        self.run_startup_labels();
        let update_labels = self
            .schedule_labels
            .iter()
            .copied()
            .filter(|l| *l >= SCHEDULE_MAX_PLACE)
            .collect::<Vec<_>>();

        loop {
            for update_label in &update_labels {
                self.run_schedule_value(*update_label);
            }

            if let Some(stop_condition) = &stop_condition {
                if stop_condition(self) {
                    break;
                }
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            resources: ResTable::default(),
            components: HashMap::new(),
            systems: HashMap::new(),
            schedule_labels: Vec::new(),
            entites: HashSet::new(),
        }
    }
}
