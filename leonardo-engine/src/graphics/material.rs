use crate::algebra::utility::random_f32;
use crate::algebra::vec3::{Color, Vec3};
use crate::graphics::hit::HitRecord;
use crate::graphics::ray::Ray;

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
    pub is_scattered: bool,
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord, attenuation: Color, scattered: Ray) -> Scatter;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord, attenuation: Color, scattered: Ray) -> Scatter {
        let scatter_dir = hit_record.normal + Vec3::random_unit_vector();
        let scattered_ray = Ray::new(hit_record.point, scatter_dir);
        return Scatter {
            scattered: scattered_ray,
            attenuation: self.albedo,
            is_scattered: true,
        };
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord, attenuation: Color, scattered: Ray) -> Scatter {
        let reflected = reflect(Vec3::unit_vector(ray.direction()), hit_record.normal);
        let scattered_ray = Ray::new(hit_record.point, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        let is_scattered = scattered_ray.direction().dot(hit_record.normal) > 0.0;
        return Scatter {
            attenuation: self.albedo,
            scattered: scattered_ray,
            is_scattered,
        };
    }
}

pub fn reflect(uvec: Vec3, nvec: Vec3) -> Vec3 {
    uvec - (nvec * 2.0 * uvec.dot(nvec))
}

pub fn refract(uvec: Vec3, nvec: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uvec).dot(nvec);
    let r_out_parallel = (uvec + (nvec * cos_theta)) * etai_over_etat;
    let r_out_perp = nvec * (-(-r_out_parallel.length_square() + 1.0).sqrt());
    r_out_parallel + r_out_perp
}

pub fn schlick_approx(cos: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

pub struct Dielectric {
    pub ref_idx: f32
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

fn get_reflected(unit_direction: Vec3, hit_record: &HitRecord) -> Scatter {
    let reflected = reflect(unit_direction, hit_record.normal);
    Scatter {
        scattered: Ray::new(hit_record.point, reflected),
        attenuation: Color::new(1.0, 1.0, 1.0),
        is_scattered: true,
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord, attenuation: Color, scattered: Ray) -> Scatter {
        let etai_over_etat = if hit_record.front_face { 1.0 / self.ref_idx } else { self.ref_idx };
        let unit_direction = Vec3::unit_vector(ray.direction());
        let cos_theta = 1.0_f32.min((-unit_direction).dot(hit_record.normal));
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            return get_reflected(unit_direction, hit_record);
        }
        let reflect_prob = schlick_approx(cos_theta, etai_over_etat);
        if random_f32(rand::thread_rng()) < reflect_prob {
            return get_reflected(unit_direction, hit_record);
        }

        let refracted = refract(unit_direction, hit_record.normal, etai_over_etat);
        return Scatter {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new(hit_record.point, refracted),
            is_scattered: true,
        };
    }
}