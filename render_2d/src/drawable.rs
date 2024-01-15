use image::RgbaImage;

use crate::prelude::Sprite;

pub trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn buffer(&self) -> &RgbaImage;
}

impl Drawable for Sprite {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn buffer(&self) -> &RgbaImage {
        &self.pixels
    }
}
