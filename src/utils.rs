use std::{path::Path, sync::Arc};

use glam::Vec3A;
use rand::Rng;
use tobj::load_obj;
use tobj::Material as TobjMaterial;
use crate::color::Color;
use crate::material::*;

use crate::hitable::{HitableStore, Hitable};
use crate::render_error::RenderError;
use crate::texture::Texture;
use crate::triangle::Triangle;

pub fn random_vec3(min: f32, max: f32) -> Vec3A {
    let mut rng = rand::thread_rng();
    Vec3A::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_sphere() -> Vec3A {
    loop {
        let p = random_vec3(-1.0, 1.0);
        // maybe the rng var can be passed as &rng? how much does it cost to create it new every time?
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn near_zero(vec: &Vec3A) -> bool {
    vec.x.abs() < f32::EPSILON && vec.y.abs() < f32::EPSILON && vec.z.abs() < f32::EPSILON
}

pub fn u_v_from_sphere_hit_point(hit_point_on_sphere: Vec3A) -> (f32, f32) {
    let n = hit_point_on_sphere.normalize();
    let x = n.x;
    let y = n.y;
    let z = n.z;
    let u = (x.atan2(z) / (2.0 * std::f32::consts::PI)) + 0.5;
    let v = y * 0.5 + 0.5;
    (u, v)
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    // schlick
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn refract(v: Vec3A, n: Vec3A, ni_over_nt: f32) -> Option<Vec3A> {
    let uv = v.normalize();

    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    } else {
        None
    }
}

pub fn reflect(v: &Vec3A, n: &Vec3A) -> Vec3A {
    *v - *n * (2.0 * v.dot(*n))
}

pub fn obj_to_hitable(path: &Path) -> Result<HitableStore, RenderError> {
    let mut hitables = HitableStore::new();

    let obj_file = tobj::load_obj(&path, &tobj::LoadOptions::default());

    match obj_file {
      Ok((models, mtls)) => {
        let mtls = mtls.expect("Failed to load MTL file");
  
        let default_mat: Arc<Material> = Arc::new(Material::lambertian(Texture::constant_color(Color::new(0.6f32, 0.6f32, 0.6f32))));
        let materials: Vec<Arc<Material>> = mtls.iter().map(|m: &TobjMaterial| {
          let mat: Arc<Material> = match m.illumination_model {
            Some(7) => Arc::new(Material::dielectric(m.optical_density.unwrap())),
            Some(5) => Arc::new(Material::metal(Texture::constant_color(Color::new(m.diffuse.unwrap()[0], m.diffuse.unwrap()[1], m.diffuse.unwrap()[2])), 1.0f32 / m.shininess.unwrap() as f32)),
            //Some(5) => Arc::new(Material::metal { albedo: Vec3A::new(m.diffuse[0], m.diffuse[1], m.diffuse[2]), fuzz: 1. / m.shininess }),
            _ => Arc::new(Material::lambertian(Texture::constant_color(Color::new(m.diffuse.unwrap()[0], m.diffuse.unwrap()[1], m.diffuse.unwrap()[2]))))
          };
      
          mat
        }).collect();
      
        for m in models.iter() {
          let mesh = &m.mesh;
          for f in 0..mesh.indices.len() / 3 {
            let i0 = mesh.indices[3 * f] as usize;
            let i1 = mesh.indices[3 * f + 1] as usize;
            let i2 = mesh.indices[3 * f + 2] as usize;
            let v0 = Vec3A::new(mesh.positions[i0 * 3], mesh.positions[i0 * 3 + 1], mesh.positions[i0 * 3 + 2]);
            let v1 = Vec3A::new(mesh.positions[i1 * 3], mesh.positions[i1 * 3 + 1], mesh.positions[i1 * 3 + 2]);
            let v2 = Vec3A::new(mesh.positions[i2 * 3], mesh.positions[i2 * 3 + 1], mesh.positions[i2 * 3 + 2]);
      
            let mat: Arc<Material> = match mesh.material_id {
              Some(id) => Arc::clone(&materials[id]),
              None => Arc::clone(&default_mat)
            };
      
            let tri: Triangle;
            if mesh.normals.len() > 0 {
              let normal = Vec3A::new(mesh.normals[i0 * 3], mesh.normals[i0 * 3 + 1], mesh.normals[i0 * 3 + 2]);
              tri = Triangle::new_with_normal(v0, v1, v2, normal, mat)
            } else {
              tri = Triangle::new(v0, v1, v2, Arc::clone(&mat));
            }
      
            hitables.push(tri);
            
          }
        }
        Ok(hitables)
      },
      Err(err) => Err(err.into()),
    } 
  }
