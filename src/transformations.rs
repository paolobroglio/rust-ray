use std::f32;

use crate::matrix::Matrix;

const PI: f32 = f32::consts::PI;

pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
    let mut identity = Matrix::new_identity(4);
    identity.vector2[0][3] = x;
    identity.vector2[1][3] = y;
    identity.vector2[2][3] = z;
    identity
}

pub fn scale(x: f32, y: f32, z: f32) -> Matrix {
    let mut identity = Matrix::new_identity(4);
    identity.vector2[0][0] = x;
    identity.vector2[1][1] = y;
    identity.vector2[2][2] = z;
    identity
}

pub fn rotate_x(r: f32) -> Matrix {
    let mut identity = Matrix::new_identity(4);
    identity.vector2[1][1] = r.cos();
    identity.vector2[1][2] = -r.sin();
    identity.vector2[2][1] = r.sin();
    identity.vector2[2][2] = -r.cos();
    identity
}

pub fn rotate_y(r: f32) -> Matrix {
    println!("radius: {}", r);
    let mut identity = Matrix::new_identity(4);
    identity.vector2[0][0] = r.cos().round();
    identity.vector2[0][2] = r.sin().round();
    identity.vector2[2][0] = -r.sin().round();
    identity.vector2[2][2] = r.cos().round();
    identity
}

pub fn rotate_z(r: f32) -> Matrix {
    let mut identity = Matrix::new_identity(4);
    identity.vector2[0][0] = r.cos();
    identity.vector2[0][1] = -r.sin();
    identity.vector2[1][0] = r.sin();
    identity.vector2[1][1] = r.cos();
    identity
}

pub fn shear(x_x: f32, x_y: f32, y_x: f32, y_y: f32, z_x: f32, z_y: f32) -> Matrix {
    let mut identity = Matrix::new_identity(4);
    identity.vector2[0][1] = x_x;
    identity.vector2[0][2] = x_y;
    identity.vector2[1][0] = y_x;
    identity.vector2[1][2] = y_y;
    identity.vector2[2][0] = z_x;
    identity.vector2[2][1] = z_y;
    identity
}


#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn test_seq_transformations() {
        let point = Tuple::new_point(1.0, 0.0, 1.0);
        let rotation = rotate_x(PI / 2.0);
        let scale = scale(5.0, 5.0, 5.0);
        let translation = translate(10.0, 5.0, 7.0);
        let rotated = rotation.mul_by_tuple(&point.to_vector());
        assert_eq!(rotated.x, 1.0);
        assert_eq!(rotated.y, -1.0);
        assert_eq!(rotated.z, 0.0);
        let scaled = scale.mul_by_tuple(&rotated.to_vector());
        assert_eq!(scaled.x, 5.0);
        assert_eq!(scaled.y, -5.0);
        assert_eq!(scaled.z, 0.0);
        let translated = translation.mul_by_tuple(&scaled.to_vector());
        assert_eq!(translated.x, 15.0);
        assert_eq!(translated.y, 0.0);
        assert_eq!(translated.z, 7.0);
    }
}