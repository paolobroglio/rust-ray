use std::f32::consts::PI;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::environment::{Environment, Projectile};
use crate::transformations::rotate_y;
use crate::tuple::Tuple;

mod tuple;
mod environment;
mod canvas;
mod color;
mod matrix;
mod transformations;

// fn projectile_ppm() {
//     let environment = Environment::new(
//         Tuple::new_vector(0.0, -0.1, 0.0),
//         Tuple::new_vector(-0.01, 0.0, 0.0),
//     );
//     let mut projectile = Projectile::new(
//         Tuple::new_point(0.0, 1.0, 0.0),
//         Tuple::new_vector(1.0, 1.8, 0.0).normalize().multiply_by(11.25),
//     );
//     let mut canvas = Canvas::new(900, 550);
//     let red = Color::new(255.0, 0.0, 0.0);
//     for _ in 1..198 {
//         projectile = environment.tick(&projectile);
//         canvas.write_pixel(projectile.position.x as usize, canvas.height - projectile.position.y as usize, red)
//     }
//     canvas.to_ppm();
// }

fn clock_ppm() {
    let mut canvas = Canvas::new(50, 50);
    let white = Color::new(255.0, 255.0, 255.0);
    let red = Color::new(255.0, 0.0, 0.0);
    let center_point = Tuple::new_point(25.0, 0.0, 25.0);
    canvas.write_pixel(center_point.x as usize, canvas.height - center_point.z as usize, white);
    println!("center={}", center_point);
    let clock_radius = (3.0 * 50.0) / 8.0_f32;
    println!("radius={}", clock_radius);
    let mut clock_point = Tuple::new_point(center_point.x, 0.0, center_point.z - clock_radius); // 0, 0, 1
    println!("twelve={}", clock_point);
    canvas.write_pixel(clock_point.x as usize, clock_point.z as usize, red);
    let rotation = rotate_y(PI / 6.0);
    println!("{}", rotation);
    let mut rotated = rotation.mul_by_tuple(&clock_point.to_vector()); // 1, 0, 0
    for i in 0..2 {
        println!("rotated={}", rotated);
        if rotated.z < 0.0 {
            rotated.z = canvas.height as f32 + rotated.z - clock_radius;
            println!("casted to {}", rotated.z);
        } else {
            rotated.z = rotated.z - clock_radius;
        }
        canvas.write_pixel(rotated.x as usize, rotated.z as usize, red);
        rotated = rotation.mul_by_tuple(&rotated.to_vector());
    }
    canvas.to_ppm();
}

fn main() {
    clock_ppm();
}
