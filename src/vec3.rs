use core::fmt;
use std::f32::consts::PI;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::Rng;

use crate::math::{random_f32, random_in_range_f32};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    pub fn new(e1: f32, e2: f32, e3: f32) -> Vec3 {
        Vec3 {
            e: [e1, e2, e3]
        }
    }
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    pub fn length(&self) -> f32 {
        self.length_square().sqrt()
    }
    pub fn length_square(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x() * other.x()
            + self.y() * other.y()
            + self.z() * other.z()
    }
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x()
            ]
        }
    }
    pub fn unit_vector(vec3: Vec3) -> Vec3 {
        vec3 / vec3.length()
    }
    pub fn random_unit_vector() -> Vec3 {
        let mut rnd = rand::thread_rng();
        let a = random_in_range_f32(rnd, 0.0, 2.0 * PI);
        let z = random_in_range_f32(rnd, -1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3 {
            e: [r * a.cos(), r * a.sin(), z]
        }
    }
    pub fn random() -> Vec3 {
        let mut rnd = rand::thread_rng();
        Vec3 {
            e: [
                random_f32(rnd),
                random_f32(rnd),
                random_f32(rnd)
            ]
        }
    }
    pub fn random_in_range(min: f32, max: f32) -> Vec3 {
        let mut rnd = rand::thread_rng();
        Vec3 {
            e: [
                random_in_range_f32(rnd, min, max),
                random_in_range_f32(rnd, min, max),
                random_in_range_f32(rnd, min, max)
            ]
        }
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_in_range(-1.0, 1.0);
            if p.length_square() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let rnd_in_unit_sphere = Vec3::random_in_unit_sphere();
        if rnd_in_unit_sphere.dot(normal) > 0.0 {
            return rnd_in_unit_sphere;
        }
        return -rnd_in_unit_sphere;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()]
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()]
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()]
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.x() * rhs, self.y() * rhs, self.z() * rhs]
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()]
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.x() / rhs, self.y() / rhs, self.z() / rhs]
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div() {
        let vec = Vec3::new(2.0, 2.0, 2.0);
        println!("{}", vec / 2.0);
    }
}