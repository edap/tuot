// TODO
// why SDF rendering in the default renderer has no shadows, che the comment in the renderer.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::tuot::color::Color;
use crate::tuot::render_error::RenderError;
use eframe::egui;
use egui::CollapsingHeader;
use egui::Color32;
use egui_extras::RetainedImage;
use glam::Vec3A;
use image::ImageBuffer;
use image::Rgba;
use tuot::camera::Camera;
use std::path::Path;
use std::time::Instant;
use tuot;
use tuot::camera_effects::camera_distorter_opt::CameraDistorterOpt;
use tuot::camera_effects::CameraEffects;
use tuot::examples::get_world_and_camera;
use tuot::examples::Worlds;
use tuot::montecarlo_pimped::deflection_opt::DeflectionForce;
use tuot::montecarlo_pimped::deflection_opt::DeflectionOpt;
use tuot::montecarlo_pimped::renderer::render_montecarlo_pimped;
use tuot::montecarlo_pimped::tint_opt::BandOp;
use tuot::montecarlo_pimped::tint_opt::TintOpt;
use tuot::renderer::render_montecarlo;
use tuot::utils::load_obj_to_hitable;
use tuot::scene::Scene;

#[derive(PartialEq)]
enum RendererEngine {
    MonteCarlo,
    MonteCarloPimped,
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 900.0)),
        ..Default::default()
    };

    //let options = eframe::NativeOptions::default();
    eframe::run_native(
        "T.U.O.T.",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    render_engine: RendererEngine,
    camera_effects: CameraEffects,
    worlds: Worlds,
    max_depth: usize,
    last_frame_rendered: Option<RetainedImage>,
    image_buffer: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    frame_width: u32,
    frame_height: u32,
    camera_fov: f32,
    camera_aperture: f32,
    msaa_samples: usize,
    n_current_frame: u32,
    tot_frames: u32,
    _renderings_folder: String,
    elapsed_time: String,
    camera_distorter_opt: CameraDistorterOpt,
    deflection_opt: DeflectionOpt,
    background_color: Color32,
    color_normal: Color32,
    tint_opt: TintOpt,
    picked_path: Option<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            render_engine: RendererEngine::MonteCarlo,
            camera_effects: CameraEffects::NoEffects,
            worlds: Worlds::SdfWall,
            last_frame_rendered: None,
            max_depth: 4, //
            image_buffer: None,
            frame_width: 640,
            frame_height: 400,
            camera_fov: 40.0,
            camera_aperture: 0.1,
            msaa_samples: 4,
            n_current_frame: 0,
            tot_frames: 25,
            _renderings_folder: "renderings".to_string(),
            elapsed_time: "0".to_string(),
            camera_distorter_opt: CameraDistorterOpt::default(),
            deflection_opt: DeflectionOpt::default(),
            color_normal: Color32::YELLOW,
            background_color: Color32::from_rgb(209, 193, 89),
            tint_opt: TintOpt::default(),
            picked_path: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("scene settings").show(ctx, |ui| {
            ui.heading("Scene");
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(_) = self.picked_path {
                if ui.button("Deselect File").clicked() {
                    self.picked_path = None;
                }
            } else {
                CollapsingHeader::new("Example Worlds")
                .default_open(true)
                .show(ui, |ui| {
                    ui.radio_value(&mut self.worlds, Worlds::CornellBox, "Cornell Box");
                    ui.radio_value(&mut self.worlds, Worlds::Random, "Random");
                    ui.radio_value(&mut self.worlds, Worlds::RandomGlass, "RandomGlass");
                    ui.radio_value(&mut self.worlds, Worlds::VerticalWall, "Vertical Wall");
                    ui.radio_value(&mut self.worlds, Worlds::ThreeSphere, "Three Spheres");
                    ui.radio_value(&mut self.worlds, Worlds::SdfSpheres, "Sdf Spheres");
                    ui.radio_value(&mut self.worlds, Worlds::SdfWall, "Sdf Wall");
                    ui.radio_value(&mut self.worlds, Worlds::SimpleAreaLight, "Area Light");
                    
                });
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("An imaginary renderer");
            if ui.button("Render").clicked() {
                let now = Instant::now();

                let render_buffer = render(self);
                match render_buffer {
                    Ok(render_buffer) => {
                        self.elapsed_time = format!(
                            "{}.{}",
                            now.elapsed().as_secs(),
                            now.elapsed().subsec_millis()
                        );

                        self.image_buffer = Some(render_buffer);
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            [self.frame_width as usize, self.frame_height as usize],
                            &self.image_buffer.as_ref().unwrap(),
                        );
                        let last_frame_rendered =
                            RetainedImage::from_color_image("0.png", color_image);

                        self.last_frame_rendered = Some(last_frame_rendered);
                        self.n_current_frame += 1;
                    }
                    Err(e) => println!("{:?}", e),
                };
            }
            if let Some(img_buffer) = &self.image_buffer {
                if ui.button("Save Frame").clicked() {
                    let filename = format!("{} {}", self.n_current_frame.to_string(), ".png");
                    img_buffer.save(&Path::new(&filename)).unwrap();
                }
            }
            if let Some(img) = &self.last_frame_rendered {
                img.show(ui);
            }
        });
        egui::SidePanel::right("renderer settings").show(ctx, |ui| {
            ui.heading("Settings");
            CollapsingHeader::new("Render Engines")
                .default_open(true)
                .show(ui, |ui| {
                    ui.radio_value(
                        &mut self.render_engine,
                        RendererEngine::MonteCarlo,
                        "MonteCarlo",
                    );
                    ui.radio_value(
                        &mut self.render_engine,
                        RendererEngine::MonteCarloPimped,
                        "T.U.O.T.",
                    );
                });

            CollapsingHeader::new("Resolution")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("width: ");
                    ui.add(egui::Slider::new(&mut self.frame_width, 10..=1920));
                    ui.label("height: ");
                    ui.add(egui::Slider::new(&mut self.frame_height, 10..=1920));
                });

            CollapsingHeader::new("RayTracer Options")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("max Depth ");
                    ui.add(egui::Slider::new(&mut self.max_depth, 1..=70));
                    ui.label("Antialiasing Samples: ");
                    ui.add(egui::Slider::new(&mut self.msaa_samples, 1..=150));
                });
            match self.render_engine {
                RendererEngine::MonteCarloPimped => {
                    CollapsingHeader::new("Camera Effects")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.radio_value(
                                &mut self.camera_effects,
                                CameraEffects::NoEffects,
                                "NoEffects",
                            );
                            ui.radio_value(
                                &mut self.camera_effects,
                                CameraEffects::Distorter,
                                "Distorter",
                            );
                        });
                    CollapsingHeader::new("Deflection Options")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label("Deflection Amplitude");
                            ui.add(egui::Slider::new(
                                &mut self.deflection_opt.amplitude,
                                0.0..=1.0,
                            ));
                            ui.label("Randomness");
                            ui.add(egui::Slider::new(
                                &mut self.deflection_opt.randomness,
                                0.0..=0.2,
                            ));
                            ui.label("Deflection Force");
                            ui.radio_value(
                                &mut self.deflection_opt.force,
                                DeflectionForce::CameraRay,
                                "Camera ray",
                            );
                            ui.radio_value(
                                &mut self.deflection_opt.force,
                                DeflectionForce::ObjectToCamera,
                                "Object to camera",
                            );
                        });
                    CollapsingHeader::new("Tint Options")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label("Background Color");
                            ui.color_edit_button_srgba(&mut self.background_color);
                            ui.label("Normal Color");
                            ui.color_edit_button_srgba(&mut self.color_normal);
                            ui.label("Mix with Normal Color");
                            ui.add(egui::Slider::new(&mut self.tint_opt.mix, 0.0..=1.0));
                            ui.label("Frequency");
                            ui.add(egui::Slider::new(&mut self.tint_opt.freq, 1.0..=120.0));
                            ui.label("Amplitude");
                            ui.add(egui::Slider::new(&mut self.tint_opt.amplitude, 1.0..=80.0));
                            ui.label("Banding Operation");
                            ui.radio_value(&mut self.tint_opt.band_op, BandOp::Sin, "sin");
                            ui.radio_value(&mut self.tint_opt.band_op, BandOp::Fract, "fract");
                            ui.radio_value(&mut self.tint_opt.band_op, BandOp::Mod, "modulo");
                        });
                }
                _ => {}
            }

            match self.camera_effects {
                CameraEffects::Distorter => {
                    CollapsingHeader::new("Camera Distorter Options")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label("Noise scale ");
                            ui.add(egui::Slider::new(
                                &mut self.camera_distorter_opt.noise_scale,
                                1.0..=36.0,
                            ));
                            ui.label("Amplitude");
                            ui.add(egui::Slider::new(
                                &mut self.camera_distorter_opt.amplitude,
                                0.0..=2.0,
                            ));
                            ui.label("Eaasing");
                            ui.add(egui::Slider::new(
                                &mut self.camera_distorter_opt.easing,
                                0.0..=1.0,
                            ));
                        });
                }
                _ => {}
            }

            CollapsingHeader::new("Camera Options")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("fov: ");
                    ui.add(egui::Slider::new(&mut self.camera_fov, 1.0..=90.0));
                    ui.label("aperture: ");
                    ui.add(egui::Slider::new(&mut self.camera_aperture, 0.1..=2.0));
                });

            ui.horizontal(|_ui| {});
            ui.separator();
        });
        egui::TopBottomPanel::bottom("timeline").show(ctx, |ui| {
            ui.heading("timeline");
            ui.add(
                egui::Slider::new(&mut self.n_current_frame, 0..=self.tot_frames)
                    .text("current_frame"),
            );
            ui.label(format!("Rendered in: {} seconds", self.elapsed_time));
        });
    }
}

