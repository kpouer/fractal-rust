use speedy2d::Window;
use crate::fractal::fractal_type::FractalType;

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
    let fractal_type = FractalType::Mandelbrot;
    let window_handler = match fractal_type {
        FractalType::Mandelbrot => mandelbrot_window_handler::MandelbrotWindowHandler::new(),
        FractalType::Buddahbrot => unimplemented!(),
    };

    window.run_loop(window_handler);
}
