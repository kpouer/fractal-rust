use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::sleep;
use egui::ImageData;
use crate::constants::DEFAULT_FRACTAL;
use crate::fractal::params::Params;
use crate::fractal_renderer::FractalRenderer;
use crate::fractal_window_handler::FractalWindowHandler;

mod fractal;
mod color_model;
mod constants;
mod fractal_window_handler;
mod fractal_renderer;
mod point;
mod color;

const INITIAL_WIDTH: u16 = 1024;
const INITIAL_HEIGHT: u16 = 768;

fn main()
{
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([INITIAL_WIDTH as f32, INITIAL_HEIGHT as f32])
            .with_min_inner_size([INITIAL_WIDTH as f32, INITIAL_HEIGHT as f32])
        ,
        ..Default::default()
    };
    let (image_data_sender, image_data_receiver) = channel::<ImageData>();
    thread::spawn(move|| run_loop(image_data_sender));
    eframe::run_native(
        "Fractal",
        native_options,
        Box::new(|cc| Box::new(FractalWindowHandler::from(image_data_receiver))),
    ).unwrap();
}

fn run_loop(image_data_sender: Sender<ImageData>) {
    let params = Params::from(DEFAULT_FRACTAL);
    let mut fractal_renderer = FractalRenderer::new(INITIAL_WIDTH, INITIAL_HEIGHT, params);
    loop {
        let image = fractal_renderer.build_image();
        image_data_sender.send(image).unwrap();
        sleep(std::time::Duration::from_millis(100));
    }
}
