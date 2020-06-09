use std::fs::File;
use std::io::Write;

use crate::ppm::PpmImage;
use crate::vec3::{Point3, Vec3};

mod tuple;
mod environment;
mod matrix;
mod transformations;
mod vec3;
mod ppm;
mod ray;

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let width = 384;
    let height = (width as f64 / aspect_ratio) as usize;
    let mut ppm = PpmImage::new(height, width);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new_zeroes();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    let low_left_corner = origin.clone() - horizontal.div_by(2.0) - vertical.div_by(2.0) -
        Vec3::new(0.0, 0.0, focal_length);

    ppm.draw(&origin, &horizontal, &vertical, &low_left_corner);

    ppm.write()?;
    Ok(())
}
