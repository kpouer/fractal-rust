use crate::constants::DEFAULT_FRACTAL;
use crate::fractal::params::Params;
use crate::fractal_renderer::FractalRenderer;
use crate::fractal_window_handler::FractalWindowHandler;
use eframe::epaint::ImageData;
use egui::style::Interaction;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod color;
mod color_model;
mod constants;
mod fractal;
mod fractal_renderer;
mod fractal_window_handler;
mod interaction;
mod point;

const INITIAL_WIDTH: u16 = 1024;
const INITIAL_HEIGHT: u16 = 768;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([INITIAL_WIDTH as f32, INITIAL_HEIGHT as f32])
            .with_min_inner_size([INITIAL_WIDTH as f32, INITIAL_HEIGHT as f32]),
        ..Default::default()
    };
    let (image_data_sender, image_data_receiver) = channel::<ImageData>();
    let (interaction_sender, interaction_receiver) = channel::<Interaction>();
    thread::spawn(move || run_loop(image_data_sender, interaction_receiver));
    eframe::run_native(
        "Fractal",
        native_options,
        Box::new(|_cc| {
            Ok(Box::new(FractalWindowHandler::new(
                image_data_receiver,
                interaction_sender,
            )))
        }),
    )
    .unwrap();
}

fn run_loop(image_data_sender: Sender<ImageData>, interaction_receiver: Receiver<Interaction>) {
    const DEFAULT_SLEEP: Duration = Duration::from_millis(100);
    let params = Params::from(DEFAULT_FRACTAL);
    let mut fractal_renderer = FractalRenderer::new(INITIAL_WIDTH, INITIAL_HEIGHT, params);
    loop {
        let start = std::time::Instant::now();
        let image = fractal_renderer.compute_and_build_image();
        image_data_sender.send(image).unwrap();
        let elapsed = start.elapsed();
        let sleeping = DEFAULT_SLEEP.saturating_sub(elapsed);
        if sleeping > Duration::ZERO {
            sleep(sleeping);
        }
    }
}
