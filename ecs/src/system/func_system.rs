use std::{any::Any, marker::PhantomData};

use bevy_utils::all_tuples;

use crate::prelude::*;

pub trait SystemParamFunction<Marker>: Send + Sync + 'static {
    type Param: SystemParam;

    fn run(&mut self, world: &mut World, param: <Self::Param as SystemParam>::Item<'_>);
}

pub trait IntoSystem<Marker>: Sized {
    /// The type of [`System`] that this instance converts into.
    type System: System;

    /// Turns this value into its corresponding [`System`].
    fn into_system(this: Self) -> Self::System;
}

pub struct FunctionSystemStruct<Marker, F>
where
    F: SystemParamFunction<Marker>,
{
    func: F,
    // NOTE: PhantomData<fn()-> T> gives this safe Send/Sync impls
    marker: PhantomData<fn() -> Marker>,
}

#[doc(hidden)]
pub struct SPF; // Marks SystemParamFunction

impl<Marker, F> System for FunctionSystemStruct<Marker, F>
where
    F: SystemParamFunction<Marker>,
{
    fn name(&self) -> &'static str {
        type_name::<F>()
    }

    unsafe fn run_unsafe(&mut self, world: UnsafeWorldCell<'_>) {
        self.func
            .run(world.world_mut(), F::Param::fetch_from_world(world));
    }

    fn access_table(&self) -> AccessTable {
        F::Param::access_table()
    }

    fn check_conflict(&self) -> bool {
        self.access_table().is_conflicted() || F::Param::access_to_whole_world().is_conflicted()
    }
}

impl<Marker, F> IntoSystem<(SPF, Marker)> for F
where
    F: SystemParamFunction<Marker>,
{
    type System = FunctionSystemStruct<Marker, F>;
    fn into_system(this: Self) -> Self::System {
        FunctionSystemStruct {
            func: this,
            marker: PhantomData,
        }
    }
}

macro_rules! impl_system_param_function {
    ($($param:ident),*) => {
        #[allow(unused, non_snake_case)]
        impl<F: Send + Sync + 'static, $($param: SystemParam),*> SystemParamFunction<fn($($param),*)> for F
        where for <'a> &'a mut F:
            FnMut($($param::Item<'_>),*) + FnMut($($param),*)
        {
            type Param = ($($param),*);

            fn run(&mut self, world: &mut World, param: <Self::Param as SystemParam>::Item<'_>) {
                // Yes, this is strange, but `rustc` fails to compile this impl
                // without using this function. It fails to recognize that `func`
                // is a function, potentially because of the multiple impls of `FnMut`
                #[allow(clippy::too_many_arguments)]
                fn call_inner<$($param,)*>(
                    mut f: impl FnMut($($param,)*),
                    $($param: $param,)*
                ){
                    f($($param,)*)
                }
                let ($($param),*) = param;
                call_inner(self, $($param),*)
            }
        }
    };
}

all_tuples!(impl_system_param_function, 0, 15, P);
