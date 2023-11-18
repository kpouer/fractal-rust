use speedy2d::Window;

use crate::constants::DEFAULT_FRACTAL;
use crate::fractal_window_handler::FractalWindowHandler;

mod fractal;
mod color_model;
mod constants;
mod fractal_window_handler;

const INITIAL_WIDTH: u32 = 1024;
const INITIAL_HEIGHT: u32 = 768;

fn main()
{
    let window = Window::new_centered("Fractal-rust", (INITIAL_WIDTH, INITIAL_HEIGHT)).unwrap();
    window.run_loop(FractalWindowHandler::new(&DEFAULT_FRACTAL, INITIAL_WIDTH, INITIAL_WIDTH));
}
