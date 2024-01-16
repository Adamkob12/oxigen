use std::any::TypeId;

use bevy_ptr::OwningPtr;
use bevy_utils::{HashMap, HashSet};

use crate::component::{Bundle, Component, ComponentDesc, ComponentId, ComponentStorage};
use crate::entity::{Entity, EntityWorldMut};
use crate::prelude::*;
use crate::query::Query;
use crate::resource::ResTable;

pub struct AccessTable {
    table: HashMap<TypeId, Access>,
    conflict: bool,
}

impl std::ops::Deref for AccessTable {
    type Target = HashMap<TypeId, Access>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl std::ops::DerefMut for AccessTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}


impl AccessTable {
    pub fn is_conflicted(&self) -> bool {
        self.conflict
    }

    pub fn new() -> Self {
        AccessTable {
            table: HashMap::new(),
            conflict: false,
        }
    }

    pub fn merge(&mut self, other: Self) {
        if other.conflict {
            self.conflict = true
        }

        for (k, v) in other.table.into_iter() {
            if let Some(prev) = self.insert(k, v) {
                match (prev, v) {
                    (Access::Read, Access::Read) => continue,
                    _ => self.conflict = true,
                }
            }
        }
    }

    pub fn single(type_id: TypeId, access: Access) -> Self {
        let mut access_table = Self::new();
        access_table.insert(type_id, access);
        access_table
    }

    pub fn extend() {}
}
