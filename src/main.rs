use speedy2d::{Graphics2D, Window};
use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::image::ImageDataType::RGB;
use speedy2d::image::ImageHandle;
use speedy2d::image::ImageSmoothingMode::NearestNeighbor;
use speedy2d::window::{KeyScancode, MouseButton, VirtualKeyCode, WindowHandler, WindowHelper};

use crate::color_model::{ColorArgs, get_color_model};
use crate::constants::DEFAULT_COLOR_MODEL;
use crate::mandelbrot::Mandelbrot;

mod mandelbrot;
mod color_model;
mod constants;

const INITIAL_WIDTH: u32 = 1024;
const INITIAL_HEIGHT: u32 = 768;

fn main()
{
    let window = Window::new_centered("Mandelbrot", (INITIAL_WIDTH, INITIAL_HEIGHT)).unwrap();
    let mandelbrot = Mandelbrot::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    let color_model = get_color_model(DEFAULT_COLOR_MODEL);
    window.run_loop(MyWindowHandler { mandelbrot, color_model, mouse_position: Vec2::ZERO, image: None, must_redraw: true });
}

struct MyWindowHandler
{
    mandelbrot: Mandelbrot,
    color_model: Box<dyn Fn(&ColorArgs) -> Color>,
    mouse_position: Vec2,
    image: Option<ImageHandle>,
    must_redraw: bool,
}

impl WindowHandler for MyWindowHandler
{
    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        self.mandelbrot.resize(size_pixels.x, size_pixels.y);
        self.must_redraw = true;
        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        if self.must_redraw {
            self.must_redraw = false;
            let start = std::time::Instant::now();
            self.mandelbrot.compute();
            println!("compute in : {:?}", start.elapsed());
            let image = self.build_image(graphics);
            self.image = Some(image);
        }
        if self.image.is_some() {
            let image = self.image.as_ref().unwrap();
            graphics.draw_image(Vec2::ZERO, image);
        } else {
            self.draw_pixels(graphics);
        }
        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _: &mut WindowHelper<()>, position: Vec2) {
        self.mouse_position = position;
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        if button == MouseButton::Left {
            self.mandelbrot.set_center(self.mouse_position);
            self.must_redraw = true;
            helper.request_redraw();
        }
    }

    fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, _: KeyScancode) {
        match virtual_key_code {
            None => {}
            Some(VirtualKeyCode::W) => {
                self.mandelbrot.decrease_iterations();
                self.must_redraw = true;
                helper.request_redraw();
            }
            Some(VirtualKeyCode::X) => {
                self.mandelbrot.increase_iterations();
                self.must_redraw = true;
                helper.request_redraw();
            }
            Some(VirtualKeyCode::V) => {
                self.mandelbrot.zoom_in();
                self.must_redraw = true;
                helper.request_redraw();
            }
            Some(VirtualKeyCode::C) => {
                self.mandelbrot.zoom_out();
                self.must_redraw = true;
                helper.request_redraw();
            }
            _ => {}
        }
    }
}

impl MyWindowHandler {
    fn draw_pixels(&mut self, graphics: &mut Graphics2D) {
        let start = std::time::Instant::now();
        graphics.clear_screen(Color::WHITE);
        let mandelbrot = &self.mandelbrot;
        let (width, height) = mandelbrot.dimensions();
        let mut color_args = ColorArgs::new(0, self.mandelbrot.max_iterations());
        let color_function = &self.color_model;
        for y in 0..height {
            for x in 0..width {
                let iterations = mandelbrot.get_pixel_iterations(x, y);
                color_args.iterations = iterations;
                let color = color_function(&color_args);
                graphics.draw_line((x as f32, y as f32),
                                   ((x + 1) as f32, (y + 1) as f32),
                                   1.0,
                                   color);
            }
        }
        let elapsed = start.elapsed();
        println!("elapsed: {:?}", elapsed);
    }

    fn build_image(&mut self, graphics: &mut Graphics2D) -> ImageHandle {
        let data = &self.mandelbrot.image.iterations;
        let color_function = &self.color_model;
        let mut buffer: Vec<u8> = Vec::new();
        data.iter()
            .map(|iterations| {
                let color_args = ColorArgs::new(*iterations, self.mandelbrot.max_iterations());
                color_function(&color_args)
            })
            .for_each(|color| {
                let r = (color.r() * 255.0) as u8;
                let g = (color.g() * 255.0) as u8;
                let b = (color.b() * 255.0) as u8;
                buffer.push(r);
                buffer.push(g);
                buffer.push(b);
            });
        let (width, height) = self.mandelbrot.dimensions();
        let size = UVec2::new(width as u32, height as u32);
        let raw = buffer.as_slice();
        let image = graphics.create_image_from_raw_pixels(
            RGB,
            NearestNeighbor,
            size,
            raw);
        image.unwrap()
    }
}