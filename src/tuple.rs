use core::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub t: f32,
}

impl Tuple {
    pub fn new_point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, t: 1.0 }
    }
    pub fn new_vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, t: 0.0 }
    }
    pub fn to_vector(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z, self.t]
    }
    pub fn from_vector(vector: &Vec<f32>) -> Tuple {
        Tuple {
            x: vector[0],
            y: vector[1],
            z: vector[2],
            t: vector[3],
        }
    }
    pub fn negate(&self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            t: self.t,
        }
    }
    pub fn multiply_by(&self, multiplier: f32) -> Tuple {
        Tuple {
            x: self.x * multiplier,
            y: self.y * multiplier,
            z: self.z * multiplier,
            t: self.t * multiplier,
        }
    }
    pub fn divide_by(&self, divisor: f32) -> Tuple {
        Tuple {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
            t: self.t / divisor,
        }
    }
    pub fn dot_product(&self, other: &Tuple) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.t * other.t
    }
    pub fn cross_product(&self, other: &Tuple) -> Tuple {
        Tuple::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            t: self.t / magnitude,
        }
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            t: self.t + rhs.t,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            t: self.t - rhs.t,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.t == other.t
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(x={}, y={}, z={}, t={})", self.x, self.y, self.z, self.t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_point() {
        let p = Tuple::new_point(1.0, 2.0, 3.0);
        assert_eq!(1.0, p.x);
        assert_eq!(2.0, p.y);
        assert_eq!(3.0, p.z);
        assert_eq!(1.0, p.t);
    }

    #[test]
    fn test_create_vector() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
        assert_eq!(0.0, v.t);
    }

    #[test]
    fn test_add_tuples() {
        let p = Tuple::new_point(3.0, -2.0, 5.0);
        let v = Tuple::new_vector(-2.0, 3.0, 1.0);
        let result = Tuple { x: 1.0, y: 1.0, z: 6.0, t: 1.0 };
        assert_eq!(v + p, result);
    }

    #[test]
    fn test_sub_tuples() {
        let p = Tuple::new_point(3.0, 2.0, 1.0);
        let p1 = Tuple::new_point(5.0, 6.0, 7.0);
        let result = Tuple { x: -2.0, y: -4.0, z: -6.0, t: 0.0 };
        assert_eq!(p - p1, result);
    }

    #[test]
    fn test_negate_vector() {
        let v = Tuple::new_vector(1.0, -2.0, 3.0);
        let result = Tuple::new_vector(-1.0, 2.0, -3.0);
        assert_eq!(v.negate(), result);
    }

    #[test]
    fn test_multiply_by() {
        let v = Tuple::new_vector(1.0, -2.0, 3.0);
        let result = Tuple::new_vector(3.5, -7.0, 10.5);
        assert_eq!(v.multiply_by(3.5), result);
    }

    #[test]
    fn test_divide_by() {
        let v = Tuple::new_vector(1.0, -2.0, 3.0);
        let result = Tuple::new_vector(0.5, -1.0, 1.5);
        assert_eq!(v.divide_by(2.0), result);
    }

    #[test]
    fn test_magnitude() {
        let v = Tuple::new_vector(1.0, 0.0, 0.0);
        let v1 = Tuple::new_vector(0.0, 1.0, 0.0);
        let v2 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v3 = Tuple::new_vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 1.0);
        assert_eq!(v1.magnitude(), 1.0);
        assert_eq!(v2.magnitude(), 14.0_f32.sqrt());
        assert_eq!(v3.magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn test_normalize() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        let normalized = v.normalize();
        let expect = Tuple::new_vector(0.26726124, 0.5345225, 0.8017837);
        assert_eq!(normalized, expect);
    }

    #[test]
    fn test_dot_product() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        let v1 = Tuple::new_vector(2.0, 3.0, 4.0);
        let result = v.dot_product(&v1);
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_cross_product() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        let v1 = Tuple::new_vector(2.0, 3.0, 4.0);
        let expect = Tuple::new_vector(-1.0, 2.0, -1.0);
        let expect1 = Tuple::new_vector(1.0, -2.0, 1.0);
        assert_eq!(v.cross_product(&v1), expect);
        assert_eq!(v1.cross_product(&v), expect1);
    }
}
