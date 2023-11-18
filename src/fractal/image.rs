use crate::fractal::pixel::Pixel;

pub(crate) struct Image {
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) iterations: Vec<u16>,

}

impl Image {
    pub(crate) fn new(width: u32, height: u32) -> Image {
        Image {
            width: width as u16,
            height: height as u16,
            iterations: vec![0; (width * height) as usize],
        }
    }

    pub(crate) fn set_pixel_iterations(&mut self, pixel: &Pixel) {
        let index = self.get_index(pixel.x(), pixel.y());
        if index >= self.iterations.len() {
            panic!("index out of bounds");
        }
        self.iterations[index] = pixel.iterations();
    }

    pub(crate) fn increment_pixel(&mut self, pixel: &Pixel) {
        let index = self.get_index(pixel.x(), pixel.y());
        if index >= self.iterations.len() {
            // println!("FAIL x: {}, y: {}, index: {}, len: {}", pixel.x(), pixel.y(), index, self.iterations.len());
            return;
        }
        let v = self.iterations[index];
        let v = v + 1;
        if v > 60000 {
            return;
        }
        self.iterations[index] = v;
    }

    pub(crate) fn get_pixel_iterations(&self, x: u16, y: u16) -> u16 {
        let index = self.get_index(x, y);
        if index >= self.iterations.len() {
            panic!("index out of bounds");
        }
        self.iterations[index]
    }

    pub(crate) fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn get_index(&self, x: u16, y: u16) -> usize {
        (y as u32 * self.width as u32 + x as u32) as usize
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_image() {
        const WIDTH: u32 = 1024;
        const HEIGHT: u32 = 768;
        let image = Image::new(WIDTH, HEIGHT);
        assert_eq!(image.width, 1024);
        assert_eq!(image.height, 768);
        assert_eq!(image.get_index(0, 0), 0);
        assert_eq!(image.get_index(0, 1), WIDTH as usize);
  //      assert_eq!(image.get_index((WIDTH - 1) as u16, (HEIGHT - 1) as u16), (WIDTH * HEIGHT) as usize);

    }
}