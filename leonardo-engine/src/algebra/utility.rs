use std::f32::consts::PI;

use rand::Rng;
use rand::rngs::ThreadRng;

pub fn random_f32(mut thread_rng: ThreadRng) -> f32 {
    thread_rng.gen::<f32>()
}

pub fn random_in_range_f32(mut thread_rng: ThreadRng, min: f32, max: f32) -> f32 {
    thread_rng.gen_range(min, max)
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}