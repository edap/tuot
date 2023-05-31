use crate::color::Color;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::setup::{BLACK, WHITE};
use crate::texture::Texture;
use crate::utils::{near_zero, random_in_unit_sphere, reflect, refract, schlick};
use glam::Vec3A;
use rand::Rng;

pub struct Scatter {
    pub color: Color,
    pub ray: Option<Ray>,
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
    fn emitted(&self, _u: f32, _v: f32, _p: Vec3A) -> Color {
        BLACK
    }
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    // Texture(Texture),
    DiffuseLight(DiffuseLight),
}

impl Material {
    pub fn lambertian(albedo: Texture) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }

    pub fn metal(albedo: Texture, fuzz: f32) -> Material {
        Material::Metal(Metal { albedo, fuzz })
    }

    pub fn dielectric(index_of_refraction: f32) -> Material {
        Material::Dielectric(Dielectric {
            index_of_refraction,
        })
    }

    pub fn diffuse_light(albedo: Texture) -> Material {
        Material::DiffuseLight(DiffuseLight { albedo })
    }
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
            Material::Dielectric(d) => d.scatter(ray, hit_record),
            Material::DiffuseLight(l) => l.scatter(ray, hit_record),
        }
    }
    fn emitted(&self, u: f32, v: f32, p: Vec3A) -> Color {
        match self {
            // just light has an emission value
            Material::DiffuseLight(dl) => dl.emitted(u, v, p),
            //_ => Color::new(0.0, 0.0, 0.0),
            _ => BLACK,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    //#[serde_as(as = "SrgbAsArray")]
    pub albedo: Texture,
}

impl Lambertian {
    pub fn new(albedo: Texture) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hit_record.normal + random_in_unit_sphere();
        // this is because some of the scattered rays hit the object they are reflecting.
        // This is because they start a bit below the hitten surface. In this case, we coerce
        // their direction to the the surface direction
        if near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }
        let target = hit_record.pos + scatter_direction;
        Some(Scatter {
            color: self
                .albedo
                .value(hit_record.u, hit_record.v, hit_record.pos),
            ray: Some(Ray::new(hit_record.pos, target - hit_record.pos)),
        })
    }
    fn emitted(&self, _u: f32, _v: f32, _p: Vec3A) -> Color {
        BLACK
    }
}

//#[serde_with::serde_as]
//#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[derive(Debug, Clone)]
pub struct Metal {
    //#[serde_as(as = "SrgbAsArray")]
    pub albedo: Texture,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Texture, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let scattered = Ray::new(
            hit_record.pos,
            reflected + random_in_unit_sphere() * self.fuzz,
        );
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, hit_record.pos);
        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some(Scatter {
                color: attenuation,
                ray: Some(scattered),
            })
        } else {
            None
        }
    }
    fn emitted(&self, _u: f32, _v: f32, _p: Vec3A) -> Color {
        BLACK
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<Scatter> {
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
                    return Some(Scatter {
                        ray: Some(Ray::new(hit.pos, refracted.normalize())),
                        color: attenuation.value(hit.u, hit.v, hit.pos),
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

    fn emitted(&self, _u: f32, _v: f32, _p: Vec3A) -> Color {
        BLACK
    }
}

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    pub albedo: Texture,
}

impl DiffuseLight {
    pub fn new(albedo: Texture) -> DiffuseLight {
        DiffuseLight { albedo }
    }
}

impl Scatterable for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _hit: &HitRecord) -> Option<Scatter> {
        None
    }
    fn emitted(&self, u: f32, v: f32, p: Vec3A) -> Color {
        self.albedo.value(u, v, p)
    }
}
