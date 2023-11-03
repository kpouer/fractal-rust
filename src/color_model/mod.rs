pub(crate) mod black_white;

use speedy2d::color::Color;

pub(crate) trait ColorModel {
    fn iterations_to_rgb(&self, iterations: u16) -> Color;
}