use super::TextureObject;
use crate::color::Color;
use glam::Vec3A;

use image::{DynamicImage, GenericImageView};

#[derive(Clone)]
pub struct Bitmap {
    pub bitmap: DynamicImage,
}

impl TextureObject for Bitmap {
    fn value(&self, u: f32, v: f32, _p: Vec3A) -> Color {
        let (width, height) = self.bitmap.dimensions();

        let mut i = (u * width as f32) as u32;
        let mut j = ((1.0 - v) * (height as f32) - 0.001) as u32;

        i = i.min(width - 1);
        j = j.min(height - 1);

        let pixel: image::Rgba<u8> = self.bitmap.get_pixel(i, j);
        // let data = pixel.data;

        // Color::from_u8(data[0], data[1], data[2])
        Color::from_rgba(pixel)
    }
}

impl std::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (width, height) = self.bitmap.dimensions();
        write!(f, "Bitmap {{ width: {}, height: {} }}", width, height)
    }
}
