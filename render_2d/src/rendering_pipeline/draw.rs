use std::cmp::Ordering;

use super::{SurfaceBuffer, ToDraw};
use ecs::prelude::*;

pub fn draw_all_entities(
    drawable_entities: Query<&ToDraw>,
    mut surface_buffer: ResMut<SurfaceBuffer>,
) {
    let mut drawables: Vec<&ToDraw> = drawable_entities.into_iter().collect();
    drawables.sort_by(|a, b| {
        a.transform
            .position
            .z
            .partial_cmp(&b.transform.position.z)
            .unwrap_or(Ordering::Equal)
    });
    for drawable in drawables {
        surface_buffer.draw_entity(drawable);
    }
}
