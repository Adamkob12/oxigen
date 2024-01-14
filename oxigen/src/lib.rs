use app::App;

pub mod prelude {
    pub use app::*;
    pub use ecs::prelude::*;
    pub use input::*;
}

#[allow(non_snake_case)]
pub fn DefaultPlugins(app: &mut App) {
    app.add_plugin(winit::WinitPlugin);
}
