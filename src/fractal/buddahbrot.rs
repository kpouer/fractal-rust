use rand::{Rng, thread_rng};

use crate::fractal::{scale_x_to_image, scale_y_to_image};
use crate::fractal::complex::Complex;
use crate::fractal::image::Image;
use crate::fractal::mandelbrot::is_in_mandelbrot_set;
use crate::fractal::params::Params;
use crate::fractal::pixel::Pixel;

pub(crate) fn compute_buddahbrot(params: &Params, image: &mut Image) {
    println!("compute");
    let pixels = compute_points(image.dimensions(), params.min_x, params.min_y, params.width, params.height, params.max_iterations);
    for complex in pixels {
        let x: u16 = scale_x_to_image(complex.re, image.width as f64, params.min_x, params.width);
        let y: u16 = scale_y_to_image(complex.im, image.height as f64, params.min_y, params.height);
        image.increment_pixel(&Pixel::new(x, y, 0));
    }
}

fn compute_points(image_dimension: (u16, u16), min_x: f64, min_y: f64, width: f64, height: f64, max_iterations: u16) -> Vec<Complex> {
    let mut rng = thread_rng();
    let mut all_points: Vec<Complex> = Vec::new();
    for _ in 0..100000 {
        let (x, y) = (rng.gen_range(0..image_dimension.0), rng.gen_range(0..image_dimension.1));
        let points = is_in_mandelbrot_set(image_dimension, x, y, min_x, min_y, width, height, max_iterations);
        for p in points {
            all_points.push(p);
        }
    }

    all_points
}