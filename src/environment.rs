use core::fmt;
use std::fmt::{Display, Formatter};

use crate::tuple::Tuple;

#[derive(Copy, Clone)]
pub struct Projectile {
    pub position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Projectile {
        Projectile { position, velocity }
    }
}

impl Display for Projectile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(position={}, velocity={})", self.position, self.velocity)
    }
}

pub struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Environment {
        Environment { gravity, wind }
    }
    pub fn tick(&self, projectile: &Projectile) -> Projectile {
        Projectile {
            position: projectile.position + projectile.velocity,
            velocity: projectile.velocity + self.gravity + self.wind,
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(gravity={}, wind={})", self.gravity, self.wind)
    }
}