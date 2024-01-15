use image::RgbaImage;

use crate::prelude::Sprite;

pub trait Drawable {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn buffer(&self) -> &RgbaImage;
}

impl Drawable for Sprite {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn buffer(&self) -> &RgbaImage {
        &self.pixels
    }
}
