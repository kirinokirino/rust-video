use std::default::Default;

use crate::common::lerp;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    #[allow(
        clippy::as_conversions,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    pub fn as_bytes(&self) -> [u8; 3] {
        [
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
        ]
    }

    pub fn lerp_rgb(from: Self, to: Self, t: f32) -> Self {
        Self {
            red: lerp(from.red, to.red, t),
            green: lerp(from.green, to.green, t),
            blue: lerp(from.blue, to.blue, t),
            alpha: lerp(from.alpha, to.alpha, t),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }
}
