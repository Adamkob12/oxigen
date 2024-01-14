mod runner;

use app::{App, Plugin};

pub struct WinitPlugin;

impl Plugin for WinitPlugin {
    fn build(self, app: &mut App) {
        app.set_runner(runner::winit_runner);
    }
}
