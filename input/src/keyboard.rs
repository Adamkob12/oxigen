use ecs::prelude::Resource;

use crate::SSNN;
pub use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

#[derive(Resource)]
pub struct Keyboard {
    pub(crate) input: SSNN<WinitInputHelper>,
}

impl Keyboard {
    fn input(&self) -> &WinitInputHelper {
        unsafe { self.input.0.as_ref() }
    }
    /// Returns true if the key went from being "not pressed" to "pressed"
    pub fn pressed(&self, keycode: KeyCode) -> bool {
        self.input().key_pressed(keycode)
    }

    // Returns true if the key went from being "pressed" to "not pressed"
    pub fn released(&self, keycode: KeyCode) -> bool {
        self.input().key_released(keycode)
    }

    /// Returns true if the key is currently "pressed"
    pub fn held(&self, keycode: KeyCode) -> bool {
        self.input().key_held(keycode)
    }
}
