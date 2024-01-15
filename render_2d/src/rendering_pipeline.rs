mod draw;
mod plugin;

use crate::{prelude::Drawable, transform::Transform};
use bevy_math::Vec3Swizzles;
use ecs::prelude::*;
use pixels::Pixels;
pub use plugin::{Render2dPipelinePlugin, Render2dPlugin};
use std::sync::Arc;

pub(crate) trait DrawableEntity: Drawable + Component {}

impl<T> DrawableEntity for T where T: Drawable + Component {}

#[derive(Resource)]
pub struct SurfaceBuffer {
    width: usize,
    height: usize,
    pixels: Pixels,
}

impl SurfaceBuffer {
    pub(crate) fn draw_entity(&mut self, to_draw: &ToDraw) {
        if let Ok(tl /* top left */) =
            self.window_pos_to_pixel(to_draw.transform.position.xy().into())
        {
            let e_w = to_draw.drawable.width() as usize; // Entity width
            let e_h = to_draw.drawable.height() as usize; // Entity height
            let br = ((tl.0 + e_w).min(self.width), (tl.1 + e_h).min(self.height));
            let (l_x, t_y) = tl;
            let (r_x, b_y) = br;

            let src = to_draw.drawable.buffer().as_flat_samples().samples;
            let buff = self.pixels.frame_mut();
            println!("\n");
            println!("\n");
            println!("\n");
            for y in t_y..b_y {
                let row_start = y * self.width + l_x;
                let row_end = row_start + (r_x - l_x);
                let e_row_start = (y - t_y) * e_w;
                let e_row_end = e_row_start + (r_x - l_x);
                // Take into account everything is 4x (4 bytes per pixel).
                let row_start = row_start * 4;
                let row_end = row_end * 4;
                let e_row_start = e_row_start * 4;
                let e_row_end = e_row_end * 4;
                buff[row_start..row_end].copy_from_slice(&src[e_row_start..e_row_end]);
            }
        }
    }
}

#[derive(Component)]
pub(crate) struct ToDraw {
    pub(crate) transform: Transform,
    pub(crate) drawable: Arc<dyn DrawableEntity>,
}

impl std::ops::Deref for SurfaceBuffer {
    type Target = Pixels;

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

/// Schedule label for executing the render-related systems
pub struct Render;

impl ScheduleLabel for Render {
    const PLACE: usize = 99;
}
