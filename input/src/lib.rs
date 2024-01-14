mod keyboard;
mod mouse;

use app::WorldPlugin;
use ecs::prelude::World;
pub use keyboard::*;
pub use mouse::*;
use std::ptr::NonNull;
use winit_input_helper::WinitInputHelper;

/// Send & Sync NotNull Pointer.
#[derive(Copy, Clone)]
pub struct SSNN<T>(pub NonNull<T>);

unsafe impl<T: Send> Send for SSNN<T> {}
unsafe impl<T: Sync> Sync for SSNN<T> {}

pub struct InputWorldPlugin {
    // This must be to a static reference on the heap.
    input: SSNN<WinitInputHelper>,
}

impl InputWorldPlugin {
    pub fn from_input(input: SSNN<WinitInputHelper>) -> Self {
        Self { input }
    }
}

impl WorldPlugin for InputWorldPlugin {
    fn build(self, world: &mut World) {
        world.insert_resource(Mouse {
            input: self.input.clone(),
        });
        world.insert_resource(Keyboard {
            input: self.input.clone(),
        });
    }
}
