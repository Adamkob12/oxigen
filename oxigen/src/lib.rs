use app::App;

pub mod prelude {
    pub use super::DefaultPlugins;
    pub use app::*;
    pub use ecs::prelude::*;
    pub use input::*;
    pub use render_2d::prelude::*;
}

#[allow(non_snake_case)]
pub fn DefaultPlugins(app: &mut App) {
    app.add_plugin(runner::RunnerPlugin);
    app.add_plugin(render_2d::prelude::Render2dPipelinePlugin);
}
