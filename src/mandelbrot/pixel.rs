pub(crate) struct Pixel {
    x: u16,
    y: u16,
    iterations: u16,
}

impl Pixel {
    pub(crate) fn new(x: u16, y: u16, iterations: u16) -> Self {
        Self {
            x,
            y,
            iterations,
        }
    }

    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn iterations(&self) -> u16 {
        self.iterations
    }
}