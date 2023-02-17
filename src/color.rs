use core::fmt;
use image::{Pixel, Rgba};
use rand::Rng;
//use serde::Deserialize;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    pub fn lerp(&self, new_color: Color, pct: f32) -> Color {
        let p = pct.min(1.0).max(0.0);
        Color {
            red: (self.red * (1.0 - p)) + new_color.red * p,
            blue: (self.blue * (1.0 - p)) + new_color.blue * p,
            green: (self.green * (1.0 - p)) + new_color.green * p,
        }
    }

    pub fn random() -> Color {
        let mut rng = rand::thread_rng();
        Color {
            red: rng.gen::<f32>(),
            green: rng.gen::<f32>(),
            blue: rng.gen::<f32>(),
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        image::Rgba([
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255,
        ])
    }

    // pub fn to_rgba_mut(&self) -> Rgba<u8> {
    //     image::Rgba([
    //         (gamma_encode(self.red) * 255.0) as u8,
    //         (gamma_encode(self.green) * 255.0) as u8,
    //         (gamma_encode(self.blue) * 255.0) as u8,
    //         255,
    //     ])
    // }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: gamma_decode(rgba.channels()[0] as f32 / 255.0),
            green: gamma_decode(rgba.channels()[1] as f32 / 255.0),
            blue: gamma_decode(rgba.channels()[2] as f32 / 255.0),
        }
    }

    pub fn from_array(array: [u8; 4]) -> Color {
        Color {
            red: gamma_decode(array[0] as f32 / 255.0),
            green: gamma_decode(array[1] as f32 / 255.0),
            blue: gamma_decode(array[2] as f32 / 255.0),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "r: {:?} g: {:?} b: {:?}",
            self.red, self.green, self.blue,
        )
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}
impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, other: f32) -> Color {
        Color {
            red: self.red / other,
            blue: self.blue / other,
            green: self.green / other,
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.red += other.red;
        self.blue += other.blue;
        self.green += other.green;
    }
}

impl Add<f32> for Color {
    type Output = Color;
    fn add(self, other: f32) -> Color {
        Color {
            red: self.red + other,
            green: self.blue + other,
            blue: self.green + other,
        }
    }
}

impl AddAssign<f32> for Color {
    fn add_assign(&mut self, other: f32) {
        self.red += other;
        self.blue += other;
        self.green += other;
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, t: f32) {
        let k = 1.0 / t;
        self.red *= k;
        self.blue *= k;
        self.green *= k;
    }
}
