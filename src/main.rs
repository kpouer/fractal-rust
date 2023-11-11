use speedy2d::Window;
use crate::buddahbrot_window_handler::BuddahbrotWindowHandler;
use crate::fractal::fractal_type::FractalType;
use crate::mandelbrot_window_handler::MandelbrotWindowHandler;

mod fractal;
mod color_model;
mod constants;
mod mandelbrot_window_handler;
mod buddahbrot_window_handler;

const INITIAL_WIDTH: u32 = 1024;
const INITIAL_HEIGHT: u32 = 768;

fn main()
{
    let window = Window::new_centered("Fractal-rust", (INITIAL_WIDTH, INITIAL_HEIGHT)).unwrap();
    let fractal_type = FractalType::Buddahbrot;
    match fractal_type {
        FractalType::Mandelbrot => { window.run_loop(MandelbrotWindowHandler::new()); }
        FractalType::Buddahbrot => { window.run_loop(BuddahbrotWindowHandler::new()); }
    };
}
