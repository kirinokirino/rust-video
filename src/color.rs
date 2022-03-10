#[derive(Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
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
