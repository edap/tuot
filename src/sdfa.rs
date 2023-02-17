// WIP
// use crate::aabb::Aabb;
// use crate::hitable::{HitRecord, Hitable};
// use crate::material::Material;
// use crate::ray::Ray;
//use crate::setup::SDF_DETAIL_SCALE;
use glam::{vec2, vec3, Vec2Swizzles, Vec3};

pub fn sphere(p: Vec3, radius: f32) -> f32 {
    p.length() - radius
}

pub fn plane(p: Vec3, n: Vec3, h: f32) -> f32 {
    p.dot(n) + h
}

pub fn map(pos: Vec3) -> f32 {
    f32::min(sphere(pos, 0.25), plane(pos, vec3(0.0, 1.0, 0.0), 0.25))
}

pub fn get_normal(pos: Vec3) -> Vec3 {
    let e = vec2(0.0001, 0.0);
    return vec3(
        map(pos + e.xyy()) - map(pos - e.xyy()),
        map(pos + e.yxy()) - map(pos - e.yxy()),
        map(pos + e.yyx()) - map(pos - e.yyx()),
    )
    .normalize();
}
