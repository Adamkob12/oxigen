use super::{downcast_res_mut, get_res_id, res_id, Resource, ResourceId};
use super::{Res, ResMut};
use hashbrown::hash_map::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

/// Types of errors that can arrise when trying to get a resource from the resource table.
#[derive(Debug)]
pub enum ResQueryError {
    ResDoesntExist,
    Poisoned,
}

/// The result of querying the resource table for an [`shared access smart pointer to a resource`](`Res`)
pub type ResQueryResult<'r, R> = Result<Res<'r, R>, ResQueryError>;

/// The result of querying the resource table for an [`exclusive access smart pointer to a resource`](`ResMut`)
pub type ResMutQueryResult<'r, R> = Result<ResMut<'r, R>, ResQueryError>;

/// This table maps a [`ResourceId`] to a refrence counted smart pointer to it.
pub(crate) struct ResTable {
    map: HashMap<ResourceId, Arc<RwLock<dyn Resource>>>,
    err_when_poisoned: bool,
}

impl Default for ResTable {
    fn default() -> Self {
        Self {
            map: HashMap::with_capacity(20),
            err_when_poisoned: false,
        }
    }
}

impl ResTable {
    /// Insert a [`Resource`] into the global resource table.
    pub(crate) fn insert_resource<R: Resource>(&mut self, res: R) {
        self.map
            .insert(get_res_id(&res), Arc::new(RwLock::new(res)));
    }

    /// Get a [`Res`] smart pointer (shared accesss) to a [`Resource`].
    pub(crate) fn get_resource<R: Resource>(&self) -> ResQueryResult<R> {
        self.get_res_entry::<R>().map(|res| Res {
            lock: res.read().map_or_else(|e| e.into_inner(), |guard| guard),
            _marker: PhantomData,
        })
    }

    /// Get a [`ResMut`] smart pointer (exclusive access) to a [`Resource`].
    pub(crate) fn get_resource_mut<R: Resource>(&self) -> ResMutQueryResult<R> {
        self.get_res_entry::<R>().map(|res| ResMut {
            lock: res.write().map_or_else(|e| e.into_inner(), |guard| guard),
            _marker: PhantomData,
        })
    }

    pub(crate) fn get_direct_resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        Some(
            downcast_res_mut(
                Arc::get_mut(self.get_res_entry_mut::<R>().ok()?)?
                    .get_mut()
                    .ok()?,
            )
            .unwrap(),
        )
    }

    pub(crate) fn _direct_resource_mut<R: Resource>(&mut self) -> &mut R {
        self.get_direct_resource_mut().unwrap()
    }

    fn get_res_entry<R: Resource>(&self) -> Result<&Arc<RwLock<dyn Resource>>, ResQueryError> {
        let res_id = res_id::<R>();
        if let Some(res) = self.map.get(&res_id) {
            if res.is_poisoned() && self.err_when_poisoned {
                return Result::Err(ResQueryError::Poisoned);
            }
            Ok(res)
        } else {
            Result::Err(ResQueryError::ResDoesntExist)
        }
    }

    fn get_res_entry_mut<R: Resource>(
        &mut self,
    ) -> Result<&mut Arc<RwLock<dyn Resource>>, ResQueryError> {
        let res_id = res_id::<R>();
        if let Some(res) = self.map.get_mut(&res_id) {
            if res.is_poisoned() && self.err_when_poisoned {
                return Result::Err(ResQueryError::Poisoned);
            }
            Ok(res)
        } else {
            Result::Err(ResQueryError::ResDoesntExist)
        }
    }
}
