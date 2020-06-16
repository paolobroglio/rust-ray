use crate::algebra::utility::degrees_to_radians;
use crate::algebra::vec3::{Point3, Vec3};
use crate::graphics::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0_f32 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(vup.cross(w));
        let v = w.cross(u);

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = look_from - (horizontal / 2.0) - (vertical / 2.0) - w;
        Camera {
            origin: look_from,
            lower_left_corner,
            horizontal: u * viewport_width,
            vertical: v * viewport_height,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin)
    }
}