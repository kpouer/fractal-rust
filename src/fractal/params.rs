use speedy2d::dimen::Vec2;

use crate::fractal::{scale_x, scale_y};

pub(crate) struct Params {
    pub(crate) max_iterations: u16,
    pub(crate) support_zoom: bool,
    pub(crate) min_x: f64,
    pub(crate) width: f64,
    pub(crate) min_y: f64,
    pub(crate) height: f64,
}

impl Params {
    pub(crate) fn decrease_iterations(&mut self) {
        self.max_iterations /= 2;
        if self.max_iterations < 10 {
            self.max_iterations = 10;
        }
        println!("max_iterations: {}", self.max_iterations)
    }

    pub(crate) fn increase_iterations(&mut self) {
        self.max_iterations *= 2;
        println!("max_iterations: {}", self.max_iterations)
    }

    pub(crate) fn set_center(&mut self, center_pixel: Vec2, image_width: f64, image_height: f64) {
        let scaled_x = scale_x(center_pixel.x, image_width, self.min_x, self.width);
        let scaled_y = scale_y(center_pixel.y, image_height, self.min_y, self.height);
        self.width /= 2.0;
        self.height /= 2.0;
        self.min_x = scaled_x - self.width / 2.0;
        self.min_y = scaled_y - self.height / 2.0;
    }

    pub(crate) fn zoom_in(&mut self) {
        self.min_x += self.width / 4.0;
        self.min_y += self.height / 4.0;
        self.width /= 2.0;
        self.height /= 2.0;
    }

    pub(crate) fn zoom_out(&mut self) {
        self.min_x -= self.width;
        self.min_y -= self.height;
        self.width *= 2.0;
        self.height *= 2.0;
    }
}