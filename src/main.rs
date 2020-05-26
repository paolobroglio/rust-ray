use crate::canvas::Canvas;
use crate::color::Color;
use crate::environment::{Environment, Projectile};
use crate::tuple::Tuple;

mod tuple;
mod environment;
mod canvas;
mod color;
mod matrix;

fn main() {
    println!("Hello, world!");
    let environment = Environment::new(
        Tuple::new_vector(0.0, -0.1, 0.0),
        Tuple::new_vector(-0.01, 0.0, 0.0),
    );
    let mut projectile = Projectile::new(
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(1.0, 1.8, 0.0).normalize().multiply_by(11.25),
    );
    let mut canvas = Canvas::new(900, 550);
    let red = Color::new(255.0, 0.0, 0.0);
    for _ in 1..198 {
        projectile = environment.tick(&projectile);
        canvas.write_pixel(projectile.position.x as usize, canvas.height - projectile.position.y as usize, red)
    }
    canvas.to_ppm();
}
