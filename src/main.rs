use std::fs::File;
use std::io::Write;

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::camera::Camera;
use crate::hit::{HitRecord, Hittable, HittableStore};
use crate::math::random_f32;
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
mod math;

fn ray_color(ray: Ray, world: &HittableStore, depth: i32) -> Color {
    let hit_record = HitRecord::new_def();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    return match world.hit(ray, 0.001, f32::MAX, &hit_record) {
        Some(new_hit_record) => {
            let target = new_hit_record.point + Vec3::random_in_hemisphere(new_hit_record.normal);
            return ray_color(
                Ray::new(
                    new_hit_record.point,
                    target - new_hit_record.point),
                world,
                depth - 1) * 0.5;
        }
        _ => {
            let unit_direction = Vec3::unit_vector(ray.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    };
}


fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0_f32;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut content = String::new();
    content.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    let mut world = HittableStore::new();
    world.store(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.store(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    let camera = Camera::new();

    let mut rnd_generator = rand::thread_rng();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f32 + random_f32(rnd_generator)) / (image_width - 1) as f32;
                let v = (j as f32 + random_f32(rnd_generator)) / (image_height - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            }
            content.push_str(write_color(pixel_color, samples_per_pixel as f32).as_str());
        }
    }

    let mut file = match File::create("ray.ppm") {
        Ok(file) => file,
        Err(err) => panic!("Could not create file {:?}", err)
    };
    file.write_all(content.as_bytes())?;
    Ok(())
}