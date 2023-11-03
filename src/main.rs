mod mandelbrot;

use std::thread;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::{Graphics2D, Window};
use speedy2d::dimen::UVec2;
use crate::mandelbrot::Mandelbrot;

const INITIAL_WIDTH: u32 = 1024;
const INITIAL_HEIGHT: u32 = 768;

fn main()
{
    let window = Window::new_centered("Speedy2D: Hello World", (INITIAL_WIDTH, INITIAL_HEIGHT)).unwrap();
    let mut mandelbrot = Mandelbrot::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    mandelbrot.compute();
    window.run_loop(MyWindowHandler { mandelbrot })
}

struct MyWindowHandler
{
    mandelbrot: Mandelbrot,
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
        let image = mandelbrot.image();
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y);
                let color = Color::from_rgb(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0);
                graphics.draw_line((x as f32, y as f32), ((x+1) as f32, (y+1) as f32), 1.0, color);
            }
        }
        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
}