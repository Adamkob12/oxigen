use super::{draw::draw_all_entities, Render, SurfaceBuffer};
use app::*;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub struct Render2dPlugin {
    width: usize,
    height: usize,
    pixels: Pixels,
}

impl Render2dPlugin {
    pub fn from_window(window: &Window, logical_width: usize, logical_height: usize) -> Self {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        Self {
            width: logical_width,
            height: logical_height,
            pixels: Pixels::new(logical_width as u32, logical_height as u32, surface_texture)
                .unwrap(),
        }
    }
}

impl WorldPlugin for Render2dPlugin {
    fn build(self, world: &mut ecs::prelude::World) {
        world.insert_resource(SurfaceBuffer {
            width: self.width,
            height: self.height,
            pixels: self.pixels,
        });
    }
}

#[allow(non_snake_case)]
pub fn Render2dPipelinePlugin(app: &mut App) {
    app.add_systems(Render, (draw_all_entities,));
}
