mod plugin;
mod runner;

use ecs::prelude::{IntoSystemSet, Resource, ScheduleLabel, World};
pub use plugin::*;
use runner::Runner;
use std::collections::HashSet;

pub struct App {
    plugins: HashSet<PluginName>,
    world: World,
    runner: Box<dyn Runner>,
}

impl App {
    pub fn new() -> Self {
        Self {
            plugins: HashSet::new(),
            world: World::new(),
            runner: Box::new(runner::simple_runner()),
        }
    }

    pub fn with_runner(mut self, runner: impl Runner) -> Self {
        self.runner = Box::new(runner);
        self
    }

    pub fn with_stop_condition(
        mut self,
        stop_condition: impl Fn(&World) -> bool + 'static,
    ) -> Self {
        self.runner = runner::simple_runner_with_stop_condition(stop_condition);
        self
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

    /// Runs and returns the [`World`] in the state after the run.
    pub fn run(self) -> World {
        (self.runner)(self.world)
    }

    pub fn world(&mut self) -> &mut World {
        &mut self.world
    }
}
