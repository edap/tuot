pub mod camera_distorter;
pub mod camera_distorter_opt;

#[derive(PartialEq)]
pub enum CameraEffects {
    Distorter,
    NoEffects,
}
