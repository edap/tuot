use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::u_v_from_sphere_hit_point;
use glam::Vec3A;

#[derive(Debug)]
pub struct Sphere {
    pub position: Vec3A,
    pub radius: f32,
    pub mat: Material,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.position;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root_a = ((-half_b) - sqrtd) / a;
            let root_b = ((-half_b) + sqrtd) / a;
            for root in [root_a, root_b].iter() {
                if *root < t_max && *root > t_min {
                    let p = ray.point_at_parameter(*root);
                    let normal = (p - self.position) / self.radius;
                    //let front_face = ray.direction.dot(normal) < 0.0;

                    let (u, v) = u_v_from_sphere_hit_point(p - self.position);

                    return Some(HitRecord {
                        t: *root,
                        pos: p,
                        normal,
                        mat: &self.mat,
                        u,
                        v,
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            // use abs because sometime you are creating sphere with a negative radius, so that the normals are flipped.
            // But of course you do not want Aabb with negative values
            min: self.position
                - Vec3A::new(self.radius.abs(), self.radius.abs(), self.radius.abs()),
            max: self.position
                + Vec3A::new(self.radius.abs(), self.radius.abs(), self.radius.abs()),
        })
    }
}
