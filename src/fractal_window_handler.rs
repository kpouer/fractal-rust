use std::sync::mpsc::Receiver;
use std::time::Duration;

use eframe::{App, Frame};
use eframe::epaint::{ImageData, TextureHandle};
use egui::Context;

pub(crate) struct FractalWindowHandler {
    must_redraw: bool,
    image_data_receiver: Receiver<ImageData>,
    texture: Option<TextureHandle>,
}

impl From<Receiver<ImageData>> for FractalWindowHandler {
    fn from(image_data_receiver: Receiver<ImageData>) -> Self {
        Self {
            must_redraw: true,
            image_data_receiver,
            texture: None,
        }
    }
}

impl App for FractalWindowHandler {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if let Ok(image_data) = self.image_data_receiver.try_recv() {
            let texture_options = egui::TextureOptions::default();
            let texture = ctx.load_texture("Screen".to_string(),
                                           image_data,
                                           texture_options);
            self.texture = Some(texture);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = self.texture.as_ref() {
                ui.image((texture.id(), ui.available_size()));
            } else {
                ui.spinner();
            }
        });
        ctx.request_repaint_after(Duration::from_millis(1000 / 60));
    }
}
//
// impl WindowHandler for FractalWindowHandler
// {
//     fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: UVec2) {
//         self.canvas = Image::new(size_pixels.x, size_pixels.y);
//         self.must_redraw = true;
//         helper.request_redraw();
//     }
//
//     fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
//         if self.must_redraw {
//             self.must_redraw = false;
//             let start = std::time::Instant::now();
//             let compute_function = &self.compute_function;
//             compute_function(&self.params, &mut self.canvas);
//             println!("compute in : {:?}", start.elapsed());
//             let image = self.build_image(graphics);
//             self.image = Some(image);
//         }
//         if self.image.is_some() {
//             let image = self.image.as_ref().unwrap();
//             graphics.draw_image(Vec2::ZERO, image);
//         } else {
//             self.draw_pixels(graphics);
//         }
//         // Request that we draw another frame once this one has finished
//         helper.request_redraw();
//     }
//
//     fn on_mouse_move(&mut self, _: &mut WindowHelper<()>, position: Vec2) {
//         self.mouse_position = position;
//     }
//
//     fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
//         if button == MouseButton::Left {
//             self.params.set_center(self.mouse_position, self.canvas.width as f64, self.canvas.height as f64);
//             self.must_redraw = true;
//             helper.request_redraw();
//         }
//     }
//
//     fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, _: KeyScancode) {
//         match virtual_key_code {
//             None => {}
//             Some(VirtualKeyCode::W) => {
//                 self.params.decrease_iterations();
//                 self.must_redraw = true;
//                 helper.request_redraw();
//             }
//             Some(VirtualKeyCode::X) => {
//                 self.params.increase_iterations();
//                 self.must_redraw = true;
//                 helper.request_redraw();
//             }
//             Some(VirtualKeyCode::V) => {
//                 if self.params.support_zoom {
//                     self.params.zoom_in();
//                     self.must_redraw = true;
//                     helper.request_redraw();
//                 }
//             }
//             Some(VirtualKeyCode::C) => {
//                 if self.params.support_zoom {
//                     self.params.zoom_out();
//                     self.must_redraw = true;
//                     helper.request_redraw();
//                 }
//             }
//             _ => {}
//         }
//     }
// }
//
// impl FractalWindowHandler {
//     fn draw_pixels(&mut self, graphics: &mut Graphics2D) {
//         let start = std::time::Instant::now();
//         graphics.clear_screen(Color::WHITE);
//         let (width, height) = self.canvas.dimensions();
//         let mut color_args = ColorArgs::new(0, self.params.max_iterations);
//         let color_function = &self.color_model;
//         for y in 0..height {
//             for x in 0..width {
//                 let iterations = self.canvas.get_pixel_iterations(x, y);
//                 color_args.iterations = iterations;
//                 let color = color_function(&color_args);
//                 graphics.draw_line((x as f32, y as f32),
//                                    ((x + 1) as f32, (y + 1) as f32),
//                                    1.0,
//                                    color);
//             }
//         }
//         let elapsed = start.elapsed();
//         println!("elapsed: {:?}", elapsed);
//     }
// }