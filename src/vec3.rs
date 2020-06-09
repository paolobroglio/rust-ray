use core::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Debug)]
pub struct Vec3 {
    pub vector: Vec<f64>
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            vector: vec![x, y, z]
        }
    }
    pub fn new_zeroes() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    pub fn x(&self) -> f64 {
        self.vector[0]
    }
    pub fn y(&self) -> f64 {
        self.vector[1]
    }
    pub fn z(&self) -> f64 {
        self.vector[2]
    }
    pub fn length(&self) -> f64 {
        self.length_square().sqrt()
    }
    pub fn length_square(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
    pub fn mul_by(&self, n: f64) -> Vec3 {
        Vec3 {
            vector: vec![self.x() * n, self.y() * n, self.z() * n]
        }
    }
    pub fn div_by(&self, n: f64) -> Vec3 {
        self.mul_by(1.0 / n)
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            vector: vec![
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x()
            ]
        }
    }
    pub fn unit_vector(&self) -> Vec3 {
        self.div_by(self.length())
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            vector: vec![self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()]
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            vector: vec![self.x() + rhs.x(), self.y() + rhs.y(), self.z() * rhs.z()]
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            vector: vec![self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()]
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            vector: vec![-self.x(), -self.y(), -self.z()]
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vector[index]
    }
}


impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3(x={}, y={}, z={})", self.x(), self.y(), self.z())
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3 {
            vector: vec![self.x(), self.y(), self.z()]
        }
    }
}


