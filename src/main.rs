use crate::environment::{Environment, Projectile};
use crate::tuple::Tuple;

mod tuple;
mod environment;
mod canvas;
mod color;


fn main() {
    println!("Hello, world!");
    let environment = Environment::new(
        Tuple::new_vector(0.0, -0.1, -0.01),
        Tuple::new_vector(1.0, 0.0, 0.0),
    );
    let mut projectile = Projectile::new(
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(1.0, 1.0, 0.0).normalize(),
    );
    for _ in 1..10 {
        projectile = environment.tick(&projectile);
        println!("{}", projectile)
    }
}
