use speedy2d::color::Color;
use crate::color_model::ColorModel;
use crate::constants::MAX_ITERATIONS;

pub(crate) struct BlackWhite {
    max_iterations: u16,
}

impl BlackWhite {
    pub(crate) fn new() -> Self {
        Self::new_with_max_iterations(MAX_ITERATIONS)
    }

    pub(crate) fn new_with_max_iterations(max_iterations: u16) -> Self {
        Self {
            max_iterations,
        }
    }
}

impl ColorModel for BlackWhite {
    fn iterations_to_rgb(&self, iterations: u16) -> Color {
        let gray_scale: f32 = 255.0 - iterations as f32 * 255.0 / self.max_iterations as f32;
        // if iterations > 0 {
        //     println!("iterations: {}, gray_scale: {}", iterations, gray_scale);
        // }
        Color::from_gray(gray_scale)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_black() {
        let black_white = BlackWhite::new_with_max_iterations(150);
        assert_eq!(black_white.iterations_to_rgb(150), Color::from_gray(0.0));
    }

    #[test]
    fn test_white() {
        let black_white = BlackWhite::new();
        assert_eq!(black_white.iterations_to_rgb(0), Color::from_gray(255.0));
    }

    #[test]
    fn test_gray() {
        let black_white = BlackWhite::new_with_max_iterations(100);
        assert_eq!(black_white.iterations_to_rgb(50), Color::from_gray(127.5));
    }
}