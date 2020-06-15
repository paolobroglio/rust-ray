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


pub fn write_color(color: Color, samples_per_pixel: f32) -> String {
    let scale = 1.0 / samples_per_pixel;
    let r = (clamp(color.x() * scale, 0.0, 0.999) * 256.0) as u32;
    let g = (clamp(color.y() * scale, 0.0, 0.999) * 256.0) as u32;
    let b = (clamp(color.z() * scale, 0.0, 0.999) * 256.0) as u32;
    format!("{} {} {}\n", r, g, b)
}