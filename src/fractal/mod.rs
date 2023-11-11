mod complex;
mod image;
mod pixel;
pub(crate) mod buddahbrot;
pub(crate) mod mandelbrot;
pub(crate) mod fractal_type;

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