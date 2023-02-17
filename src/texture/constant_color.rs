use super::TextureObject;
use crate::color::Color;
use glam::Vec3A;

#[derive(Clone, Debug)]
pub struct ConstantColor {
    pub color: Color,
}

impl TextureObject for ConstantColor {
    fn value(&self, _u: f32, _v: f32, _p: Vec3A) -> Color {
        self.color
    }
}
