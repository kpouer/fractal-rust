mod complex;
mod image;
mod pixel;

use speedy2d::dimen::Vec2;
use crate::constants::MAX_ITERATIONS;
use crate::mandelbrot::complex::Complex;
use crate::mandelbrot::image::Image;
use rayon::prelude::*;
use crate::mandelbrot::pixel::Pixel;

const MIN_X: f64 = -2.0;
const MAX_X: f64 = 1.0;
const MIN_Y: f64 = -1.0;
const MAX_Y: f64 = 1.0;

pub(crate) struct Mandelbrot {
    image: Image,
    version: u32,
    min_x: f64,
    width: f64,
    min_y: f64,
    height: f64,
    max_iterations: u16,
}

impl Mandelbrot {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {
            image: Image::new(width, height),
            version: 0,
            min_x: MIN_X,
            width: MAX_X - MIN_X,
            min_y: MIN_Y,
            height: MAX_Y - MIN_Y,
            max_iterations: MAX_ITERATIONS,
        }
    }

    pub(crate) fn set_center(&mut self, center: Vec2) {
        let scaled_x = self.scale_x(center.x);
        let scaled_y = self.scale_y(center.y);
        self.width /= 2.0;
        self.height /= 2.0;
        self.min_x = scaled_x - self.width / 2.0;
        self.min_y = scaled_y - self.height / 2.0;
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

    pub(crate) fn iterations(&self) -> &[u16] {
        &self.image.iterations()
    }

    pub(crate) fn compute(&mut self) {
        println!("compute");
        let (width, height) = self.image.dimensions();
        let height_ranges: Vec<u16> = (0..height).collect();
        let pixels:Vec<Vec<Pixel>> = height_ranges
            .par_iter()
            .map(|y| {
                let mut pixel_vec = Vec::new();
                for x in 0..width {
                    let iterations = self.is_in_mandelbrot_set(x, *y, self.max_iterations);
                    pixel_vec.push(Pixel::new(x, *y, iterations));
                }
                pixel_vec
            }).collect();
        for pixel_vec in pixels {
            let pixel_vec: Vec<Pixel> = pixel_vec;
            for pixel in pixel_vec {
                self.image.set_pixel_iterations(&pixel);
            }
        }
    }

    fn is_in_mandelbrot_set(&self, x: u16, y: u16, max_iterations: u16) -> u16 {
        let scaled_x = self.scale_x(x);
        let scaled_y = self.scale_y(y);
        let c = Complex::new(scaled_x, scaled_y);
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..max_iterations {
            z = z * z + c;
            if z.norm_sqr() > 4.0 {
                return i;
            }
        }
        max_iterations
    }

    fn scale_x<T: Into<f64>>(&self, x: T) -> f64 {
        (x.into() / self.image.width() as f64) * self.width + self.min_x
    }

    fn scale_y<T: Into<f64>>(&self, y: T) -> f64 {
        (y.into() / self.image.height() as f64) * self.height + self.min_y
    }

    pub(crate) fn zoom_in(&mut self) {
        self.min_x += self.width / 4.0;
        self.min_y += self.height / 4.0;
        self.width /= 2.0;
        self.height /= 2.0;
    }

    pub(crate) fn zoom_out(&mut self) {
        self.min_x -= self.width;
        self.min_y -= self.height;
        self.width *= 2.0;
        self.height *= 2.0;
    }

    pub(crate) fn max_iterations(&self) -> u16 {
        self.max_iterations
    }

    pub(crate) fn decrease_iterations(&mut self) {
       self.max_iterations /= 2;
        if self.max_iterations < 10 {
            self.max_iterations = 10;
        }
        println!("max_iterations: {}", self.max_iterations)
    }

    pub(crate) fn increase_iterations(&mut self) {
        self.max_iterations *= 2;
        println!("max_iterations: {}", self.max_iterations)
    }
}