use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
//use crate::setup::SDF_DETAIL_SCALE;
use glam::Vec3A;
use sdfu::SDF;

//use sdfu::*;

const MAX_MARCHES: u32 = 256;
const EPSILON: f32 = 0.0001;
//const MAX_VIS_MARCHES: u32 = 100;

pub struct TracedSDF<S> {
    sdf: S,
    mat: Material,
    // radius and position are used just to create the AABB box, they are totally
    // unrelate to the SDF struct.
    // in case you scale the SDF, you have to scale the radius too
    aabb_radius: f32,
    aabb_position: Vec3A,
}

impl<S> TracedSDF<S> {
    pub fn new(sdf: S, mat: Material, aabb_radius: f32, aabb_position: Vec3A) -> Self {
        TracedSDF {
            sdf,
            mat,
            aabb_radius,
            aabb_position,
        }
    }
}

impl<S: SDF<f32, Vec3A> + Send + Sync> Hitable for TracedSDF<S> {
    fn hit(&self, ray: &Ray, _t0: f32, t1: f32) -> Option<HitRecord> {
        let mut t = 0.0;
        let mut hit = false;

        for _march in 0..MAX_MARCHES {
            let pos = ray.point_at_parameter(t);
            let dist = self.sdf.dist(pos);
            if dist < EPSILON {
                hit = true;
                break;
            }

            t += dist;

            if t > t1 || t.is_nan() {
                break;
            }
        }

        // you had to decrease t otherwise there was a self intersection with the surface of the hit object
        t -= EPSILON;
        if hit {
            let normals = self.sdf.normals_fast(EPSILON);
            let normal = normals.normal_at(ray.point_at_parameter(t));
            //let front_face = ray.direction.dot(normal) < 0.0;
            // normal: if front_face { normal } else { -normal },

            return Some(HitRecord {
                t,
                pos: ray.point_at_parameter(t),
                normal,
                mat: &self.mat,
                u: 0.0,
                v: 0.0,
            });
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            // use abs because sometime you are creating sphere with a negative radius, so that the normals are flipped.
            // But of course you do not want Aabb with negative values
            min: self.aabb_position
                - Vec3A::new(
                    self.aabb_radius.abs(),
                    self.aabb_radius.abs(),
                    self.aabb_radius.abs(),
                ),
            max: self.aabb_position
                + Vec3A::new(
                    self.aabb_radius.abs(),
                    self.aabb_radius.abs(),
                    self.aabb_radius.abs(),
                ),
        })
    }
}

#[test]
fn test_glam() {
    use sdfu::SDF;
    let sdf = sdfu::Sphere::new(1.0);
    let dist: f32 = sdf.dist(Vec3A::ZERO);
    assert_eq!(dist, -1.0);
}
