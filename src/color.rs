use core::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
    pub fn multiply_by(&self, scalar: f32) -> Color {
        Color {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_color() {
        let result = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(result.b, 1.7);
        assert_eq!(result.r, -0.5);
        assert_eq!(result.g, 0.4);
    }

    #[test]
    fn test_add() {
        let color = Color::new(1.0, 0.0, 0.0);
        let color1 = Color::new(2.0, -1.0, 1.4);
        let expect = Color::new(3.0, -1.0, 1.4);
        assert_eq!(color + color1, expect);
    }

    #[test]
    fn test_sub() {
        let color = Color::new(1.0, 0.0, 0.0);
        let color1 = Color::new(2.0, -1.0, 1.4);
        let expect = Color::new(-1.0, 1.0, -1.4);
        assert_eq!(color - color1, expect);
    }

    #[test]
    fn test_mul() {
        let color = Color::new(1.0, 0.2, 0.4);
        let color1 = Color::new(0.9, 1.0, 0.1);
        let expect = Color::new(0.9, 0.2, 0.040000003);
        assert_eq!(color * color1, expect);
    }


    #[test]
    fn test_multiply_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);
        let expect = Color::new(0.4, 0.6, 0.8);
        assert_eq!(color.multiply_by(2.0), expect);
    }
}