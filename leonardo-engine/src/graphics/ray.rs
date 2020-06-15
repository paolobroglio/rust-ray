use crate::algebra::vec3::{Point3, Vec3};

#[derive(Copy, Clone, Debug)]
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
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn origin(&self) -> Point3 {
        self.origin
    }
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}