use std::ops;

#[derive(Clone, Copy)]
pub(crate) struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    pub(crate) fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    fn square(&self) -> Complex {
        Complex {
            re: self.re * self.re - self.im * self.im,
            im: 2.0 * self.re * self.im,
        }
    }

    pub(crate) fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}