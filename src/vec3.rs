use core::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

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
    pub fn new_zeroes() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
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