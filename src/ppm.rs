use crate::vec3::Color;

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}


pub fn write_color(color: Color, samples_per_pixel: i32) -> String {
    let scale = 1.0 / samples_per_pixel as f32;
    let r = clamp(color.x() * scale, 0.0, 0.999) as u32 * 256;
    let g = clamp(color.y() * scale, 0.0, 0.999) as u32 * 256;
    let b = clamp(color.z() * scale, 0.0, 0.999) as u32 * 256;
    format!("{} {} {}\n", r, g, b)
}