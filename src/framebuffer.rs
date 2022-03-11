use std::io::Write;

use crate::color::Color;

pub struct FrameBuffer {
    pub width: u16,
    pub height: u16,
    buffer: Vec<Color>,
}

impl FrameBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            buffer: vec![Color::new(0.0, 0.0, 0.0, 1.0); usize::from(width) * usize::from(height)],
        }
    }

    pub fn fill(&mut self, color: Color) {
        self.buffer.fill(color);
    }

    pub fn pixel(&mut self, x: u16, y: u16, color: Color) {
        if let Some(pixel) = self
            .buffer
            .get_mut(usize::from(y) * usize::from(self.width) + usize::from(x))
        {
            *pixel = color;
        }
    }

    fn get(&self, x: u16, y: u16) -> Color {
        self.buffer
            .get(usize::from(y) * usize::from(self.width) + usize::from(x))
            .copied()
            .unwrap_or_default()
    }

    fn header(&self) -> String {
        format!("P6\n{} {}\n255\n", self.width, self.height)
    }

    pub fn write<T: Write>(&self, to: &mut T) {
        write!(to, "{}", self.header()).expect("I/O Error: Unable to write.");
        let bytes: Vec<u8> = self.buffer.iter().flat_map(Color::as_bytes).collect();
        to.write_all(&bytes).expect("I/O Error: Unable to write.");
        to.flush().expect("I/O Error: Unable to flush.");
    }
}
