use crate::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }
    pub fn new_zeroes() -> Ray {
        Ray {
            origin: Point3::new_zeroes(),
            direction: Vec3::new_zeroes(),
        }
    }
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}