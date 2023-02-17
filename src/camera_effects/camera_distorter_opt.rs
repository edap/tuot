pub struct CameraDistorterOpt {
    pub noise_scale: f32,
    pub amplitude: f32, // 0.0 to 2.0
    pub easing: f32,    // 0.0 to 1.0
}

impl CameraDistorterOpt {
    pub fn default() -> CameraDistorterOpt {
        CameraDistorterOpt {
            noise_scale: 16.0,
            amplitude: 0.4, // 0.0 to 2.0
            easing: 0.8,    // 0.0 to 1.0
        }
    }
}
