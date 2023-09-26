use crate::camera::Camera;
use crate::camera_effects::camera_distorter::get_distorted_ray;
use crate::camera_effects::camera_distorter_opt::CameraDistorterOpt;
use crate::camera_effects::CameraEffects;
use crate::color::Color;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::render_error::RenderError;
use crate::scene::Scene;
use crate::setup::{BLACK, WHITE};
use crate::texture::Texture;
use glam::Vec3;
use image::ImageBuffer;
use image::Rgba;
use rayon::prelude::*;

use rand::prelude::*;
//use std::path::Path;
use std::result::Result;

use crate::hitable::Hitable;

use super::deflection_opt::DeflectionOpt;
use super::material::DeflectableNormal;
use super::tint_opt::TintOpt;

pub fn render_montecarlo_pimped(
    frame_width: u32,
    frame_height: u32,
    max_depth: usize,
    n_msaa: usize,
    _current_frame: u32,
    _tot_frames: u32,
    camera_effects: &CameraEffects,
    camera_distorter_opt: &CameraDistorterOpt,
    deflection_opt: &DeflectionOpt,
    tint_opt: &TintOpt,
    scene: &Scene,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, RenderError> {
    let mut imgbuf = image::ImageBuffer::new(frame_width, frame_height);
    let mut coords = Vec::with_capacity((frame_width * frame_height) as usize);
    for y in 0..frame_height {
        for x in 0..frame_width {
            coords.push((x, y));
        }
    }

    let noise = Texture::noise(camera_distorter_opt.noise_scale);
    let pixels: Vec<(u32, u32, Color)> = coords
        .par_iter()
        .map(|&(x, y)| {
            let flipped_y = frame_height - y; // render from bottom up to avoid image needing to be flipped

            let mut col = BLACK;

            for _s in 0..n_msaa {
                let mut rng = rand::thread_rng();
                let u = ((x as f32) + rng.gen::<f32>()) / (frame_width as f32);
                let v = ((flipped_y as f32) + rng.gen::<f32>()) / (frame_height as f32);

                let ray = get_camera_ray(
                    &scene.camera,
                    u,
                    v,
                    &camera_effects,
                    camera_distorter_opt,
                    &noise,
                );
                // col += Color {
                //     red: (ray.direction().x.powf(3.0) * 40.).sin(),
                //     blue: (ray.direction().y.powf(3.0 * 40.)).cos(),
                //     green: 0.0,
                // };
                col += colora(
                    &ray,
                    &scene,
                    &deflection_opt,
                    &tint_opt,
                    max_depth,
                    max_depth,
                );
            }
            col /= n_msaa as f32;
            (x, y, col)
        })
        .collect();

    for (x, y, col) in pixels {
        imgbuf.put_pixel(x, y, col.to_rgba());
    }
    println!("{}", imgbuf.width());
    Ok(imgbuf)
}

// fn produce_error() -> Result<(), RenderError> {
//     Err(RenderError {
//         kind: String::from("wtf"),
//         message: String::from("Page not found"),
//     })
// }

fn get_camera_ray(
    camera: &Camera,
    u: f32,
    v: f32,
    camera_effects: &CameraEffects,
    camera_distorter_opt: &CameraDistorterOpt,
    noise: &Texture,
) -> Ray {
    match camera_effects {
        CameraEffects::NoEffects => camera.get_ray(u, v),
        CameraEffects::Distorter => get_distorted_ray(
            camera,
            u,
            v,
            noise,
            camera_distorter_opt.amplitude,
            camera_distorter_opt.easing,
        ),
    }
}

fn background_color(ray: &Ray, col: &mut Color, background: &Color) {
    let t = 0.5 * (ray.direction().normalize().y + 1.0);
    // let c = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)
    //     + Vec3::new(background.red, background.green, background.blue) * t;
    let c = Vec3::new(background.red, background.green, background.blue);
    col.red = c.x * t;
    col.green = c.y * (1.0 - t);
    col.blue = c.z;
}

fn colora(
    ray: &Ray,
    scene: &Scene,
    deflection_opt: &DeflectionOpt,
    tint_opt: &TintOpt,
    max_depth: usize,
    depth: usize,
) -> Color {
    let mut col = WHITE;
    // exit if the max depth has been reached
    if depth <= 0 {
        return col;
    }
    let hit = scene.bvh.hit(ray, 0.001, f32::MAX);
    match hit {
        Some(hit_record) => {
            let scattered =
                hit_record
                    .mat
                    .scattero(ray, &hit_record, &scene.camera, deflection_opt, tint_opt);

            let emitted = hit_record.mat.emitted(0.0, 0.0, hit_record.pos);
            match scattered {
                Some(scatter) => {
                    if let Some(bounce) = scatter.ray {
                        col = emitted
                            + scatter.color
                                * colora(
                                    &bounce,
                                    scene,
                                    deflection_opt,
                                    tint_opt,
                                    max_depth,
                                    depth - 1,
                                );
                    } else {
                        col = emitted;
                    }
                }
                None => col = emitted,
            }
        }
        None => {
            // background color, sky in this case.
            background_color(ray, &mut col, &tint_opt.background_color);
            //col = BLACK;
        }
    }
    col.clamp()
}

// #[test]
// fn test_can_render_scene() {
//     let scene = Scene {
//         width: 800,
//         height: 600,
//         fov: 90.0,
//         sphere: Sphere {
//             center: Vector3::new(0.0, 0.0, -5.0),
//             radius: 1.0,
//             color: Color {
//                 red: 0.4,
//                 green: 1.0,
//                 blue: 0.4,
//             },
//         },
//     };

//     let img: DynamicImage = render(&scene);
//     assert_eq!(scene.width, img.width());
//     assert_eq!(scene.height, img.height());
// }
