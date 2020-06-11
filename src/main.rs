use std::fs::File;
use std::io::Write;

use crate::ppm::write_color;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

mod ray;
mod ppm;
mod vec3;
mod hit;
mod sphere;

fn hit_sphere(center: Point3, radius: f32, ray: Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let disc = b * b - 4.0 * a * c;
    return if disc < 0.0 {
        -1.0
    } else {
        (-b - disc.sqrt()) / (2.0 * a)
    }
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = Vec3::unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}


fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0_f32;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let mut content = String::new();
    content.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    let viewport_height = 2.0_f32;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0_f32;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = (image_height - 1 - j) as f32 / (image_height - 1) as f32;
            let ray = Ray::new(origin, lower_left_corner + (horizontal * u) + (vertical * v) - origin);
            content.push_str(write_color(ray_color(ray)).as_str());
        }
    }

    let mut file = match File::create("ray.ppm") {
        Ok(file) => file,
        Err(err) => panic!("Could not create file {:?}", err)
    };
    file.write_all(content.as_bytes())?;
    Ok(())
}