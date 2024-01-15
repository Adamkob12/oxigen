use std::sync::Arc;

use crate::rendering_pipeline::ToDraw;

use super::transform::Transform;
use ecs::prelude::*;
use image::RgbaImage;

#[derive(Component)]
pub struct Sprite {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) pixels: RgbaImage,
}

pub struct SpriteBundle {
    sprite: Arc<Sprite>,
    transform: Transform,
}

impl SpriteBundle {
    pub fn from_sprite(sprite: Arc<Sprite>) -> Self {
        Self {
            sprite,
            transform: Transform::default(),
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

impl Bundle for SpriteBundle {
    fn components(self) -> Vec<Box<dyn Component>> {
        vec![Box::new(ToDraw {
            drawable: self.sprite,
            transform: self.transform,
        })]
    }
}
