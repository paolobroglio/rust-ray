use crate::algebra::vec3::{Point3, Vec3};
use crate::graphics::material::Material;
use crate::graphics::ray::Ray;
use std::rc::Rc;

pub struct HittableStore {
    store: Vec<Box<dyn Hittable>>
}

impl HittableStore {
    pub fn new() -> HittableStore {
        HittableStore {
            store: Vec::new()
        }
    }
    pub fn store(&mut self, hittable: Box<dyn Hittable>) {
        self.store.push(hittable);
    }
    pub fn clear(&mut self) {
        self.store.clear();
    }
}

impl Hittable for HittableStore {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, _hit_record: &HitRecord) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::new_def();
        let mut hit_anything = false;
        for item in self.store.iter() {
            match item.hit(ray, t_min, t_max, &temp_rec) {
                Some(new_hit_record) => {
                    hit_anything = true;
                    temp_rec = new_hit_record;
                }
                _ => continue
            }
        }
        if hit_anything {
            return Some(temp_rec);
        }
        return None;
    }
}


pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn new_def() -> HitRecord {
        HitRecord::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            false,
            Option::None,
        )
    }
    pub fn new(point: Point3, normal: Vec3, t: f32, front_face: bool, material: Option<Rc<dyn Material>>) -> HitRecord {
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
    pub fn with_normal(hit_record: HitRecord, ray: Ray, outward_normal: Vec3) -> HitRecord {
        let front_face = outward_normal.dot(ray.direction()) < 0.0;
        HitRecord {
            point: hit_record.point,
            normal: if front_face { outward_normal } else { -outward_normal },
            t: hit_record.t,
            front_face,
            material: hit_record.material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &HitRecord) -> Option<HitRecord>;
}