mod complex;

use image::{ImageBuffer, RgbImage};
use crate::mandelbrot::complex::Complex;

pub(crate) const MAX_ITERATIONS: u32 = 100;

const MIN_X: f64 = -2.0;
const MAX_X: f64 = 1.0;
const MIN_Y: f64 = -1.0;
const MAX_Y: f64 = 1.0;

pub(crate) struct Mandelbrot {
    image: RgbImage,
    version: u32,
}

impl Mandelbrot {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {
            image: ImageBuffer::new(width, height),
            version: 0,
        }
    }

    pub(crate) fn image(&self) -> &RgbImage {
        &self.image
    }

    pub(crate) fn resize(&mut self, new_width: u32, new_height: u32) {
        self.version += 1;
        self.image = ImageBuffer::new(new_width, new_height);
    }

    pub(crate) fn compute(&mut self) {
        let (width, height) = self.image.dimensions();
        let version = self.version;
        for y in 0..height {
            for x in 0..width {
                if version != self.version {
                    return;
                }
                let iterations = self.is_in_mandelbrot_set(x, y);
                if iterations == 100 {
                    self.image.put_pixel(x, y, image::Rgb([0, 0, 0]));
                } else {
                    let ratio = ((iterations * 255) as f64 / 100.0) as u8;
                    self.image.put_pixel(x, y, image::Rgb([ratio, ratio, ratio]));
                }
            }
        }
    }

    fn is_in_mandelbrot_set(&self, x: u32, y: u32) -> u32 {
        let scaled_x = (x as f64 / self.image.width() as f64) * (MAX_X - MIN_X) + MIN_X;
        let scaled_y = (y as f64 / self.image.height() as f64) * (MAX_Y - MIN_Y) + MIN_Y;
        let c = Complex::new(scaled_x, scaled_y);
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..MAX_ITERATIONS {
            z = z * z + c;
            if z.norm_sqr() > 4.0 {
                return i;
            }
        }
        MAX_ITERATIONS
    }
}