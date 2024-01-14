mod plugin;

use ecs::prelude::{IntoSystemSet, Resource, ScheduleLabel, World};
pub use plugin::*;
use std::collections::HashSet;

pub struct App {
    plugins: HashSet<PluginName>,
    world: World,
    stop_condition: &'static dyn Fn(&World) -> bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            plugins: HashSet::new(),
            world: World::new(),
            stop_condition: &|_| false,
        }
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
        assert!(
            self.plugins.insert(plugin.name()),
            "Plugin {} already added",
            plugin.name()
        );
        plugin.build(self);
        self
    }

    pub fn init_resource<R: Resource + Default>(&mut self) -> &mut Self {
        self.world.init_resource::<R>();
        self
    }

    pub fn insert_resource<R: Resource>(&mut self, res: R) -> &mut Self {
        self.world.insert_resource(res);
        self
    }

    pub fn add_systems<M, L>(&mut self, _label: L, sys_set: impl IntoSystemSet<M>) -> &mut Self
    where
        L: ScheduleLabel,
    {
        self.world.add_systems::<M, L>(sys_set);
        self
    }

    pub fn set_stop_condition(
        &mut self,
        stop_condition: &'static dyn Fn(&World) -> bool,
    ) -> &mut Self {
        self.stop_condition = stop_condition;
        self
    }

    pub fn run(&mut self) {
        self.world.run(Some(self.stop_condition))
    }

    pub fn world(&mut self) -> &mut World {
        &mut self.world
    }
}
