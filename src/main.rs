mod mandelbrot;
mod color_model;
mod constants;

use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use speedy2d::dimen::UVec2;
use crate::color_model::black_white::BlackWhite;
use crate::color_model::ColorModel;
use crate::mandelbrot::Mandelbrot;

const INITIAL_WIDTH: u32 = 1024;
const INITIAL_HEIGHT: u32 = 768;

fn main()
{
    let window = Window::new_centered("Mandelbrot", (INITIAL_WIDTH, INITIAL_HEIGHT)).unwrap();
    let mut mandelbrot = Mandelbrot::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    mandelbrot.compute();
    let color_model: BlackWhite = BlackWhite::new();
    window.run_loop(MyWindowHandler { mandelbrot, color_model: Box::new(color_model) });
}

struct MyWindowHandler
{
    mandelbrot: Mandelbrot,
    color_model: Box<dyn ColorModel>,
}

impl WindowHandler for MyWindowHandler
{
    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: UVec2) {

        self.mandelbrot.resize(size_pixels.x, size_pixels.y);
        self.mandelbrot.compute();
        println!("request redraw");
        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::WHITE);
        let mandelbrot = &self.mandelbrot;
        let (width, height) = mandelbrot.dimensions();
        for y in 0..height {
            for x in 0..width {
                let iterations = mandelbrot.get_pixel_iterations(x, y);
                let color = self.color_model.iterations_to_rgb(iterations);
                graphics.draw_line((x as f32, y as f32),
                                   ((x+1) as f32, (y+1) as f32),
                                   1.0,
                                   color);
            }
        }
        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
}