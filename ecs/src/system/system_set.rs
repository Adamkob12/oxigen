use super::*;
use crate::prelude::*;

pub trait SystemSet {
    fn systems(self) -> Vec<BoxedSystem>;
}

pub trait IntoSystemSet<Marker> {
    type SysSet: SystemSet;

    fn into_system_set(self) -> Self::SysSet;
}

macro_rules! impl_system_set {
    ($($system:ident),*) => {
        #[allow(non_snake_case, unused_parens)]
        impl<$($system: System + 'static),*> SystemSet for ($(Box<$system>),*) {
            fn systems(self) -> Vec<BoxedSystem> {
                let ($($system),*) = self;
                vec![$($system as BoxedSystem),*]
            }
        }
    }
}

all_tuples!(impl_system_set, 0, 15, S);

macro_rules! impl_into_sys_set {
    ($(($into_sys:ident, $mark:ident, $sys:ident)),*) => {
        #[allow(non_snake_case, unused_parens)]
        impl<$($into_sys,$mark,$sys: System + 'static),*> IntoSystemSet<($($mark),*)> for ($($into_sys,)*)
        where
            $($into_sys: IntoSystem<$mark, System = $sys>),*
        {
            type SysSet = ($(Box<$into_sys::System>),*);

            fn into_system_set(self) -> Self::SysSet {
                let ($($into_sys,)*) = self;
                ($(Box::new(IntoSystem::into_system($into_sys))),*)
            }
        }
    };
}

all_tuples!(impl_into_sys_set, 0, 15, S, M, T);
