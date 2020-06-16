use std::fs::File;
use std::io::Write;
use std::rc::Rc;

use indicatif::ProgressBar;

use leonardo_engine::algebra::utility::{random_f32, random_in_range_f32};
use leonardo_engine::algebra::vec3::{Color, Point3, Vec3};
use leonardo_engine::graphics::camera::Camera;
use leonardo_engine::graphics::hit::{HitRecord, Hittable, HittableStore};
use leonardo_engine::graphics::material::{Dielectric, Lambertian, Metal};
use leonardo_engine::graphics::ppm::write_color;
use leonardo_engine::graphics::ray::Ray;
use leonardo_engine::graphics::sphere::Sphere;

fn ray_color(ray: Ray, world: &HittableStore, depth: i32) -> Color {
    let hit_record = HitRecord::new_def();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    return match world.hit(ray, 0.001, f32::MAX, &hit_record) {
        Some(new_hit_record) => {
            let scattered = Ray::new(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            );
            let attenuation = Color::new(0.0, 0.0, 0.0);
            return match new_hit_record.material.as_ref() {
                Some(material) => {
                    let scatter = material.scatter(ray, &new_hit_record, attenuation, scattered);
                    if scatter.is_scattered {
                        return scatter.attenuation * ray_color(scatter.scattered, world, depth - 1);
                    }
                    return Color::new(0.0, 0.0, 0.0);
                }
                None => Color::new(0.0, 0.0, 0.0)
            };
        }
        _ => {
            let unit_direction = Vec3::unit_vector(ray.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    };
}

fn random_scene() -> HittableStore {
    let rnd_gen = rand::thread_rng();
    let mut world = HittableStore::new();
    //let ground = Some(Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))));
    world.store(Box::new(
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Some(Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)))))));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32(rnd_gen);
            let center = Point3::new(
                (random_f32(rnd_gen) * 0.9) + a as f32,
                0.2,
                (random_f32(rnd_gen) * 0.9) + b as f32,
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    world.store(Box::new(
                        Sphere::new(
                            center,
                            0.2,
                            Some(Rc::new(
                                Lambertian::new(albedo)
                            )),
                        )
                    ))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_in_range_f32(rnd_gen, 0.0, 0.5);
                    world.store(Box::new(
                        Sphere::new(
                            center,
                            0.2,
                            Some(Rc::new(
                                Metal::new(albedo, fuzz)
                            )),
                        )
                    ))
                } else {
                    world.store(Box::new(
                        Sphere::new(
                            center,
                            0.2,
                            Some(Rc::new(
                                Dielectric::new(1.5)
                            )),
                        )
                    ))
                }
            }
        }
    }

    world.store(
        Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0,
                             Some(Rc::new(Dielectric::new(1.5))))));
    world.store(
        Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0,
                             Some(Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)))))));
    world.store(
        Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0,
                             Some(Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0))))));
    world
}


fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0_f32;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut content = String::new();
    content.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    let mut world = random_scene();

    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio);

    let rnd_generator = rand::thread_rng();

    let progress_bar = ProgressBar::new((image_width * image_height) as u64);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f32 + random_f32(rnd_generator)) / (image_width - 1) as f32;
                let v = (j as f32 + random_f32(rnd_generator)) / (image_height - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            }
            content.push_str(write_color(pixel_color, samples_per_pixel as f32).as_str());
            progress_bar.inc(1);
        }
    }
    progress_bar.finish();
    let mut file = match File::create("ray.ppm") {
        Ok(file) => file,
        Err(err) => panic!("Could not create file {:?}", err)
    };
    file.write_all(content.as_bytes())?;
    Ok(())
}