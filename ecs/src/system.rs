pub mod func_system;
pub(crate) mod schedule;
pub(crate) mod system_param;
pub(crate) mod system_set;

use crate::prelude::{unsafe_world_cell::UnsafeWorldCell, *};
use bevy_utils::all_tuples;
pub use func_system::*;
pub use system_param::SystemParam;
pub use system_set::*;

/// The trait for all systems.
pub trait System {
    /// The name of the system.
    fn name(&self) -> &'static str;

    /// Run using an [`UnsafeWorldCell`] as opposed to a regular exclusive reference.
    unsafe fn run_unsafe(&mut self, world: UnsafeWorldCell<'_>);

    fn run(&mut self, world: &mut World) {
        let uwc = UnsafeWorldCell::from_world(world);
        // SAFETY: We have mutable access to the world, so we can safely create an UnsafeWorldCell
        // and run the system with it.
        unsafe { self.run_unsafe(uwc) };
    }

    /// Check for access conflicts within the system's parameters.
    fn check_conflict(&self) -> bool;

    /// Which of the [`World`]'s resources are [`Access`]ed by the system.
    fn access_table(&self) -> AccessTable;
}

pub type BoxedSystem = Box<dyn System>;

#[doc(hidden)]
pub struct BS; // Marks BoxedSystem

impl<S: System> IntoSystem<BS> for Box<S> {
    type System = S;

    fn into_system(this: Self) -> Self::System {
        *this
    }
}

// macro_rules! impl_system {
//     ($($param:ident),*) => {
//         #[allow(unused_parens)]
//         impl<F, $($param: SystemParam),*> System for SystemStruct<($($param),*), F>
//         // impl<F, $($param: SystemParam),*> System<()> for F
//         where
//             F: FnMut($($param::Item<'_>),*)
//         {
//             fn name(&self) -> &'static str {
//                 type_name::<F>()
//             }
//
//             #[allow(non_snake_case)]
//             unsafe fn run_unsafe(&mut self, world: UnsafeWorldCell<'_>) {
//                 (self.sys)($($param::fetch_from_world(world)),*);
//             }
//
//             #[allow(unused_mut)]
//             fn access_table(&self) -> AccessTable {
//                 let mut access_table = AccessTable::new();
//                 $(
//                     access_table.merge($param::access_table());
//                 )*
//                 access_table
//             }
//
//             fn check_conflict(&self) -> bool {
//                 self.access_table().is_conflicted()
//             }
//         }
//
//     };
// }
//
// all_tuples!(impl_system, 1, 15, P);
