use hsv::hsv_to_rgb;
use speedy2d::color::Color;

pub(crate) struct ColorArgs {
    pub(crate) iterations: u16,
    pub(crate) max_iterations: u16,
}

impl ColorArgs {
    pub(crate) fn new(iterations: u16, max_iterations: u16) -> Self {
        Self {
            iterations,
            max_iterations,
        }
    }

    pub(crate) fn set_iterations(&mut self, iterations: u16) {
        self.iterations = iterations;
    }

    fn iterations_as_f64(&self) -> f64 {
        self.iterations as f64
    }

    fn max_iterations_as_f64(&self) -> f64 {
        self.max_iterations as f64
    }

    fn escaped(&self) -> bool {
        self.iterations < self.max_iterations
    }
}

fn black_and_white(color_args: &ColorArgs) -> Color {
    let gray_scale: f64 = color_args.iterations_as_f64() / color_args.max_iterations_as_f64();
    Color::from_gray(gray_scale as f32)
}

fn hsv(color_args: &ColorArgs) -> Color {
    if !color_args.escaped() {
        return Color::BLACK;
    }
    let hue: f64 = color_args.iterations_as_f64() * 360.0 / color_args.max_iterations_as_f64();
    let saturation: f64 = 1.0;
    let lightness: f64 = 0.5;
    let color = hsv_to_rgb(hue, saturation, lightness);
    Color::from_int_rgb(color.0, color.1, color.2)
}

pub(crate) enum ColorModelType {
    BlackWhite,
    HSVColor,
}

pub(crate) fn get_color_model(color_model_type: ColorModelType) -> Box<dyn Fn(&ColorArgs) -> Color> {
    return match color_model_type {
        ColorModelType::BlackWhite => { Box::new(black_and_white) }
        ColorModelType::HSVColor => { Box::new(hsv) }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_black() {
        let color_arg = ColorArgs::new(150, 150);
        assert_eq!(black_and_white(&color_arg), Color::from_gray(0.0));
    }

    #[test]
    fn test_white() {
        let color_arg = ColorArgs::new(0, 150);
        assert_eq!(black_and_white(&color_arg), Color::from_gray(255.0));
    }

    #[test]
    fn test_gray() {
        let color_arg = ColorArgs::new(50, 100);
        assert_eq!(black_and_white(&color_arg), Color::from_gray(127.5));
    }
}