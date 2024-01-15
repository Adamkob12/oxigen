#![allow(unused_imports)]

mod component;
mod entity;
mod query;
mod resource;
mod system;
pub(crate) mod utils;
mod world;

pub mod prelude {
    pub use crate::component::{comp_id, drop_ptr, Bundle, Component, ComponentId};
    pub use crate::entity::{Entity, EntityWorldMut};
    pub use crate::query::{Query, QueryIter, WorldQuery};
    pub use crate::resource::prelude::*;
    pub use crate::system::*;
    pub use crate::world::*;
    pub use bevy_ptr::{OwningPtr, Ptr};
    pub use derive::*;
    pub(crate) use std::any::{type_name, TypeId};
}
