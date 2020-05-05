use core::fmt;
use std::fmt::{Display, Formatter};

use crate::tuple::Tuple;

#[derive(Copy, Clone)]
pub struct Projectile {
    position: Tuple,
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
            position: projectile.position + self.wind,
            velocity: projectile.velocity + self.gravity + self.wind,
        }
    }
}