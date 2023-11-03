mod complex;
mod image;

use crate::constants::MAX_ITERATIONS;
use crate::mandelbrot::complex::Complex;
use crate::mandelbrot::image::Image;

const MIN_X: f64 = -2.0;
const MAX_X: f64 = 1.0;
const MIN_Y: f64 = -1.0;
const MAX_Y: f64 = 1.0;

pub(crate) struct Mandelbrot {
    image: Image,
    version: u32,
}

impl Mandelbrot {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {
            image: Image::new(width, height),
            version: 0,
        }
    }

    pub(crate) fn resize(&mut self, new_width: u32, new_height: u32) {
        self.version += 1;
        self.image = Image::new(new_width, new_height);
    }

    pub(crate) fn dimensions(&self) -> (u16, u16) {
        self.image.dimensions()
    }

    pub(crate) fn get_pixel_iterations(&self, x: u16, y: u16) -> u16 {
        self.image.get_pixel_iterations(x, y)
    }

    pub(crate) fn compute(&mut self) {
        println!("compute");
        let (width, height) = self.image.dimensions();
        let version = self.version;
        for y in 0..height {
            for x in 0..width {
                if version != self.version {
                    return;
                }
                let iterations = self.is_in_mandelbrot_set(x, y);
                self.image.set_pixel_iterations(x, y, iterations);
            }
        }
    }

    fn is_in_mandelbrot_set(&self, x: u16, y: u16) -> u16 {
        let scaled_x = (x as f64 / self.image.width() as f64) * (MAX_X - MIN_X) + MIN_X;
        let scaled_y = (y as f64 / self.image.height() as f64) * (MAX_Y - MIN_Y) + MIN_Y;
        let c = Complex::new(scaled_x, scaled_y);
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..MAX_ITERATIONS {
            z = z * z + c;
            if z.norm_sqr() > 4.0 {
                return i as u16;
            }
        }
        MAX_ITERATIONS
    }
}