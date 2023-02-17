#[derive(PartialEq)]
pub enum DeflectionForce {
    CameraRay,
    ObjectToCamera,
}

pub struct DeflectionOpt {
    pub amplitude: f32,
    pub randomness: f32,
    pub force: DeflectionForce,
}

impl DeflectionOpt {
    pub fn default() -> DeflectionOpt {
        DeflectionOpt {
            amplitude: 0.9,  // the higher the more it diverge
            randomness: 0.0, // the higher the more the random value influence the normal.
            // keep it low for more unreal result, higher for result more similar to a montercarlo
            // path tracer
            force: DeflectionForce::CameraRay,
        }
    }
}
