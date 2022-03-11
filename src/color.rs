use std::default::Default;

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

    pub fn as_bytes(&self) -> [u8; 3] {
        [
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
        ]
    }

    pub fn lerp_rgb(from: Self, to: Self, t: f32) -> Self {
        Self {
            red: to.red.mul_add(t, from.red * (1.0 - t)),
            blue: to.blue.mul_add(t, from.blue * (1.0 - t)),
            green: to.green.mul_add(t, from.green * (1.0 - t)),
            alpha: to.alpha.mul_add(t, from.alpha * (1.0 - t)),
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
