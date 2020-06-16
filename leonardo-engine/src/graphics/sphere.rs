use std::rc::Rc;

use crate::algebra::vec3::Point3;
use crate::graphics::hit::{HitRecord, Hittable};
use crate::graphics::material::Material;
use crate::graphics::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Option<Rc<dyn Material>>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
    fn compute_normal(&self, ray: Ray, temp: f32, t_max: f32, t_min: f32) -> Option<HitRecord> {
        if temp < t_max && temp > t_min {
            return get_new_record(ray, self.center, self.radius, temp, self.material.clone());
        }
        None
    }
}

fn get_new_record(ray: Ray, center: Point3, radius: f32, temp: f32, material: Option<Rc<dyn Material>>) -> Option<HitRecord> {
    let mut new_hit_record = HitRecord::new_def();
    new_hit_record.t = temp;
    new_hit_record.point = ray.at(temp);
    new_hit_record.material = material;
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
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, _hit_record: &HitRecord) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_square();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_square() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            return match self.compute_normal(ray, temp, t_max, t_min) {
                Some(hrec) => Some(hrec),
                None => {
                    temp = (-half_b + root) / a;
                    self.compute_normal(ray, temp, t_max, t_min)
                }
            };
        }
        return None;
    }
}