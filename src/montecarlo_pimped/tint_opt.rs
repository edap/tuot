use crate::color::Color;
#[derive(PartialEq)]
pub enum BandOp {
    Sin,
    Fract,
    Mod,
}
pub struct TintOpt {
    pub normal_color: Color,
    pub background_color: Color,
    pub mix: f32,
    pub freq: f32,
    pub amplitude: f32,
    pub band_op: BandOp,
}

impl TintOpt {
    pub fn default() -> TintOpt {
        TintOpt {
            normal_color: Color {
                red: 1.0,
                green: 0.5,
                blue: 1.0,
            },
            background_color: Color {
                red: 0.8196,
                green: 0.7568,
                blue: 0.3490,
            },
            mix: 0.9,
            freq: 45.0,
            amplitude: 40.0,
            band_op: BandOp::Sin,
        }
    }
}
