mod runner;

use app::{App, Plugin};
pub use winit;
pub use winit_input_helper;

pub struct RunnerPlugin;

impl Plugin for RunnerPlugin {
    fn build(self, app: &mut App) {
        app.set_runner(runner::winit_runner);
    }
}
