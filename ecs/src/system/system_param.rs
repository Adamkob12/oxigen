use std::any::TypeId;

use bevy_utils::{all_tuples, HashMap};

use crate::{prelude::*, resource::res_id};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessToWorld {
    NoAccess,
    Access(Access),
    ConflictingAccess,
}

/// A trait for types that can be used as a system parameter.
pub unsafe trait SystemParam {
    /// The type of the item that will be passed to the system.
    type Item<'a>;

    unsafe fn fetch_from_world(world: UnsafeWorldCell<'_>) -> Self::Item<'_>;

    fn access_table() -> AccessTable;

    fn access_to_whole_world() -> AccessToWorld {
        AccessToWorld::NoAccess
    }
}

impl AccessToWorld {
    pub fn is_conflicted(&self) -> bool {
        matches!(self, AccessToWorld::ConflictingAccess)
    }
}

unsafe impl<R: Resource> SystemParam for Res<'_, R> {
    type Item<'a> = Res<'a, R>;

    unsafe fn fetch_from_world(world: UnsafeWorldCell<'_>) -> Self::Item<'_> {
        world.get_resource::<R>().unwrap()
    }

    fn access_table() -> AccessTable {
        let mut access_table = AccessTable::new();
        access_table.insert(res_id::<R>(), Access::Read);
        access_table
    }
}

unsafe impl<R: Resource> SystemParam for ResMut<'_, R> {
    type Item<'a> = ResMut<'a, R>;

    unsafe fn fetch_from_world(world: UnsafeWorldCell<'_>) -> Self::Item<'_> {
        world.get_resource_mut::<R>().unwrap()
    }

    fn access_table() -> AccessTable {
        let mut access_table = AccessTable::new();
        access_table.insert(res_id::<R>(), Access::Write);
        access_table
    }
}

unsafe impl<Q: WorldQuery> SystemParam for Query<'_, Q> {
    type Item<'a> = Query<'a, Q>;

    unsafe fn fetch_from_world(world: UnsafeWorldCell<'_>) -> Self::Item<'_> {
        world.query::<Q>()
    }

    fn access_table() -> AccessTable {
        Q::access_table()
    }
}

unsafe impl SystemParam for &World {
    type Item<'a> = &'a World;

    unsafe fn fetch_from_world(world: UnsafeWorldCell<'_>) -> Self::Item<'_> {
        unsafe { world.world() }
    }

    fn access_table() -> AccessTable {
        AccessTable::new()
    }

    fn access_to_whole_world() -> AccessToWorld {
        AccessToWorld::Access(Access::Read)
    }
}

unsafe impl SystemParam for &mut World {
    type Item<'a> = &'a mut World;

    unsafe fn fetch_from_world(world: UnsafeWorldCell<'_>) -> Self::Item<'_> {
        unsafe { world.world_mut() }
    }

    fn access_table() -> AccessTable {
        AccessTable::new()
    }

    fn access_to_whole_world() -> AccessToWorld {
        AccessToWorld::Access(Access::Write)
    }
}

macro_rules! impl_tuple_sys_param {
    ($($param:ident),*) => {
        #[allow(unused)]
        unsafe impl<$($param: SystemParam),*> SystemParam for ($($param,)*) {
            type Item<'a> = ($($param::Item<'a>,)*);

            unsafe fn fetch_from_world(world: UnsafeWorldCell<'_>) -> Self::Item<'_> {
                ($($param::fetch_from_world(world),)*)
            }

            fn access_table() -> AccessTable {
                let mut access_table = AccessTable::new();
                $(
                    access_table.merge($param::access_table());
                )*
                access_table
            }

            fn access_to_whole_world() -> AccessToWorld {
                let mut access = AccessToWorld::NoAccess;
                $(
                     match (access, $param::access_to_whole_world()) {
                            (AccessToWorld::NoAccess, a) => access = a,
                            (_, AccessToWorld::NoAccess) => {}
                            (_, AccessToWorld::ConflictingAccess) => access = AccessToWorld::ConflictingAccess,
                            (AccessToWorld::Access(Access::Read), AccessToWorld::Access(Access::Read)) => {}
                            _ => access = AccessToWorld::ConflictingAccess,
                    }
                )*
                access
            }
        }
    };
}

all_tuples!(impl_tuple_sys_param, 0, 15, P);
