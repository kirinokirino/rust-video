use crate::color::Color;
use crate::framebuffer::PPM;

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let x = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    x.powi(2) * (3.0 - 2.0 * x)
}

pub fn dot(
    buf: &mut (impl PPM + ?Sized),
    x: f32,
    y: f32,
    color: Color,
    radius: f32,
    blur: Option<f32>,
) {
    let min_radius = radius;
    let max_radius = radius + blur.unwrap_or(5.0);
    let Color {
        red: fr,
        green: fg,
        blue: fb,
        ..
    } = color;
    let miny = (y - max_radius - 1.0).floor() as u32;
    let maxy = (y + max_radius + 1.0).ceil() as u32;
    let minx = (x - max_radius - 1.0).floor() as u32;
    let maxx = (x + max_radius + 1.0).ceil() as u32;

    for py in miny..maxy {
        let dy = py as f32 - y;
        for px in minx..maxx {
            let dx = px as f32 - x;
            let d = dy.hypot(dx);
            let a = smoothstep(max_radius, min_radius, d);
            let Color {
                red: br,
                green: bg,
                blue: bb,
                ..
            } = buf.ppm_get(px, py);
            let r = a.mul_add(fr, (1.0 - a) * br);
            let g = a.mul_add(fg, (1.0 - a) * bg);
            let b = a.mul_add(fb, (1.0 - a) * bb);
            buf.ppm_set(px, py, Color::new(r, g, b, 1.0));
        }
    }
}
