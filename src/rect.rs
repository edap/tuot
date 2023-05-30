use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;

#[derive(Clone, Debug)]
enum Plane {
    XY,
    YZ,
    XZ,
}

#[derive(Debug)]
// Axis Aligned rectangle
pub struct Rect {
    a_bound: (f32, f32),
    b_bound: (f32, f32),
    a_idx: usize,
    b_idx: usize,
    k_idx: usize,
    plane: Plane,
    plane_normal: Vec3A,
    k: f32,
    mat: Material,
}

impl Rect {
    pub fn new_xy(a_bound: (f32, f32), b_bound: (f32, f32), k: f32, mat: Material) -> Self {
        Self::new(a_bound, b_bound, Plane::XY, k, mat)
    }

    pub fn new_yz(a_bound: (f32, f32), b_bound: (f32, f32), k: f32, mat: Material) -> Self {
        Self::new(a_bound, b_bound, Plane::YZ, k, mat)
    }

    pub fn new_xz(a_bound: (f32, f32), b_bound: (f32, f32), k: f32, mat: Material) -> Self {
        Self::new(a_bound, b_bound, Plane::XZ, k, mat)
    }

    fn new(a_bound: (f32, f32), b_bound: (f32, f32), plane: Plane, k: f32, mat: Material) -> Self {
        let (a_idx, b_idx, k_idx, plane_normal) = match plane {
            Plane::XY => (0, 1, 2, Vec3A::new(0.0, 0.0, 1.0)),
            Plane::YZ => (1, 2, 0, Vec3A::new(1.0, 0.0, 0.0)),
            Plane::XZ => (0, 2, 1, Vec3A::new(0.0, 1.0, 0.0)),
        };
        Self { a_bound, b_bound, a_idx, b_idx, k_idx, plane, plane_normal, k, mat }
    }
}

impl Hitable for Rect {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let origin = r.origin();
        let direction = r.direction();

        let t = (self.k - origin[self.k_idx]) / direction[self.k_idx];
        if t < t0 || t > t1 {
            return None;
        }

        let a = origin[self.a_idx] + t * direction[self.a_idx];
        if a < self.a_bound.0 || a > self.a_bound.1 {
            return None;
        }

        let b = origin[self.b_idx] + t * direction[self.b_idx];
        if b < self.b_bound.0 || b > self.b_bound.1 {
            return None;
        }

        let hit_rec = HitRecord{
            t,
            pos: r.point_at_parameter(t),
            normal: self.plane_normal,
            mat: &self.mat,
            u:(a - self.a_bound.0) / (self.a_bound.1 - self.a_bound.0),
            v:(b - self.b_bound.0) / (self.b_bound.1 - self.b_bound.0),
        };
        Some(hit_rec)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let a_min = self.a_bound.0;
        let a_max = self.a_bound.1;
        let b_min = self.b_bound.0;
        let b_max = self.b_bound.1;
        let k_min = self.k - 0.0001;
        let k_max = self.k + 0.0001;
        let (min, max) = match self.plane {
            Plane::XY => (Vec3A::new(a_min, b_min, k_min), Vec3A::new(a_max, b_max, k_max)),
            Plane::YZ => (Vec3A::new(k_min, a_min, b_min), Vec3A::new(k_max, a_max, b_max)),
            Plane::XZ => (Vec3A::new(a_min, k_min, b_min), Vec3A::new(a_max, k_max, b_max)),
        };
        Some(Aabb{min, max})
    }
}