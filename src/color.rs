pub(crate) const BLACK: Color = Color { r: 0, g: 0, b: 0 };

pub(crate) struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}

impl From<&f64> for Color {
    fn from(value: &f64) -> Self {
        let color = if *value < 0.0 {
             0u8
        } else if *value > 1.0 {
             255
        } else {
            (*value * 255.0) as u8
        };
        Self {
            r: color,
            g: color,
            b: color,
        }
    }
}