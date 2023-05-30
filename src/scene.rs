use crate::bvh::BvhTree;
use crate::camera::Camera;
use crate::color::{Color, self};
use crate::hitable::HitableStore;
use crate::sphere::Sphere;

// use crate::color::Color;
// use crate::disc::Disc;
// use crate::material::Material;
// use crate::sphere::Sphere;
// use crate::texture::Texture;
// use glam::Vec3A;

pub struct Scene<'a> {
    pub camera: Camera,
    pub bvh: BvhTree<'a>,
    pub background: Color,

}

impl<'a> Scene<'a> {
    pub fn new(models: &'a mut HitableStore, camera: Camera, background:Color) -> Scene<'a> {
        Scene {
            camera,
            bvh: BvhTree::new(models),
            background,
            //lights: vec![],
        }
    }

    // pub fn add_spherical_light(&mut self, sphere: Sphere) {
    //     self.lights.push(sphere);
    // }

    // TODO, allow to create a scene
    // pub fn cornell_box_world(width: f32, height: f32) -> Scene<'a> {
    //     let red = Color::new(0.75, 0.25, 0.25);
    //     let white = Color::new(0.75, 0.75, 0.75);
    //     let blue = Color::new(0.25, 0.25, 0.75);
    //     let light = Color::new(1.0, 1.0, 1.0) * 15.0;
    //     // light
    //     let disc_light = Disc {
    //         position: Vec3A::new(0.0, 10.0, -5.0),
    //         radius: 1.5,
    //         normal: Vec3A::new(0.0, -1.0, 0.0),
    //         mat: Material::diffuse_light(Texture::constant_color(light)),
    //     };
    //     let spheres: Vec<Sphere> = vec![
    //         // right wall
    //         Sphere {
    //             position: Vec3A::new(5006.0, 0.0, 0.0),
    //             radius: 5000.0,
    //             mat: Material::lambertian(Texture::constant_color(blue)),
    //         },
    //         // left wall
    //         Sphere {
    //             position: Vec3A::new(-5006.0, 0.0, 0.0),
    //             radius: 5000.0,
    //             mat: Material::lambertian(Texture::constant_color(red)),
    //         },
    //         // ceiling
    //         Sphere {
    //             position: Vec3A::new(0.0, 5010.0, 0.0),
    //             radius: 5000.0,
    //             mat: Material::lambertian(Texture::constant_color(white)),
    //         },
    //         // floor
    //         Sphere {
    //             position: Vec3A::new(0.0, -5000.0, 0.0),
    //             radius: 5000.0,
    //             mat: Material::lambertian(Texture::constant_color(white)),
    //         },
    //         // back wall
    //         Sphere {
    //             position: Vec3A::new(0.0, 0.0, -5010.0),
    //             radius: 5000.0,
    //             mat: Material::lambertian(Texture::constant_color(white)),
    //         },
    //         Sphere {
    //             position: Vec3A::new(-3.5, 2.0, -3.0),
    //             radius: 2.0,
    //             mat: Material::dielectric(1.52),
    //         },
    //         Sphere {
    //             position: Vec3A::new(3.5, 2.0, -7.0),
    //             radius: 2.0,
    //             mat: Material::metal(Texture::constant_color(Color::new(0.05, 1.0, 0.05)), 0.25),
    //         },
    //         Sphere {
    //             position: Vec3A::new(5.0, 1.0, 0.0),
    //             radius: 1.0,
    //             mat: Material::metal(Texture::constant_color(Color::new(1.0, 0.05, 0.05)), 0.0),
    //         },
    //     ];

    //     let mut hitables = HitableStore::new();
    //     for s in spheres {
    //         hitables.push(s);
    //     }
    //     hitables.push(disc_light);

    //     let look_from = Vec3A::new(0.0, 5.0, 15.0);
    //     let look_at = Vec3A::new(0.0, 5.0, 0.0);
    //     let aspect_ratio = f32::from(width) / f32::from(height);
    //     let dist_to_focus = look_from.distance(look_at);
    //     let up = Vec3A::new(0.0, 1.0, 0.0);
    //     Scene {
    //         camera: Camera::new(
    //             look_from,
    //             look_at,
    //             up,
    //             45.0,
    //             aspect_ratio,
    //             0.0,
    //             dist_to_focus,
    //         ),
    //         bvh: BvhTree::new(hitables),
    //     }
    // }
}