fn render(a: &mut MyApp) ->  Result<ImageBuffer<Rgba<u8>, Vec<u8>>, RenderError> {
    let scene;
    let mut world;
    let camera;
    let background: Color;
    if let Some(path) = &a.picked_path {
        // TODO, camera should be set depending on the dimension of the object
        // TODO, load gltf
        let look_at = Vec3A::new(0.0, 0.0, -1.0);
        let look_from = Vec3A::new(1.1, 0.9, 1.0);
        let camera = Camera::new(
            look_from,
            look_at,
            a.camera_fov,
            (a.frame_width as f32) / (a.frame_height as f32),
            a.camera_aperture,
        );
        world = load_obj_to_hitable(&Path::new(path))?;
        scene = Scene::new(&mut world, camera, Color::new(0.0, 0.0, 1.0));
    } else {
        (world, camera, background) = get_world_and_camera(
            &a.worlds,
            a.camera_fov,
            a.frame_width,
            a.frame_height,
            a.camera_aperture,
        );
        scene = Scene::new(&mut world, camera, background);

    }
    match a.render_engine {
        RendererEngine::MonteCarlo => {
            return render_montecarlo(
                a.frame_width,
                a.frame_height,
                a.max_depth,
                a.msaa_samples,
                a.n_current_frame,
                a.tot_frames,
                //&camera, // in the future, it could be a pool of cameras.
                &scene,
            );
        }
        RendererEngine::MonteCarloPimped => {
            a.tint_opt.normal_color = Color::from_array(a.color_normal.to_array());
            a.tint_opt.background_color = Color::from_array(a.background_color.to_array());
            return render_montecarlo_pimped(
                a.frame_width,
                a.frame_height,
                a.max_depth,
                a.msaa_samples,
                a.n_current_frame,
                a.tot_frames,
                &a.camera_effects,
                &a.camera_distorter_opt,
                &a.deflection_opt,
                &a.tint_opt,
                &scene,
            );
        }
        
    }
}

//fn get_save_frame_path() {
// let frame = current_frame.to_string();
// // an io::Error is called if the dir can not be created
// use std::fs::DirBuilder;
// DirBuilder::new()
//     .recursive(true)
//     .create(renderings_folder)?;

// //let filename = renderings_folder.join(format!("{}_color.png", frame));
// let filename = format!("{}/{}.png", renderings_folder, frame);

// // an ImageError is called if the image can not be saved

// return filename;
//}
