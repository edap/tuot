use crate::camera::Camera;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::utils::random_in_unit_sphere;
use glam::Vec3A;

pub fn get_distorted_ray(
    cam: &Camera,
    s: f32,
    t: f32,
    noise: &Texture,
    amp: f32,
    easing: f32,
) -> Ray {
    let rd = cam.lens_radius * random_in_unit_sphere();
    let noised = noise.value(s, t, Vec3A::new(s, t, 0.0));

    let offset = rd.x * cam.u + rd.y * cam.v;

    // use t because the distortsion grows on the y axis
    let smooth_t = exponential_easing(t, easing);
    let ns = s + noised.red * (amp * smooth_t);
    let nt = t + noised.green * (amp * smooth_t);

    Ray::new(
        cam.origin + offset,
        cam.lower_left_corner + ns * cam.horizontal + nt * cam.vertical - cam.origin - offset,
        //Vec3A::new(ns, nt, 0.0),
    )
}

// http://www.flong.com/archive/texts/code/shapers_exp/index.html
fn exponential_easing(x: f32, a: f32) -> f32 {
    let epsilon: f32 = 0.00001;
    let min_param_a: f32 = 0.0 + epsilon;
    let max_param_a: f32 = 1.0 - epsilon;
    let a = min_param_a.max(max_param_a.min(a));

    if a < 0.5 {
        // emphasis
        let a = 2.0 * (a);
        let y = x.powf(a);
        return y;
    } else {
        // de-emphasis
        let a = 2.0 * (a - 0.5);
        let y = x.powf(1.0 / (1.0 - a));
        return y;
    }
}
