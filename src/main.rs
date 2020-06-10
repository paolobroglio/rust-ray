use std::fs::File;
use std::io::Write;

use crate::ppm::write_color;
use crate::vec3::Color;

mod ray;
mod ppm;
mod vec3;

fn main() -> std::io::Result<()> {
    let image_height = 256;
    let image_width = 256;

    let mut content = String::new();
    content.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());
    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = (image_height - 1 - j) as f32 / (image_height - 1) as f32;
            let b = 0.25_f32;
            content.push_str(write_color(Color::new(r, g, b)).as_str());
        }
    }


    let mut file = match File::create("ray.ppm") {
        Ok(file) => file,
        Err(err) => panic!("Could not create file {:?}", err)
    };
    file.write_all(content.as_bytes())?;
    Ok(())
}