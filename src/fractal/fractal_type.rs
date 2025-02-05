use crate::fractal::buddahbrot::compute_buddahbrot;
use crate::fractal::image::Image;
use crate::fractal::mandelbrot::compute_mandelbrot;
use crate::fractal::params::Params;

#[derive(Copy, Clone)]
pub(crate) enum FractalType {
    Mandelbrot,
    Buddahbrot,
}

impl FractalType {
    pub(crate) fn compute(&self, params: &Params, image: &mut Image) {
        match self {
            FractalType::Mandelbrot => compute_mandelbrot(params, image),
            FractalType::Buddahbrot => compute_buddahbrot(params, image),
        }
    }
}
