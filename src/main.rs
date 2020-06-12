use std::fs::File;
use std::io::Write;

use crate::camera::Camera;
use crate::hit::{HitRecord, Hittable, HittableStore};
use crate::ppm::write_color;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};

mod ray;
mod ppm;
mod vec3;
mod hit;
mod sphere;
mod camera;

fn ray_color(ray: Ray, world: &HittableStore) -> Color {
    let hit_record = HitRecord::new_def();
    return match world.hit(ray, 0.0, f32::MAX, &hit_record) {
        Some(hit_record) => (hit_record.normal + Color::new(1.0, 1.0, 1.0)) * 0.5_f32,
        _ => {
            let unit_direction = Vec3::unit_vector(ray.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    };
}

fn random_f32() -> f32 {
    rand::random::<f32>()
}


fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0_f32;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    let mut content = String::new();
    content.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    let mut world = HittableStore::new();
    world.store(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.store(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    let camera = Camera::new();

    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f32 + random_f32()) / (image_width - 1) as f32;
                let v = (image_height - 1 - j) as f32 + random_f32() / (image_height - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world);
            }
            content.push_str(write_color(pixel_color, samples_per_pixel).as_str());
        }
    }

    let mut file = match File::create("ray.ppm") {
        Ok(file) => file,
        Err(err) => panic!("Could not create file {:?}", err)
    };
    file.write_all(content.as_bytes())?;
    Ok(())
}