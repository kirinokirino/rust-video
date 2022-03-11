pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let x = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    x.powi(2) * (3.0 - 2.0 * x)
}
