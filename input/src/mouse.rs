use crate::Ssnn;
use ecs::prelude::Resource;
pub use winit::event::MouseButton;
pub use winit_input_helper::TextChar;
use winit_input_helper::WinitInputHelper;

#[derive(Resource)]
pub struct Mouse {
    pub(crate) input: Ssnn<WinitInputHelper>,
}

fn mouse_button_as_usize(button: MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(n) => n as usize,
    }
}

impl Mouse {
    fn input(&self) -> &WinitInputHelper {
        unsafe { self.input.0.as_ref() }
    }
    /// Returns true if the button went from being "not pressed" to "pressed"
    pub fn pressed(&self, button: MouseButton) -> bool {
        self.input().mouse_held(mouse_button_as_usize(button))
    }

    // Returns true if the button went from being "pressed" to "not pressed"
    pub fn released(&self, button: MouseButton) -> bool {
        self.input().mouse_held(mouse_button_as_usize(button))
    }

    /// Returns true if the button is currently "pressed"
    pub fn held(&self, button: MouseButton) -> bool {
        self.input().mouse_held(mouse_button_as_usize(button))
    }

    /// Returns the change in mouse coordinates that occured.
    pub fn diff(&self) -> (f32, f32) {
        self.input().mouse_diff()
    }

    /// Returns the characters pressed since the last frame.
    pub fn test(&self) -> Vec<TextChar> {
        self.input().text()
    }
}
