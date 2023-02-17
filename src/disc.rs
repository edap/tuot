use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;

#[derive(Debug)]
pub struct Disc {
    pub position: Vec3A,
    pub normal: Vec3A,
    pub radius: f32,
    pub mat: Material,
}

impl Hitable for Disc {
    fn bounding_box(&self) -> Option<Aabb> {
        let corner = Vec3A::new(self.radius, self.radius, self.radius);

        Some(Aabb {
            min: self.position - corner,
            max: self.position + corner,
        })
    }

    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() > std::f32::EPSILON {
            let v = self.position - ray.origin;

            let distance = v.dot(self.normal) / denom;

            let p = ray.origin + distance * ray.direction;
            let d = p - self.position;

            let n = d.x * d.x + d.y * d.y + d.z * d.z;

            if n < self.radius * self.radius && distance < max && distance > min {
                return Some(HitRecord {
                    t: distance,
                    pos: ray.point_at_parameter(distance),
                    normal: self.normal,
                    mat: &self.mat,
                    u: 0.0,
                    v: 0.0,
                });
            } else {
                None
            }
        } else {
            None
        }
    }
}
