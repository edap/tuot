use std::ops::{Add, Neg, Sub};

use super::deflection_opt::{DeflectionForce, DeflectionOpt};
use super::tint_opt::{ BandOp, TintOpt};
use glam::Vec3A;
use rand::Rng;

use crate::{
    camera::Camera,
    color::Color,
    //color::Color,
    hitable::HitRecord,
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal, Scatter},
    ray::Ray,
    setup::WHITE,
    texture::Texture,
    utils::{near_zero, random_in_unit_sphere, reflect, refract, schlick},
};

pub trait DeflectableNormal {
    fn scattero(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        camera: &Camera,
        deflection_opt: &DeflectionOpt,
        tint_opt: &TintOpt,
    ) -> Option<Scatter>;
    //fn default_color(&self) -> Texture;
}

impl DeflectableNormal for Material {
    fn scattero(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        camera: &Camera,
        deflection_opt: &DeflectionOpt,
        tint_opt: &TintOpt,
    ) -> Option<Scatter> {
        match self {
            Material::Lambertian(l) => {
                l.scattero(ray, hit_record, camera, deflection_opt, tint_opt)
            }
            Material::Metal(m) => m.scattero(ray, hit_record, camera, deflection_opt, tint_opt),
            Material::Dielectric(_d) => {
                _d.scattero(ray, hit_record, camera, deflection_opt, tint_opt)
            }
            Material::DiffuseLight(li) => {
                li.scattero(ray, hit_record, camera, deflection_opt, tint_opt)
            }
        }
    }
}

fn get_deflected_color(
    scatter_direction: &Vec3A,
    original_color: &Color,
    tint_opt: &TintOpt,
) -> Color {
    let x = scatter_direction.x;
    let y = scatter_direction.y;
    let z = scatter_direction.z;

    let new_color: Color;
    match tint_opt.band_op {
        BandOp::Sin => {
            new_color = Color {
                red: ((x * tint_opt.freq).sin() / 2.0) + 0.5,
                blue: ((y * tint_opt.freq).cos() / 2.0) + 0.5,
                green: ((z * tint_opt.freq).sin() / 2.0) + 0.5,
            };
        }
        BandOp::Fract => {
            new_color = Color {
                red: (x * tint_opt.freq).fract().abs(),
                blue: (y * tint_opt.freq).fract().abs(),
                green: (z * tint_opt.freq).fract().abs(),
            };
        }
        BandOp::Mod => {
            new_color = Color {
                red: ((x * tint_opt.freq) % tint_opt.amplitude).abs() / tint_opt.amplitude,
                blue: ((y * tint_opt.freq) % tint_opt.amplitude).abs() / tint_opt.amplitude,
                green: ((z * tint_opt.freq) % tint_opt.amplitude).abs() / tint_opt.amplitude,
            };
        }
    }

    return original_color.lerp(new_color * tint_opt.normal_color, tint_opt.mix);
}

impl DeflectableNormal for Lambertian {
    fn scattero(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        camera: &Camera,
        deflection_opt: &DeflectionOpt,
        tint_opt: &TintOpt,
    ) -> Option<Scatter> {
        //let mut scatter_direction = hit_record.normal + random_in_unit_sphere();
        let mut scatter_direction = deflect_normal(
            &hit_record.pos,
            &hit_record.normal,
            &camera.origin,
            deflection_opt,
        );
        // this is because some of the scattered rays hit the object they are reflecting.
        // This is because they start a bit below the hitten surface. In this case, we coerce
        // their direction to the the surface direction
        if near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        let target = hit_record.pos + scatter_direction;
        let original_color = self
            .albedo
            .value(hit_record.u, hit_record.v, hit_record.pos);
        let new_color = get_deflected_color(&scatter_direction, &original_color, &tint_opt);

        Some(Scatter {
            color: new_color,
            ray: Some(Ray::new(hit_record.pos, target - hit_record.pos)),
        })
    }
}

