use core::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Write;

use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut pixels: Vec<Vec<Color>> = Vec::new();
        for _i in 0..height {
            let mut line: Vec<Color> = Vec::new();
            for _j in 0..width {
                line.push(Color::new(0.0, 0.0, 0.0));
            }
            pixels.push(line);
        }
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        println!("Updating pixel at x={} y={}", x, y);
        if x < self.width && y < self.height {
            self.pixels[y][x] = color;
        }
    }

    pub fn to_ppm(&self) -> Result<String, io::Error> {
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

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut content = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
                let pixel = &self.pixels[i][j];
                content.push_str(format!("{} {} {} ", pixel.r, pixel.g, pixel.b).as_str());
            }
            content.push('\n');
        }
        write!(f, "{}", content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppm() {
        let canvas = Canvas::new(3, 3);
        let result = canvas.to_ppm();
        assert_eq!(result.is_ok(), true)
    }
}