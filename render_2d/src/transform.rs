pub use bevy_math::Vec3;
use ecs::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct Transform {
    /// The X and Y positions are the 2D coordinates of the sprite. The Z position is the depth.
    /// Entities with the lowest Z value will be drawn first, and entities with the highest Z value
    /// will be drawn last.
    pub position: Vec3,
}
