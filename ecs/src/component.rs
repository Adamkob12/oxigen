mod storage;

use std::alloc::Layout;

use bevy_ptr::OwningPtr;
use bevy_utils::all_tuples;
pub(crate) use storage::*;

/// The trait for all components.
pub trait Component: 'static + Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    fn comp_id(&self) -> ComponentId;
    fn layout(&self) -> std::alloc::Layout;
    fn drop_fn(&self) -> Option<unsafe fn(OwningPtr<'_>)>;
    fn name(&self) -> &'static str;
}

pub type ComponentId = std::any::TypeId;

pub fn comp_id<C: Component>() -> ComponentId {
    std::any::TypeId::of::<C>()
}

pub struct ComponentDesc {
    _name: &'static str,
    _id: ComponentId,
    layout: std::alloc::Layout,
    drop: Option<unsafe fn(bevy_ptr::OwningPtr<'_>)>,
}

pub(crate) fn _downcast_res<C: Component>(comp: &dyn Component) -> Option<&C> {
    comp.as_any().downcast_ref::<C>()
}

// SAFETY: The pointer points to a valid value of type `T` and it is safe to drop this value.
pub unsafe fn drop_ptr<T>(x: OwningPtr<'_>) {
    x.drop_as::<T>();
}
impl ComponentDesc {
    pub fn new<C: Component>() -> Self {
        Self {
            _name: std::any::type_name::<C>(),
            _id: std::any::TypeId::of::<C>(),
            layout: std::alloc::Layout::new::<C>(),
            drop: Some(drop_ptr::<C>),
        }
    }

    pub fn from_desc(
        comp_id: ComponentId,
        layout: Layout,
        name: &'static str,
        drop: Option<unsafe fn(OwningPtr<'_>)>,
    ) -> Self {
        Self {
            _name: name,
            _id: comp_id,
            layout,
            drop,
        }
    }
}

pub trait Bundle {
    fn components(self) -> Vec<Box<dyn Component>>;
}

impl<C: Component> Bundle for C {
    fn components(self) -> Vec<Box<dyn Component>> {
        vec![Box::new(self)]
    }
}

macro_rules! impl_bundle {
    ($($name:ident),*) => {
        impl<$($name: Bundle),*> Bundle for ($($name,)*) {
            #[allow(non_snake_case, unused_mut)]
            fn components(self) -> Vec<Box<dyn Component>> {
                let ($($name,)*) = self;
                let mut cmps = Vec::new();
                $(cmps.extend($name.components());)*
                cmps
            }
        }
    };
}

all_tuples!(impl_bundle, 0, 15, B);
