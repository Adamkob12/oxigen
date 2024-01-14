use ecs::prelude::World;

use crate::App;
use std::borrow::Cow;

pub type PluginName = Cow<'static, str>;

pub trait Plugin {
    fn name(&self) -> PluginName {
        std::any::type_name::<Self>().into()
    }
    fn build(self, world: &mut App);
}

impl<F> Plugin for F
where
    F: FnMut(&mut App),
{
    fn name(&self) -> PluginName {
        std::any::type_name::<F>().into()
    }

    fn build(mut self, world: &mut App) {
        (self)(world)
    }
}

/// Like a normal plugin, but it can be applied after [`App::run`]
pub trait WorldPlugin {
    fn name(&self) -> PluginName {
        std::any::type_name::<Self>().into()
    }
    fn build(self, world: &mut World);
}

impl<F> WorldPlugin for F
where
    F: FnMut(&mut World),
{
    fn name(&self) -> PluginName {
        std::any::type_name::<F>().into()
    }

    fn build(mut self, world: &mut World) {
        (self)(world)
    }
}
