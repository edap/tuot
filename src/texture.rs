use crate::color::Color;
use glam::Vec3A;

mod bitmap;
mod checker;
mod constant_color;
pub mod noise;
pub mod perlin;

use bitmap::Bitmap;
use checker::Checker;
use constant_color::ConstantColor;
use noise::Noise;

/// Texture object.
#[derive(Clone, Debug)]
pub enum Texture {
    Bitmap(Bitmap),
    Checker(Checker),
    ConstantColor(ConstantColor),
    Noise(Noise),
}

pub trait TextureObject {
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Color;
}

impl Texture {
    pub fn bitmap(path: &str) -> Texture {
        Texture::Bitmap(Bitmap {
            bitmap: image::open(path).unwrap(),
        })
    }

    pub fn checker(squares: usize, odd: Color, even: Color) -> Texture {
        Texture::Checker(Checker { squares, odd, even })
    }

    pub fn constant_color(color: Color) -> Texture {
        Texture::ConstantColor(ConstantColor { color })
    }

    pub fn noise(s: f32) -> Texture {
        Texture::Noise(Noise::new(s))
    }

    pub fn value(&self, u: f32, v: f32, p: Vec3A) -> Color {
        match self {
            Texture::Bitmap(bitmap) => bitmap.value(u, v, p),
            Texture::Checker(checker) => checker.value(u, v, p),
            Texture::ConstantColor(color) => color.value(u, v, p),
            Texture::Noise(noise) => noise.value(u, v, p),
        }
    }
}
