use crate::color::Color;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::render_error::RenderError;
use crate::scene::Scene;
use crate::setup::{BLACK, WHITE};
use image::ImageBuffer;
use image::Rgba;
use rayon::prelude::*;

use rand::prelude::*;
//use std::path::Path;
use std::result::Result;

use crate::hitable::Hitable;

pub fn render_montecarlo(
    frame_width: u32,
    frame_height: u32,
    max_depth: usize,
    n_msaa: usize,
    _current_frame: u32,
    _tot_frames: u32,
    scene: &Scene,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, RenderError> {
    let mut imgbuf = image::ImageBuffer::new(frame_width, frame_height);
    let mut coords = Vec::with_capacity((frame_width * frame_height) as usize);
    for y in 0..frame_height {
        for x in 0..frame_width {
            coords.push((x, y));
        }
    }
    let pixels: Vec<(u32, u32, Color)> = coords
        .par_iter()
        .map(|&(x, y)| {
            let flipped_y = frame_height - y; // render from bottom up to avoid image needing to be flipped

            let mut col = BLACK;

            for _s in 0..n_msaa {
                let mut rng = rand::thread_rng();
                let u = ((x as f32) + rng.gen::<f32>()) / (frame_width as f32);
                let v = ((flipped_y as f32) + rng.gen::<f32>()) / (frame_height as f32);

                let ray = scene.camera.get_ray(u, v);
                col += color(&ray, &scene, max_depth, max_depth);
            }
            col /= n_msaa as f32;
            (x, y, col)
        })
        .collect();

    for (x, y, col) in pixels {
        //let rgb = to_colour(&col);
        imgbuf.put_pixel(x, y, col.to_rgba());
    }
    Ok(imgbuf)
}

// fn produce_error() -> Result<(), RenderError> {
//     Err(RenderError {
//         kind: String::from("wtf"),
//         message: String::from("Page not found"),
//     })
// }

// fn background_color(ray: &Ray, col: &mut Color) {
//     // background color, sky in this case.
//     let t = 0.5 * (ray.direction().normalize().y + 1.0);
//     let c = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
//     col.red = c.x;
//     col.green = c.y;
//     col.blue = c.z;
// }

fn color(ray: &Ray, scene: &Scene, max_depth: usize, depth: usize) -> Color {
    let mut col = WHITE;
    // exit if the max depth has been reached
    if depth <= 0 {
        return col;
    }
    let hit = scene.bvh.hit(ray, 0.001, f32::MAX);
    match hit {
        Some(hit_record) => {
            let scattered = hit_record.mat.scatter(ray, &hit_record);

            let emitted = hit_record
                .mat
                .emitted(hit_record.u, hit_record.v, hit_record.pos);
            match scattered {
                Some(scatter) => {
                    if let Some(bounce) = scatter.ray {
                        col = emitted + scatter.color * color(&bounce, scene, max_depth, depth - 1);
                    } else {
                        col = emitted;
                    }
                }
                None => col = emitted,
            }
        }
        None => {
            col  = scene.background;
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
