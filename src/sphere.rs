use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
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

fn get_new_record(ray: Ray, center: Point3, radius: f32, temp: f32) -> Option<HitRecord> {
    let mut new_hit_record = HitRecord::new_def();
    new_hit_record.t = temp;
    new_hit_record.point = ray.at(temp);
    let outward_normal = (new_hit_record.point - center) / radius;
    return Some(
        HitRecord::with_normal(
            new_hit_record,
            ray,
            outward_normal,
        )
    );
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &HitRecord) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_square();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_square() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                return get_new_record(ray, self.center, self.radius, temp);
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return get_new_record(ray, self.center, self.radius, temp);
            }
        }
        return None;
    }
}