mod mandelbrot;
mod color_model;
mod constants;

use speedy2d::color::Color;
use speedy2d::window::{KeyScancode, MouseButton, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::image::ImageDataType::RGB;
use speedy2d::image::ImageHandle;
use speedy2d::image::ImageSmoothingMode::NearestNeighbor;
use crate::color_model::{ColorArgs, get_color_model};
use crate::color_model::ColorModelType::HSVColor;
use crate::mandelbrot::Mandelbrot;

const INITIAL_WIDTH: u32 = 1024;
const INITIAL_HEIGHT: u32 = 768;

fn main()
{
    let window = Window::new_centered("Mandelbrot", (INITIAL_WIDTH, INITIAL_HEIGHT)).unwrap();
    let mut mandelbrot = Mandelbrot::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    mandelbrot.compute();
    let color_model = get_color_model(HSVColor);
    window.run_loop(MyWindowHandler { mandelbrot, color_model, mouse_position: Vec2::ZERO, image: None });
}

struct MyWindowHandler
{
    mandelbrot: Mandelbrot,
    color_model: Box<dyn Fn(&ColorArgs) -> Color>,
    mouse_position: Vec2,
    image: Option<ImageHandle>,
}

impl WindowHandler for MyWindowHandler
{
    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: UVec2) {

        self.mandelbrot.resize(size_pixels.x, size_pixels.y);
        self.repaint(helper);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        if (self.image.is_some()) {
            graphics.draw_image((0, 0), &self.image.unwrap());
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
            self.repaint(helper);
        }
    }

    fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        match virtual_key_code {
            None => {}
            Some(VirtualKeyCode::W) => {self.mandelbrot.decrease_iterations(); self.repaint(helper); }
            Some(VirtualKeyCode::X) => {self.mandelbrot.increase_iterations(); self.repaint(helper); }
            Some(VirtualKeyCode::V) => {self.mandelbrot.zoom_in(); self.repaint(helper); }
            Some(VirtualKeyCode::C) => {self.mandelbrot.zoom_out(); self.repaint(helper);}
            _ => {}
        }
    }
}

impl MyWindowHandler {
    fn repaint(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        println!("repaint");
        self.mandelbrot.compute();
        self.build_image(graphics);
        println!("request redraw");
        helper.request_redraw();
    }

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
                color_args.set_iterations(iterations);
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

    fn build_image(&self, graphics: &mut Graphics2D) {
        let data = self.mandelbrot.iterations();
        let color_function = &self.color_model;
        data.iter().map(|iterations| {
            let color_args = ColorArgs::new(*iterations, self.mandelbrot.max_iterations());
            color_function(&color_args)
        }
        let image = graphics.create_image_from_raw_pixels(
            RGB,
            NearestNeighbor,
            Vec2::new(self.mandelbrot.width() as f32, self.mandelbrot.height() as f32),
            &data);
        todo!()
    }
}