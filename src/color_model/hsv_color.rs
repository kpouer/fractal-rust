use hsv::hsv_to_rgb;
use speedy2d::color::Color;
use crate::color_model::ColorModel;
use crate::constants::MAX_ITERATIONS;

pub(crate) struct HSVColor {
    max_iterations: u16,
}

impl HSVColor {
    pub(crate) fn new() -> Self {
        Self::new_with_max_iterations(MAX_ITERATIONS)
    }

    pub(crate) fn new_with_max_iterations(max_iterations: u16) -> Self {
        Self {
            max_iterations,
        }
    }
}

//C = (1 - |2L - 1|) × S
// X = C × (1 - |(H / 60°) mod 2 - 1|)
// m = L - C/2
//
// (R,G,B) = ((R'+m)×255, (G'+m)×255,(B'+m)×255)

impl ColorModel for HSVColor {
    fn iterations_to_rgb(&self, iterations: u16) -> Color {
        if iterations == self.max_iterations {
            return Color::from_gray(0.0);
        }
        let hue: f64 = iterations as f64 * 360.0 / self.max_iterations as f64;
        let saturation: f64 = 1.0;
        let lightness: f64 = 0.5;
        let color = hsv_to_rgb(hue, saturation, lightness);
        // if iterations > 0 {
        //     println!("iterations: {}, gray_scale: {}", iterations, gray_scale);
        // }
        Color::from_int_rgb(color.0, color.1, color.2)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_black() {
        let black_white = HSVColor::new_with_max_iterations(150);
        assert_eq!(black_white.iterations_to_rgb(150), Color::from_gray(0.0));
    }

    #[test]
    fn test_white() {
        let black_white = HSVColor::new();
        assert_eq!(black_white.iterations_to_rgb(0), Color::from_gray(255.0));
    }

    #[test]
    fn test_gray() {
        let black_white = HSVColor::new_with_max_iterations(100);
        assert_eq!(black_white.iterations_to_rgb(50), Color::from_gray(127.5));
    }
}