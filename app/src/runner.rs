use ecs::prelude::World;

pub trait Runner: FnOnce(World) -> World + 'static {}

impl<F: FnOnce(World) -> World + 'static> Runner for F {}

pub fn simple_runner() -> Box<dyn Runner> {
    Box::new(move |mut world: World| {
        world.run_startup_labels();

        loop {
            world.update();
        }
        #[allow(unreachable_code)]
        world
    })
}

pub fn simple_runner_with_stop_condition(
    stop_condition: impl Fn(&World) -> bool + 'static,
) -> Box<dyn Runner> {
    Box::new(move |mut world: World| {
        world.run_startup_labels();

        loop {
            world.update();

            if stop_condition(&world) {
                break;
            }
        }
        world
    })
}
