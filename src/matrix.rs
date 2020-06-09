use core::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Mul;

use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Matrix {
    pub vector2: Vec<Vec<f32>>,
    square: bool,
    m: usize,
    n: usize,
}

impl Matrix {
    pub fn new(n: usize, m: usize, values: Vec<f32>) -> Matrix {
        let mut vector: Vec<Vec<f32>> = Vec::new();
        for i in 0..n {
            let mut row: Vec<f32> = Vec::new();
            for j in 0..m {
                row.push(values[j + i * m]);
            }
            vector.push(row);
        }
        Matrix {
            vector2: vector,
            square: n == m,
            m,
            n,
        }
    }
    pub fn new_identity(size: usize) -> Matrix {
        let mut vector: Vec<Vec<f32>> = Vec::new();
        for i in 0..size {
            let mut row: Vec<f32> = Vec::new();
            for j in 0..size {
                row.push(0.0);
            }
            row[i] = 1.0;
            vector.push(row);
        }
        Matrix {
            vector2: vector,
            m: size,
            n: size,
            square: true,
        }
    }
    pub fn mul_by_tuple(&self, tuple: &Vec<f32>) -> Tuple {
        let mut vector: Vec<f32> = Vec::new();
        for i in 0..self.n {
            let mut sum = 0.0_f32;
            for j in 0..self.m {
                sum += self.vector2[i][j] * tuple[j];
            }
            vector.push(sum);
        }
        Tuple::from_vector(&vector)
    }
    pub fn transpose(&self) -> Matrix {
        let mut transposed: Vec<Vec<f32>> = Vec::new();
        for j in 0..self.m {
            let mut row: Vec<f32> = Vec::new();
            for i in 0..self.n {
                row.push(self.vector2[i][j]);
            }
            transposed.push(row);
        }
        Matrix {
            m: self.m,
            n: self.n,
            vector2: transposed,
            square: self.m == self.n,
        }
    }
    fn determinant2(&self) -> f32 {
        self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0)
    }
    pub fn determinant(&self) -> f32 {
        if self.n == 2 && self.m == 2 {
            return self.determinant2();
        }
        let mut det = 0.0;
        for j in 0..self.m {
            det += self.at(0, j) * self.cofactor(0, j);
        }
        det
    }
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix {
        let mut vector: Vec<Vec<f32>> = Vec::new();
        for i in 0..self.n {
            let mut row_vec: Vec<f32> = Vec::new();
            if i == row {
                continue;
            }
            for j in 0..self.m {
                if j == column {
                    continue;
                }
                row_vec.push(self.vector2[i][j]);
            }
            vector.push(row_vec);
        }
        Matrix {
            n: self.n - 1,
            m: self.m - 1,
            vector2: vector,
            square: self.n == self.m,
        }
    }
    pub fn minor(&self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }
    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 { minor } else { -minor }
    }
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
    pub fn invert(&self) -> Matrix {
        let mut vector: Vec<Vec<f32>> = Vec::new();
        let det = self.determinant();
        for i in 0..self.n {
            let mut row: Vec<f32> = Vec::new();
            for j in 0..self.m {
                let value = self.cofactor(i, j) / det;
                row.push(value);
            }
            vector.push(row);
        }
        Matrix {
            vector2: vector,
            square: self.m == self.n,
            m: self.m,
            n: self.n,
        }.transpose()
    }
    pub fn at(&self, x: usize, y: usize) -> f32 {
        self.vector2[x][y]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.vector2 == other.vector2
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Matrix {
        Matrix {
            vector2: self.vector2.clone(),
            m: self.m,
            n: self.n,
            square: self.square,
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_vector: Vec<Vec<f32>> = Vec::new();
        for i in 0..self.n {
            let mut new_row: Vec<f32> = Vec::new();
            for j in 0..self.m {
                let mut sum = 0.0_f32;
                let mut m_col: Vec<f32> = Vec::new();
                for r in 0..self.n {
                    m_col.push(rhs.vector2[r][j]);
                }
                for idx in 0..self.m {
                    sum += m_col[idx] * self.vector2[i][idx]
                }
                new_row.push(sum);
            }
            new_vector.push(new_row);
        }
        Matrix {
            vector2: new_vector,
            m: self.m,
            n: self.n,
            square: true,
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push('[');
        for i in 0..self.n {
            res.push('[');
            for j in 0..self.m {
                res.push_str(self.at(i, j).to_string().as_str());
                if j < self.m - 1 {
                    res.push_str(", ");
                }
            }
            res.push(']');
            if i < self.n - 1 {
                res.push_str(",\n");
            }
        }
        res.push(']');
        write!(f, "{}", res)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_matrix() {
        let matrix = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(matrix.at(0, 0), 1.0);
        assert_eq!(matrix.at(0, 1), 2.0);
        assert_eq!(matrix.at(1, 0), 3.0);
        assert_eq!(matrix.at(1, 1), 4.0);
    }

    #[test]
    fn test_eq() {
        let matrix = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix2 = Matrix::new(2, 2, vec![2.0, 2.0, 3.0, 4.0]);
        assert_eq!(matrix, matrix1);
        assert_ne!(matrix, matrix2);
    }

    #[test]
    fn test_mul() {
        let matrix = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let result = matrix1 * matrix;
        let multiplied = Matrix::new(2, 2, vec![7.0, 10.0, 15.0, 22.0]);
        assert_eq!(result, multiplied);
    }

    #[test]
    fn test_mul_by_tuple() {
        let matrix = Matrix::new(4, 4, vec![1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0,
                                            2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        let tuple = vec![1.0, 2.0, 3.0, 1.0];
        let result = matrix.mul_by_tuple(&tuple);
        assert_eq!(result, Tuple { x: 18.0, y: 24.0, z: 33.0, t: 1.0 });
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix::new(4, 4, vec![
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0]);
        let result = Matrix::new(4, 4, vec![
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0
        ]);
        assert_eq!(result, matrix.transpose());
    }

    #[test]
    fn test_determinant() {
        let matrix2x2 = Matrix::new(2, 2, vec![
            1.0, 5.0,
            -3.0, 2.0
        ]);
        let matrix4x4 = Matrix::new(4, 4, vec![
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0
        ]);
        assert_eq!(matrix2x2.determinant(), 17.0);
        assert_eq!(matrix4x4.determinant(), -4071.0);
    }

    #[test]
    fn test_submatrix() {
        let matrix = Matrix::new(3, 3, vec![
            1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0
        ]);
        let result = Matrix::new(2, 2, vec![
            -3.0, 2.0,
            0.0, 6.0,
        ]);
        let matrix4x4 = Matrix::new(4, 4, vec![
            1.0, 5.0, 0.0, 2.0,
            -3.0, 2.0, 7.0, 2.0,
            0.0, 6.0, -3.0, 2.0,
            1.0, 2.0, 3.0, 4.0
        ]);
        let result3x3 = Matrix::new(3, 3, vec![
            1.0, 0.0, 2.0,
            0.0, -3.0, 2.0,
            1.0, 3.0, 4.0
        ]);
        assert_eq!(result, matrix.submatrix(0, 2));
        assert_eq!(result3x3, matrix4x4.submatrix(1, 1));
    }

    #[test]
    fn test_minor() {
        let matrix3x3 = Matrix::new(3, 3, vec![
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0
        ]);
        assert_eq!(matrix3x3.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactor() {
        let matrix3x3 = Matrix::new(3, 3, vec![
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0
        ]);
        assert_eq!(matrix3x3.cofactor(0, 0), -12.0);
        assert_eq!(matrix3x3.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_is_invertible() {
        let matrix4x4 = Matrix::new(4, 4, vec![
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0
        ]);
        let second_matrix4x4 = Matrix::new(4, 4, vec![
            -4.0, 2.0, -2.0, -3.0,
            9.0, 6.0, 2.0, 6.0,
            0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0
        ]);
        assert_eq!(matrix4x4.is_invertible(), true);
        assert_eq!(second_matrix4x4.is_invertible(), false);
    }

    #[test]
    fn test_invert() {
        let matrix4x4 = Matrix::new(4, 4, vec![
            -5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0
        ]);
        let snd_matrix4x4 = Matrix::new(4, 4, vec![
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0
        ]);
        let res = matrix4x4 * snd_matrix4x4.clone();
        // TODO: add assert
    }

    #[test]
    fn test_identity() {
        let res = Matrix::new_identity(4);
        assert_eq!(res.at(0, 0), 1.0);
        assert_eq!(res.at(1, 1), 1.0);
        assert_eq!(res.at(2, 2), 1.0);
        assert_eq!(res.at(3, 3), 1.0);
    }
}