impl DeflectableNormal for Metal {
    fn scattero(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        _camera: &Camera,
        _deflection_opt: &DeflectionOpt,
        tint_opt: &TintOpt,
    ) -> Option<Scatter> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let scattered = Ray::new(
            hit_record.pos,
            reflected + random_in_unit_sphere() * self.fuzz,
        );
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, hit_record.pos);

        let new_color = get_deflected_color(&scattered.direction(), &attenuation, tint_opt);

        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some(Scatter {
                color: new_color,
                ray: Some(scattered),
            })
        } else {
            None
        }
    }
}

impl DeflectableNormal for Dielectric {
    fn scattero(
        &self,
        r_in: &Ray,
        hit: &HitRecord,
        _camera: &Camera,
        _deflection_opt: &DeflectionOpt,
        tint_opt: &TintOpt,
    ) -> Option<Scatter> {
        let mut rng = rand::thread_rng();
        let outward_normal: Vec3A;
        let ni_over_nt: f32;
        let cosine: f32;
        let attenuation = Texture::constant_color(WHITE);

        if r_in.direction.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.index_of_refraction;
            cosine =
                self.index_of_refraction * r_in.direction.dot(hit.normal) / r_in.direction.length();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.index_of_refraction;
            cosine = -r_in.direction.dot(hit.normal) / r_in.direction.length();
        }

        match refract(r_in.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                if rng.gen::<f32>() > schlick(cosine, self.index_of_refraction) {
                    let scattered = Ray::new(hit.pos, refracted.normalize());
                    let old_color = attenuation.value(hit.u, hit.v, hit.pos);
                    let new_color =
                        get_deflected_color(&scattered.direction(), &old_color, tint_opt);

                    return Some(Scatter {
                        ray: Some(scattered),
                        //color: attenuation.value(hit.u, hit.v, hit.pos),
                        color: new_color,
                    });
                }
            }
            None => {}
        }

        Some(Scatter {
            color: attenuation.value(hit.u, hit.v, hit.pos),
            ray: Some(Ray::new(
                hit.pos,
                reflect(&r_in.direction.normalize(), &hit.normal),
            )),
        })
    }
}

impl DeflectableNormal for DiffuseLight {
    fn scattero(
        &self,
        _r_in: &Ray,
        _hit: &HitRecord,
        _camera: &Camera,
        _deflection_opt: &DeflectionOpt,
        _tint_opt: &TintOpt,
    ) -> Option<Scatter> {
        None
    }
}

// fn get_scatter_direction() -> Vec3A {}

// the more the normal of a surface is aligned with the camera ray, the more it is pushed away
// from the camera.
// the more is close to the camera ray, the more it is similar to the camera ray.
// to check 2 vectors you use the dot product.
// to push away you use the add method.
pub fn deflect_normal(
    surface_position: &Vec3A,
    surface_normal: &Vec3A,
    camera_pos: &Vec3A,
    deflection_opt: &DeflectionOpt,
) -> Vec3A {
    let surface_to_eye = camera_pos.sub(*surface_position);
    let camera_ray = surface_to_eye.neg();
    let random_ray = random_in_unit_sphere();

    let pushed_away;
    match deflection_opt.force {
        DeflectionForce::CameraRay => {
            pushed_away = surface_normal.add(camera_ray);
        }
        DeflectionForce::ObjectToCamera => {
            pushed_away = surface_normal.add(surface_to_eye);
        }
    }

    // piu' sono allineati, piu' vuoi che il normal della superficie
    // coincida con quello della camera.
    // push p a bit far away from the eye. the more it is closer, the more you push it away.
    // if deflection_opt.amplitude is 1, lerped is equal to pushed away.
    let lerped = surface_normal
        .lerp(pushed_away, deflection_opt.amplitude)
        .normalize();

    return (lerped + random_ray * deflection_opt.randomness).normalize();
}
