use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f32, front_face: bool) -> HitRecord {
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
    pub fn with_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        let front_face = outward_normal.dot(ray.direction()) < 0.0;
        self.normal = if front_face { outward_normal } else { -outward_normal };
        self.front_face = front_face;
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}