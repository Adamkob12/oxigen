use crate::App;
use std::borrow::Cow;

pub type PluginName = Cow<'static, str>;

pub trait Plugin {
    fn name(&self) -> PluginName;
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
