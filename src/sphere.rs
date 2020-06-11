use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_square();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_square() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            for i in 0..3 {
                if i == 2 {
                    return false;
                }
                if temp >= t_max || temp <= t_min {
                    temp = (-half_b - root) / a;
                }
            }
            hit_record.t = temp;
            hit_record.point = ray.at(hit_record.t);
            let outward_normal = (hit_record.point - self.center) / self.radius;
            hit_record.with_normal(ray, outward_normal);
            return true;
        }
        return false;
    }
}