#[derive(Debug)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    t: f32,
}

impl Tuple {
    fn new_point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, t: 1.0 }
    }
    fn new_vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, t: 0.0 }
    }
    fn add(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            t: self.t + other.t,
        }
    }
    fn sub(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            t: self.t - other.t,
        }
    }
    fn negate(&self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            t: self.t,
        }
    }
    fn multiply_by(&self, multiplier: f32) -> Tuple {
        Tuple {
            x: self.x * multiplier,
            y: self.y * multiplier,
            z: self.z * multiplier,
            t: self.t * multiplier,
        }
    }
    fn divide_by(&self, divisor: f32) -> Tuple {
        Tuple {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
            t: self.t / divisor,
        }
    }
    fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.t == other.t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_point() {
        let p = Tuple::new_point(1.0, 2.0, 3.0);
        assert_eq!(1.0, p.x);
        assert_eq!(2.0, p.y);
        assert_eq!(3.0, p.z);
        assert_eq!(1.0, p.t);
    }

    #[test]
    fn test_create_vector() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
        assert_eq!(0.0, v.t);
    }

    #[test]
    fn test_add_tuples() {
        let p = Tuple::new_point(3.0, -2.0, 5.0);
        let v = Tuple::new_vector(-2.0, 3.0, 1.0);
        let result = Tuple { x: 1.0, y: 1.0, z: 6.0, t: 1.0 };
        assert_eq!(v.add(&p), result);
    }

    #[test]
    fn test_sub_tuples() {
        let p = Tuple::new_point(3.0, 2.0, 1.0);
        let p1 = Tuple::new_point(5.0, 6.0, 7.0);
        let result = Tuple { x: -2.0, y: -4.0, z: -6.0, t: 0.0 };
        assert_eq!(p.sub(&p1), result);
    }

    #[test]
    fn test_negate_vector() {
        let v = Tuple::new_vector(1.0, -2.0, 3.0);
        let result = Tuple::new_vector(-1.0, 2.0, -3.0);
        assert_eq!(v.negate(), result);
    }

    #[test]
    fn test_multiply_by() {
        let v = Tuple::new_vector(1.0, -2.0, 3.0);
        let result = Tuple::new_vector(3.5, -7.0, 10.5);
        assert_eq!(v.multiply_by(3.5), result);
    }

    #[test]
    fn test_divide_by() {
        let v = Tuple::new_vector(1.0, -2.0, 3.0);
        let result = Tuple::new_vector(0.5, -1.0, 1.5);
        assert_eq!(v.divide_by(2.0), result);
    }

    #[test]
    fn test_magnitude() {
        let v = Tuple::new_vector(1.0, 0.0, 0.0);
        let v1 = Tuple::new_vector(0.0, 1.0, 0.0);
        let v2 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v3 = Tuple::new_vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 1.0);
        assert_eq!(v1.magnitude(), 1.0);
        assert_eq!(v2.magnitude(), 14.0_f32.sqrt());
        assert_eq!(v3.magnitude(), 14.0_f32.sqrt());
    }
}


fn main() {
    println!("Hello, world!");
}
