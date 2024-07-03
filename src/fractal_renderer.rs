use eframe::epaint::{ColorImage, ImageData};

use crate::color::Color;
use crate::color_model;
use crate::color_model::ColorArgs;
use crate::constants::DEFAULT_COLOR_MODEL;
use crate::fractal::get_compute_function;
use crate::fractal::image::Image;
use crate::fractal::params::Params;

pub(crate) struct FractalRenderer {
    width: u16,
    height: u16,
    params: Params,
    canvas: Image,
    color_model: Box<dyn Fn(&ColorArgs) -> Color>,
}

impl FractalRenderer {
    pub(crate) fn new(width: u16, height: u16, params: Params) -> Self {
        let color_model = color_model::get_color_model(DEFAULT_COLOR_MODEL);

        Self {
            width,
            height,
            params,
            canvas: Image::new(width, height),
            color_model,
        }
    }

    pub(crate) fn compute(&mut self) {
        let compute_function = get_compute_function(&self.params.fractal_type);
        compute_function(&self.params, &mut self.canvas);
    }

     pub(crate) fn build_image(&mut self) -> ImageData {
        let data = &self.canvas.iterations;
        let color_function = &self.color_model;
        let mut buffer: Vec<u8> = Vec::new();
        data.iter()
            .map(|iterations| {
                let color_args = ColorArgs::new(*iterations, self.params.max_iterations);
                color_function(&color_args)
            })
            .for_each(|color| {
                buffer.push(color.r());
                buffer.push(color.g());
                buffer.push(color.b());
            });
        let (width, height) = self.canvas.dimensions();
         let color_image = ColorImage::from_rgb([self.width as usize, self.height as usize], buffer.as_slice());
         let image_data = ImageData::from(color_image);
         image_data
    }
}