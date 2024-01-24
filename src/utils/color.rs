use std::ops::Mul;

use opencv::core::VecN;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_vecn(vec: &VecN<u8, 3>) -> Self {
        Self {
            r: vec[0],
            g: vec[1],
            b: vec[2],
        }
    }

    pub fn to_greyscale(&self) -> Self {
        let greyscale =
            (self.r as f32 * 0.299 + self.g as f32 * 0.587 + self.b as f32 * 0.114) as u8;
        Self {
            r: greyscale,
            g: greyscale,
            b: greyscale,
        }
    }

    pub fn multiply(&self, factor: f32) -> Self {
        Self {
            r: (self.r as f32 * factor) as u8,
            g: (self.g as f32 * factor) as u8,
            b: (self.b as f32 * factor) as u8,
        }
    }

    /// Max distance is 441.67295593
    pub fn distance_from(&self, other: &Self) -> f32 {
        let r = self.r as f32 - other.r as f32;
        let g = self.g as f32 - other.g as f32;
        let b = self.b as f32 - other.b as f32;

        (r * r + g * g + b * b).sqrt()
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
