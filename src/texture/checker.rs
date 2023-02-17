use super::TextureObject;
use crate::color::Color;
use glam::Vec3A;

#[derive(Clone, Debug)]
pub struct Checker {
    pub squares: usize,
    pub even: Color,
    pub odd: Color,
}

impl TextureObject for Checker {
    fn value(&self, u: f32, v: f32, _p: Vec3A) -> Color {
        let x = (u * self.squares as f32) as u32;
        let y = (v * self.squares as f32) as u32;

        match (is_even(x), is_even(y)) {
            (true, true) | (false, false) => self.even,
            _ => self.odd,
        }
    }
}

fn is_even(v: u32) -> bool {
    v % 2 == 0
}
