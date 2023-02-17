use super::TextureObject;
use crate::color::Color;
use crate::texture::perlin::Perlin;
use glam::Vec3A;

#[derive(Clone, Debug)]
pub struct Noise {
    pub scale: f32,
    pub perlin: Perlin,
}

impl Noise {
    pub fn new(scale: f32) -> Self {
        Self {
            scale,
            perlin: Perlin::new(),
        }
    }

    fn turbulence(&self, point: &Vec3A, depth: u32) -> f32 {
        let mut acc = 0.0;
        let mut temp_point = point.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            acc += weight * self.perlin.noise(&temp_point);
            weight *= 0.5;
            temp_point *= 2.0;
        }

        acc.abs()
    }
}

impl TextureObject for Noise {
    fn value(&self, _u: f32, _v: f32, point: Vec3A) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0
                + (self.scale * point[0] + 5.0 * self.turbulence(&(self.scale * point), 7)).sin())
    }
}
