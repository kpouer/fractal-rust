use rand::{Rng, thread_rng};
use speedy2d::dimen::Vec2;
use crate::constants::MAX_ITERATIONS;
use crate::fractal::complex::Complex;
use crate::fractal::image::Image;
use crate::fractal::pixel::Pixel;
use crate::fractal::{scale_x, scale_x_to_image, scale_y, scale_y_to_image};

const MIN_X: f64 = -2.0;
const MAX_X: f64 = 1.0;
const MIN_Y: f64 = -1.0;
const MAX_Y: f64 = 1.0;

pub(crate) struct Buddahbrot {
    pub(crate) image: Image,
    version: u32,
    min_x: f64,
    width: f64,
    min_y: f64,
    height: f64,
    max_iterations: u16,
}

impl Buddahbrot {
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

    pub(crate) fn set_center(&mut self, center_pixel: Vec2) {
        let scaled_x = scale_x(center_pixel.x, self.image.width() as f64, self.min_x, self.width);
        let scaled_y = scale_y(center_pixel.y, self.image.height() as f64, self.min_y, self.height);
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

    pub(crate) fn compute(&mut self) {
        println!("compute");
        let pixels = self.compute_points();
        for complex in pixels {
            let x: u16 = scale_x_to_image(complex.re, self.image.width() as f64, self.min_x, self.width);
            let y: u16 = scale_y_to_image(complex.im, self.image.height() as f64, self.min_y, self.height);
            self.image.increment_pixel(&Pixel::new(x, y, 0));
        }
    }

    fn compute_points(&mut self) -> Vec<Complex> {
        let mut rng = thread_rng();
        let mut all_points: Vec<Complex> = Vec::new();
        for _ in 0..100000 {
            let (x, y) = (rng.gen_range(0..self.image.width()), rng.gen_range(0..self.image.height()));
            if let Some(points) = self.is_in_mandelbrot_set(x, y, self.max_iterations) {
                for p in points {
                    all_points.push(p);
                }
            }
        }

        all_points
    }

    fn is_in_mandelbrot_set(&self, x: u16, y: u16, max_iterations: u16) -> Option<Vec<Complex>> {
        let scaled_x = scale_x(x, self.image.width() as f64, self.min_x, self.width);
        let scaled_y = scale_y(y, self.image.height() as f64, self.min_y, self.height);
        let c = Complex::new(scaled_x, scaled_y);
        let mut points: Vec<Complex> = Vec::new();
        let mut z = Complex::new(0.0, 0.0);
        points.push(z);
        for _ in 0..max_iterations {
            z = z * z + c;
            points.push(z);
            if z.norm_sqr() > 4.0 {
                return None;
            }
        }
        Some(points)
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