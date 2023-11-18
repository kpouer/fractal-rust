use params::Params;
use crate::fractal::buddahbrot::compute_buddahbrot;
use crate::fractal::fractal_type::FractalType;

use crate::fractal::image::Image;
use crate::fractal::mandelbrot::{compute_mandelbrot, get_mandelbrot_params};

mod complex;
pub(crate) mod image;
mod pixel;
pub(crate) mod buddahbrot;
pub(crate) mod mandelbrot;
pub(crate) mod fractal_type;
pub(crate) mod params;

pub(crate) fn get_params(fractal_type: &FractalType) -> Params {
    match fractal_type {
        FractalType::Mandelbrot => get_mandelbrot_params(),
        FractalType::Buddahbrot => {
            let mut params = get_mandelbrot_params();
            params.support_zoom = false;
            params
        }
    }
}

pub(crate) fn get_compute_function(fractal_type: &FractalType) -> Box<dyn Fn(&Params, &mut Image)> {
    match fractal_type {
        FractalType::Mandelbrot => { Box::new(compute_mandelbrot) },
        FractalType::Buddahbrot => { Box::new(compute_buddahbrot) }
    }
}

pub(crate) fn scale_x<T: Into<f64>>(x: T, image_width: f64, min_x_re: f64, width_re: f64) -> f64 {
    (x.into() / image_width) * width_re + min_x_re
}

pub(crate) fn scale_y<T: Into<f64>>(y: T, image_height: f64, min_y_im: f64, height_im: f64) -> f64 {
    (y.into() / image_height) * height_im + min_y_im
}

pub(crate) fn scale_x_to_image(x: f64, image_width: f64, min_x_re: f64, width_re: f64) -> u16 {
    ((x - min_x_re) / width_re * image_width) as u16
}

pub(crate) fn scale_y_to_image(y: f64, image_height: f64, min_y_im: f64, height_im: f64) -> u16 {
    ((y - min_y_im) / height_im * image_height) as u16
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_scale_x_min() {
        let min_re = -2.0;
        let max_re = 1.0;
        let width_re = max_re - min_re;
        assert_eq!(min_re, scale_x(0, 200.0, min_re, width_re));
    }

    #[test]
    fn test_scale_x_max() {
        let min_re = -2.0;
        let max_re = 1.0;
        let width_re = max_re - min_re;
        assert_eq!(max_re, scale_x(200.0, 200.0, min_re, width_re));
    }

    #[test]
    fn test_scale_y_min() {
        let min_im = -1.0;
        let max_im = 1.0;
        let height_im = max_im - min_im;
        assert_eq!(min_im, scale_y(0, 200.0, min_im, height_im));
    }

    #[test]
    fn test_scale_y_max() {
        let min_im = -1.0;
        let max_im = 1.0;
        let height_im = max_im - min_im;
        assert_eq!(max_im, scale_y(200.0, 200.0, min_im, height_im));
    }
}