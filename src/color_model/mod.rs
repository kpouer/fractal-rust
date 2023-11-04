pub(crate) mod black_white;
pub(crate) mod hsv_color;

use speedy2d::color::Color;

pub(crate) trait ColorModel {
    fn iterations_to_rgb(&self, iterations: u16) -> Color;
}

pub(crate) enum ColorModelType {
    BlackWhite,
    HSVColor,
}

pub(crate) fn get_color_model(color_model_type: ColorModelType) -> Box<dyn ColorModel> {
    return match color_model_type {
        ColorModelType::BlackWhite => { Box::new(black_white::BlackWhite::new()) }
        ColorModelType::HSVColor => { Box::new(hsv_color::HSVColor::new()) }
    }
}