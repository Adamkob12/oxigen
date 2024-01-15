use app::WorldPlugin;
use ecs::prelude::World;
use input::{InputWorldPlugin, Ssnn};
use render_2d::prelude::*;
use winit::{dpi::LogicalSize, event::Event, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

const LWIDTH: u32 = 1080;
const LHEIGHT: u32 = 720;

pub(crate) fn winit_runner(mut world: World) -> World {
    println!("Winit Runner!");
    env_logger::init();
    let event_loop = EventLoop::new();
    let input = Box::leak::<'static>(Box::new(WinitInputHelper::new()));
    let raw_input = input as *mut WinitInputHelper;
    let ssnn = Ssnn(std::ptr::NonNull::new(raw_input).unwrap());

    let window = {
        let size = LogicalSize::new(LWIDTH, LHEIGHT);
        WindowBuilder::new()
            .with_title("App")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // The app will never read from the pointer while its being mutated.
    InputWorldPlugin::from_input(ssnn).build(&mut world);
    // Init the world plugin before the event loop starts.
    Render2dPlugin::from_window(&window, LWIDTH as usize, LHEIGHT as usize).build(&mut world);
    // We can't move world into the event loop because it will take owndership of it.
    // So we pass an exclusive reference to the event loop.
    println!("Starting event loop!");
    let _ = event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            world.run_schedule::<Render>();
            if let Err(err) = world.get_resource::<SurfaceBuffer>().unwrap().render() {
                control_flow.set_exit();
                log::error!("Error rendering: {}", err);
            }
        }

        if !input.update(&event) {
            return;
        } // Wait untill the next update.

        world.update();
        window.request_redraw();
    });

    #[allow(unreachable_code)]
    world
}
