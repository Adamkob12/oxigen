use std::{path::Path, sync::Arc};

use crate::rendering_pipeline::ToDraw;

use super::transform::Transform;
use ecs::prelude::*;
use image::RgbaImage;

#[derive(Component)]
pub struct Sprite {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixels: RgbaImage,
}

impl Sprite {
    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let img = image::io::Reader::open(path)
            .ok()?
            .decode()
            .ok()?
            .into_rgba8();

        Some(Sprite {
            width: img.width(),
            height: img.height(),
            pixels: img,
        })
    }
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
