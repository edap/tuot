use crate::color::Color;
// This part has been taken from https://github.com/fu5ha/rayn/

// The level of detail to render SDFs with (which is how the fractal is rendered).
// Closer to 0 = smaller detail will be shown. Larger means less detail.
pub const SDF_DETAIL_SCALE: f32 = 2.0;

// The number of iterations to run of the fractal function. More iterations will mean
// higher potential detail but also higher render times. If you use lower iterations, the
// surface of the fractal will be more sparsely defined, so you should use a higher SDF_DETAIL_SCALE
// in order to see it better, whereas with more iterations the surface will be more defined so you can
// use a lower (more detailed) SDF_DETAIL_SCALE.
pub const FRACTAL_ITERATIONS: usize = 12;

pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};
