use crate::vec3::Color;

pub fn write_color(color: Color) -> String {
    let r = (color.x() * 255.999) as u32;
    let g = (color.y() * 255.999) as u32;
    let b = (color.z() * 255.999) as u32;
    format!("{} {} {}\n", r, g, b)
}