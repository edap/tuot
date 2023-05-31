use std::fmt;
use crate::aabb::{surrounding_box, Aabb};
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;
// TODO, Material should be an Arc, as when we load the material from
// an obj, the same material is shared in a safe thread mode between more triangles

#[derive(Debug, Clone, Copy)]
pub struct HitRecord<'material> {
    pub t: f32,
    pub pos: Vec3A,
    pub normal: Vec3A,
    pub mat: &'material Material,
    pub u: f32,
    pub v: f32,
}

pub struct HitableStore(Vec<Box<dyn Hitable + Send + Sync>>);

pub trait Hitable: Sync {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<Aabb>;
}

impl fmt::Debug for dyn Hitable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hitable {{ aabb: {:?} }}", self.bounding_box())
    }
}

impl Hitable for Vec<Box<dyn Hitable + Send + Sync>> {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;

        for hitable in self.iter() {
            if let Some(candidate_hit) = hitable.hit(r, tmin, tmax) {
                match hit {
                    None => hit = Some(candidate_hit),
                    Some(prev) => {
                        if candidate_hit.t < prev.t {
                            hit = Some(candidate_hit);
                        }
                    }
                }
            }
        }

        hit
    }

    fn bounding_box(&self) -> Option<Aabb> {
        if self.len() < 1 {
            return None;
        }

        let mut result: Aabb;
        let first = self[0].bounding_box();
        match first {
            Some(b) => result = b,
            None => return None,
        }

        for i in 1..self.len() {
            if let Some(b) = self[i].bounding_box() {
                result = surrounding_box(&result, &b);
            } else {
                return None;
            }
        }

        Some(result)
    }
}

impl HitableStore {
    pub fn new() -> Self {
        HitableStore(Vec::new())
    }

    pub fn push<H: Hitable + Send + Sync + 'static>(&mut self, hitable: H) {
        self.0.push(Box::new(hitable))
    }
}

impl ::std::ops::Deref for HitableStore {
    type Target = Vec<Box<dyn Hitable + Send + Sync>>;

    fn deref(&self) -> &Vec<Box<dyn Hitable + Send + Sync>> {
        &self.0
    }
}

impl ::std::ops::DerefMut for HitableStore {
    // You don't need type Target = Vec<Box<dyn Hitable + Send + Sync>>; here because it already knows thanks to Deref
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
