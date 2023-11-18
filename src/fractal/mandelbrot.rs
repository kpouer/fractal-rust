use rayon::prelude::*;

use crate::constants::MAX_ITERATIONS;
use crate::fractal::{scale_x, scale_y};
use crate::fractal::complex::Complex;
use crate::fractal::image::Image;
use crate::fractal::params::Params;
use crate::fractal::pixel::Pixel;

const MIN_X: f64 = -2.0;
const MAX_X: f64 = 1.0;
const MIN_Y: f64 = -1.0;
const MAX_Y: f64 = 1.0;

pub(crate) fn get_mandelbrot_params() -> Params {
    Params {
        max_iterations: MAX_ITERATIONS,
        support_zoom: true,
        min_x: MIN_X,
        width: MAX_X - MIN_X,
        min_y: MIN_Y,
        height: MAX_Y - MIN_Y,
    }
}

pub(crate) fn compute_mandelbrot(params: &Params, image: &mut Image) {
    println!("compute");
    let (width, height) = image.dimensions();
    let height_ranges: Vec<u16> = (0..height).collect();
    let pixels: Vec<Vec<Pixel>> = height_ranges
        .par_iter()
        .map(|y| {
            let mut pixel_vec = Vec::new();
            for x in 0..width {
                let iterations = is_in_mandelbrot_set(image.dimensions(), x, *y, params.min_x, params.min_y, params.width, params.height, params.max_iterations).len() as u16;
                pixel_vec.push(Pixel::new(x, *y, iterations));
            }
            pixel_vec
        }).collect();
    for pixel_vec in pixels {
        let pixel_vec: Vec<Pixel> = pixel_vec;
        for pixel in pixel_vec {
            image.set_pixel_iterations(&pixel);
        }
    }
}

pub(crate) fn is_in_mandelbrot_set(image_dimension: (u16, u16), x: u16, y: u16, min_x: f64, min_y: f64, width: f64, height: f64, max_iterations: u16) -> Vec<Complex> {
    let scaled_x = scale_x(x, image_dimension.0 as f64, min_x, width);
    let scaled_y = scale_y(y, image_dimension.1 as f64, min_y, height);
    let c = Complex::new(scaled_x, scaled_y);
    let mut points: Vec<Complex> = Vec::new();
    let mut z = Complex::new(0.0, 0.0);
    points.push(z);
    for _ in 0..max_iterations {
        z = z * z + c;
        points.push(z);
        if z.norm_sqr() > 4.0 {
            return points;
        }
    }
    points
}