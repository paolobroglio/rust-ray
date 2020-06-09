use crate::vec3::{Color, Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }
    pub fn new_zeroes() -> Ray {
        Ray::new(Point3::new_zeroes(), Vec3::new_zeroes())
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + self.direction.mul_by(t)
    }
    pub fn color(&self) -> Color {
        let unit_vector = self.direction.unit_vector();
        let t = 0.5 * (unit_vector.y() + 1.0);
        Color::new(1.0, 1.0, 1.0).mul_by(1.0 - t) +
            Color::new(0.5, 0.7, 1.0).mul_by(t)
    }
}