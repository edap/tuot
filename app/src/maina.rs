use eframe::egui;
use egui_extras::RetainedImage;
use image::ImageBuffer;
use image::Rgba;
use std::path::Path;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 900.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    image: Option<RetainedImage>,
    buffer: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    frame_n: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image: None,
            buffer: None,
            frame_n: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is an image:");
            if let Some(img) = &self.image {
                img.show(ui);
                if ui.button("save").clicked() {
                    if let Some(buf) = &self.buffer {
                        println!("img saved!");
                        //let x = String::from(self.frame_n);
                        let filename = format!("{} {}", self.frame_n.to_string(), ".png");
                        buf.save(&Path::new(&filename)).unwrap();
                        println!("img {}", filename);
                    }
                };
            }
            if ui.button("Render").clicked() {
                let w = 200;
                let h = 100;
                self.buffer = Some(image::ImageBuffer::new(w, h));
                for (x, y, pixels) in self.buffer.as_mut().unwrap().enumerate_pixels_mut() {
                    let a = 1.0_f32;
                    *pixels = image::Rgba([
                        (x as f32 / w as f32 * 255.0_f32) as u8,
                        (y as f32 / h as f32 * 255.0_f32) as u8,
                        0.2_f32 as u8,
                        (a * 255.0).min(255.0).max(0.0) as u8,
                    ]);
                }
                self.frame_n += 1;

                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [w as usize, h as usize],
                    &self.buffer.as_ref().unwrap(),
                );
                let render_result = RetainedImage::from_color_image("0.png", color_image);
                self.image = Some(render_result);
            }
        });
    }
}
