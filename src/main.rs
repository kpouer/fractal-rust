mod mandelbrot;

use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::{Graphics2D, Window};
use speedy2d::dimen::UVec2;
use speedy2d::image::ImageDataType::RGB;
use speedy2d::image::ImageSmoothingMode::NearestNeighbor;
use crate::mandelbrot::Mandelbrot;

fn main()
{
    let window = Window::new_centered("Speedy2D: Hello World", (512, 512)).unwrap();
    let mandelbrot = Mandelbrot::new();
    window.run_loop(MyWindowHandler { mandelbrot })
}

struct MyWindowHandler
{
    mandelbrot: Mandelbrot,
}

impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::WHITE);
        let mandelbrot = &self.mandelbrot;
        let image = mandelbrot.image();
      //  let gimage = graphics.create_image_from_raw_pixels(RGB, NearestNeighbor, image.into_raw(), false).unwrap(
        //graphics.draw_image(image, (0.0, 0.0), (image.width() as f32, image.height() as f32), 1.0, gimage);
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