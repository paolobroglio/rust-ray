use core::fmt;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io;
use std::io::Write;

use indicatif::ProgressBar;

use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

pub struct PpmImage {
    pub height: usize,
    pub width: usize,
    pub map: Vec<Vec<Color>>,
}

impl PpmImage {
    pub fn new(height: usize, width: usize) -> PpmImage {
        let mut pixels: Vec<Vec<Color>> = Vec::new();
        for _i in 0..height {
            let mut row: Vec<Color> = Vec::new();
            for _j in 0..width {
                row.push(Color::new(0.0, 0.0, 0.0));
            }
            pixels.push(row);
        }
        PpmImage {
            height,
            width,
            map: pixels,
        }
    }
    pub fn draw(&mut self, origin: &Point3, horizontal: &Vec3, vertical: &Vec3, ll_corner: &Point3) {
        let mut pixels: Vec<Vec<Color>> = Vec::new();
        for i in 0..self.height {
            let mut row: Vec<Color> = Vec::new();
            for j in 0..self.width {
                let u = (i as f64) / (self.width - 1) as f64;
                let v = (j as f64) / (self.height - 1) as f64;
                let ray = Ray::new(
                    origin.clone(),
                    ll_corner.clone() + horizontal.mul_by(u) + vertical.mul_by(v) - origin.clone(),
                );
                row.push(ray.color());
            }
            pixels.push(row);
        }
        self.map = pixels;
    }
    pub fn write(&self) -> Result<String, io::Error> {
        let file = File::create("canvas.ppm");
        let mut file = match file {
            Ok(file) => file,
            Err(e) => return Err(e)
        };
        let content = format!("P3\n{} {}\n255\n{}", self.width, self.height, self);
        match file.write(content.as_bytes()) {
            Ok(_) => Ok(String::from("Content written!")),
            Err(e) => return Err(e)
        }
    }
}

impl Display for PpmImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut content = String::new();
        let progress = ProgressBar::new((self.width * self.height) as u64);
        progress.println("Writing PPM image");
        for i in 0..self.height {
            for j in 0..self.width {
                let pixel = &self.map[i][j];
                let r = 255.999 * pixel.x();
                let g = 255.999 * pixel.y();
                let b = 255.999 * pixel.z();
                content.push_str(format!("{} {} {} ", r as u64, g as u64, b as u64).as_str());
                progress.inc(1);
            }
            content.push('\n');
        }
        progress.finish();
        write!(f, "{}", content)
    }
}

