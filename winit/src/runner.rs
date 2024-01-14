use app::WorldPlugin;
use ecs::prelude::World;
use input::{InputWorldPlugin, SSNN};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

const WINDOW_WIDTH: u32 = 1080;
const WINDOW_HEIGHT: u32 = 720;

pub(crate) fn winit_runner(mut world: World) -> World {
    println!("Winit Runner!");
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let input = Box::leak::<'static>(Box::new(WinitInputHelper::new()));
    let raw_input = input as *mut WinitInputHelper;
    let ssnn = SSNN(std::ptr::NonNull::new(raw_input).unwrap());

    let _window = {
        let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        WindowBuilder::new()
            .with_title("App")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    world.run_startup_labels();
    // The app will never read from the pointer while its being mutated.
    InputWorldPlugin::from_input(ssnn).build(&mut world);

    // We can't move world into the event loop because it will take owndership of it.
    // So we pass an exclusive reference to the event loop.
    let world_xref = &mut world;
    println!("Starting event loop!");
    let _ = event_loop
        .run(move |event, _elwt| {
            if !input.update(&event) {
                return;
            } // Wait untill the next update.

            world_xref.update();
        })
        .expect("Couldn't execute event loop");

    world
}
