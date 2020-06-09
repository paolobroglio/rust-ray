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