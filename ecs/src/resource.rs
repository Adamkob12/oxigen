mod impls;
mod table;

pub(crate) use table::ResTable;

use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

/// Smart pointer for accessing a resource in a shared manner.
pub struct Res<'r, R: Resource> {
    lock: RwLockReadGuard<'r, dyn Resource>,
    _marker: PhantomData<R>,
}

/// Smart pointer for accessing a resource in an exclusive manner.
pub struct ResMut<'r, R: Resource> {
    lock: RwLockWriteGuard<'r, dyn Resource>,
    _marker: PhantomData<R>,
}

/// The type to identify a resource in the world.
pub type ResourceId = TypeId;

/// The resource trait.
pub trait Resource: Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub(crate) fn downcast_res<R: Resource>(res: &dyn Resource) -> Option<&R> {
    res.as_any().downcast_ref::<R>()
}

pub(crate) fn downcast_res_mut<R: Resource>(res: &mut dyn Resource) -> Option<&mut R> {
    res.as_any_mut().downcast_mut::<R>()
}

pub(crate) fn res_id<R: Resource>() -> ResourceId {
    TypeId::of::<R>()
}

pub(crate) fn get_res_id<R: Resource + ?Sized>(_: &R) -> ResourceId {
    TypeId::of::<R>()
}

pub mod prelude {
    pub use super::table::{ResMutQueryResult, ResQueryError, ResQueryResult};
    pub use super::Resource;
    pub use super::{Res, ResMut};
}
