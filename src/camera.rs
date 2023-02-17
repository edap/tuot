use crate::ray::Ray;
use crate::utils::random_in_unit_sphere;
use glam::Vec3A;

//#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3A,
    pub lower_left_corner: Vec3A,
    pub horizontal: Vec3A,
    pub vertical: Vec3A,
    pub lens_radius: f32,
    pub u: Vec3A,
    pub v: Vec3A,
    pub w: Vec3A,
}

impl Camera {
    pub fn new(lookfrom: Vec3A, lookat: Vec3A, vfov: f32, aspect: f32, aperture: f32) -> Camera {
        let vup = Vec3A::new(0.0, 1.0, 0.0);
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let focus_dist = lookfrom.distance(lookat);

        let w = (lookfrom - lookat).normalize();
        let u = (vup.cross(w)).normalize();
        let v = w.cross(u);

        Camera {
            lower_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_sphere();
        let offset = rd.x * self.u + rd.y * self.v;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
