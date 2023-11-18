use crate::color_model::ColorModelType;
use crate::fractal::fractal_type::FractalType;

pub(crate) const MAX_ITERATIONS: u16 = 40;
pub(crate) const DEFAULT_COLOR_MODEL: ColorModelType = ColorModelType::BlackWhite;
pub(crate) const DEFAULT_FRACTAL: FractalType = FractalType::Mandelbrot;