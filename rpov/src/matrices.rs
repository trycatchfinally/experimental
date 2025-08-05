#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix<T, const N: usize> {
    data: [[T; N]; N],
}

impl<T: Copy, const N: usize> Matrix<T, N> {
    pub fn from(data: [[T; N]; N]) -> Self {
        Matrix { data }
    }
}

impl<T, const N: usize> std::ops::Index<(usize, usize)> for Matrix<T, N> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}
pub type Matrix2<T> = Matrix<T, 2>;
pub type Matrix3<T> = Matrix<T, 3>;
pub type Matrix4<T> = Matrix<T, 4>;

use std::{
    iter::zip,
    ops::{Add, Mul, Sub},
};

use crate::Tuple4;

fn dot_product<T, const N: usize>(a: &[T; N], b: &[T; 4]) -> T
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Default,
{
    zip(a.iter(), b.iter())
        .map(|(x, y)| *x * *y)
        .fold(T::default(), |acc, x| acc + x)
}

impl<T, const N: usize> Matrix<T, N>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Default,
{
    pub fn multiply_tuple_dot(&self, other: &Tuple4<T>) -> Tuple4<T> {
        let t: [T; 4] = [other.x, other.y, other.z, other.w];

        Tuple4 {
            x: dot_product(&self.data[0], &t),
            y: dot_product(&self.data[1], &t),
            z: dot_product(&self.data[2], &t),
            w: dot_product(&self.data[3], &t),
        }
    }
    pub fn multiply_tuple(&self, other: &Tuple4<T>) -> Tuple4<T> {
        let t = [other.x, other.y, other.z, other.w];
        let mut r = [T::default(), T::default(), T::default(), T::default()];
        for i in 0..N {
            let mut acc = T::default();
            // for j in 0..N {
            for (j, t_value) in t.iter().enumerate().take(N) {
                acc = acc + self.data[i][j] * (*t_value);
            }
            r[i] = acc;
        }
        Tuple4 {
            x: r[0],
            y: r[1],
            z: r[2],
            w: r[3],
        }
    }
    pub fn multiply_matrix(&self, other: &Matrix<T, N>) -> Matrix<T, N> {
        let mut result = [[T::default(); N]; N];
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    result[i][j] = result[i][j] + self.data[i][k] * other.data[k][j];
                }
            }
        }
        Matrix { data: result }
    }
}

// Implement operator overloading
impl<T, const N: usize> std::ops::Mul<Matrix<T, N>> for Matrix<T, N>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Default,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_matrix(&rhs)
    }
}

impl<T, const N: usize> std::ops::Mul<Tuple4<T>> for Matrix<T, N>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Default,
{
    type Output = Tuple4<T>;

    fn mul(self, rhs: Tuple4<T>) -> Self::Output {
        self.multiply_tuple(&rhs)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    /*
    Scenario: Constructing and inspecting a 4x4 matrix
      Given the following 4x4 matrix M:
        |  1   |  2   |  3   |  4   |
        |  5.5 |  6.5 |  7.5 |  8.5 |
        |  9   | 10   | 11   | 12   |
        | 13.5 | 14.5 | 15.5 | 16.5 |
      Then M[0,0] = 1
        And M[0,3] = 4
        And M[1,0] = 5.5
        And M[1,2] = 7.5
        And M[2,2] = 11
        And M[3,0] = 13.5
        And M[3,2] = 15.5
    */
    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    /*
    Scenario: A 2x2 matrix ought to be representable
      Given the following 2x2 matrix M:
        | -3 |  5 |
        |  1 | -2 |
      Then M[0,0] = -3
        And M[0,1] = 5
        And M[1,0] = 1
        And M[1,1] = -2
    */
    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m = Matrix2::from([[-3.0, 5.0], [1.0, -2.0]]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);
    }

    /*
    Scenario: A 3x3 matrix ought to be representable
      Given the following 3x3 matrix M:
        | -3 |  5 |  0 |
        |  1 | -2 | -7 |
        |  0 |  1 |  1 |
      Then M[0,0] = -3
        And M[1,1] = -2
        And M[2,2] = 1
    */
    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m = Matrix3::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }

    /*
    Scenario: Matrix equality with identical matrices
      Given the following matrix A:
          | 1 | 2 | 3 | 4 |
          | 5 | 6 | 7 | 8 |
          | 9 | 8 | 7 | 6 |
          | 5 | 4 | 3 | 2 |
        And the following matrix B:
          | 1 | 2 | 3 | 4 |
          | 5 | 6 | 7 | 8 |
          | 9 | 8 | 7 | 6 |
          | 5 | 4 | 3 | 2 |
      Then A = B
    */
    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(a, b);
    }

    /*
    Scenario: Matrix equality with different matrices
      Given the following matrix A:
          | 1 | 2 | 3 | 4 |
          | 5 | 6 | 7 | 8 |
          | 9 | 8 | 7 | 6 |
          | 5 | 4 | 3 | 2 |
        And the following matrix B:
          | 2 | 3 | 4 | 5 |
          | 6 | 7 | 8 | 9 |
          | 8 | 7 | 6 | 5 |
          | 4 | 3 | 2 | 1 |
      Then A != B
    */
    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::from([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(a, b);
    }

    /*
    Scenario: Multiplying two matrices
      Given the following matrix A:
          | 1 | 2 | 3 | 4 |
          | 5 | 6 | 7 | 8 |
          | 9 | 8 | 7 | 6 |
          | 5 | 4 | 3 | 2 |
        And the following matrix B:
          | -2 | 1 | 2 |  3 |
          |  3 | 2 | 1 | -1 |
          |  4 | 3 | 6 |  5 |
          |  1 | 2 | 7 |  8 |
      Then A * B is the following 4x4 matrix:
          | 20|  22 |  50 |  48 |
          | 44|  54 | 114 | 108 |
          | 40|  58 | 110 | 102 |
          | 16|  26 |  46 |  42 |
    */
    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let expected = Matrix4::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(a * b, expected);
    }

    /*
    Scenario: A matrix multiplied by a tuple
      Given the following matrix A:
          | 1 | 2 | 3 | 4 |
          | 2 | 4 | 4 | 2 |
          | 8 | 6 | 4 | 1 |
          | 0 | 0 | 0 | 1 |
        And b ← tuple(1, 2, 3, 1)
      Then A * b = tuple(18, 24, 33, 1)
    */
    #[test]
    fn matrix_multiplied_by_tuple() {
        let a = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple4::new(1.0, 2.0, 3.0, 1.0);
        let expected = Tuple4::new(18.0, 24.0, 33.0, 1.0);
        let r = a * b;
        assert_eq!(r, expected);
    }
    #[test]
    fn matrix_multiplied_by_tuple1() {
        let a = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 8.0, 16.0],
            [3.0, 6.0, 9.0, 12.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let b = Tuple4::new(1.0, 1.0, 1.0, 1.0);
        let expected = Tuple4::new(10.0, 30.0, 30.0, 4.0 + 8.0 + 16.0 + 32.0);
        let r = a * b;
        assert_eq!(r, expected);
    }
}

#[cfg(false)]
mod unimplemented_tests {

    /*
    Scenario: Multiplying a matrix by the identity matrix
      Given the following matrix A:
        | 0 | 1 |  2 |  4 |
        | 1 | 2 |  4 |  8 |
        | 2 | 4 |  8 | 16 |
        | 4 | 8 | 16 | 32 |
      Then A * identity_matrix = A
    */
    #[test]
    fn multiplying_by_identity_matrix() {
        let a = Matrix4::from([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let identity = Matrix4::identity();
        assert_eq!(a * identity, a);
    }

    /*
    Scenario: Multiplying the identity matrix by a tuple
      Given a ← tuple(1, 2, 3, 4)
      Then identity_matrix * a = a
    */
    #[test]
    fn identity_matrix_multiplied_by_tuple() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let identity = Matrix4::identity();
        assert_eq!(identity * a, a);
    }

    /*
    Scenario: Transposing a matrix
      Given the following matrix A:
        | 0 | 9 | 3 | 0 |
        | 9 | 8 | 0 | 8 |
        | 1 | 8 | 5 | 3 |
        | 0 | 0 | 5 | 8 |
      Then transpose(A) is the following matrix:
        | 0 | 9 | 1 | 0 |
        | 9 | 8 | 8 | 0 |
        | 3 | 0 | 5 | 5 |
        | 0 | 8 | 3 | 8 |
    */
    #[test]
    fn transposing_a_matrix() {
        let a = Matrix4::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let expected = Matrix4::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(a.transpose(), expected);
    }

    /*
    Scenario: Transposing the identity matrix
      Given A ← transpose(identity_matrix)
      Then A = identity_matrix
    */
    #[test]
    fn transposing_identity_matrix() {
        let identity = Matrix4::identity();
        assert_eq!(identity.transpose(), identity);
    }

    /*
    Scenario: Calculating the determinant of a 2x2 matrix
        Given the following 2x2 matrix A:
        |  1 | 5 |
        | -3 | 2 |
        Then determinant(A) = 17
    */
    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let a = Matrix2::from([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(a.determinant(), 17.0);
    }

    /*
    Scenario: A submatrix of a 3x3 matrix is a 2x2 matrix
        Given the following 3x3 matrix A:
        |  1 | 5 |  0 |
        | -3 | 2 |  7 |
        |  0 | 6 | -3 |
        Then submatrix(A, 0, 2) is the following 2x2 matrix:
        | -3 | 2 |
        |  0 | 6 |
    */
    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let a = Matrix3::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let expected = Matrix2::from([[-3.0, 2.0], [0.0, 6.0]]);
        assert_eq!(a.submatrix(0, 2), expected);
    }

    /*
    Scenario: A submatrix of a 4x4 matrix is a 3x3 matrix
        Given the following 4x4 matrix A:
        | -6 |  1 |  1 |  6 |
        | -8 |  5 |  8 |  6 |
        | -1 |  0 |  8 |  2 |
        | -7 |  1 | -1 |  1 |
        Then submatrix(A, 2, 1) is the following 3x3 matrix:
        | -6 |  1 | 6 |
        | -8 |  8 | 6 |
        | -7 | -1 | 1 |
    */
    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let a = Matrix4::from([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let expected = Matrix3::from([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        assert_eq!(a.submatrix(2, 1), expected);
    }

    /*
    Scenario: Calculating a minor of a 3x3 matrix
        Given the following 3x3 matrix A:
            |  3 |  5 |  0 |
            |  2 | -1 | -7 |
            |  6 | -1 |  5 |
        And B ← submatrix(A, 1, 0)
        Then determinant(B) = 25
        And minor(A, 1, 0) = 25
    */
    #[test]
    fn calculating_minor_of_3x3_matrix() {
        let a = Matrix3::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    /*
    Scenario: Calculating a cofactor of a 3x3 matrix
        Given the following 3x3 matrix A:
            |  3 |  5 |  0 |
            |  2 | -1 | -7 |
            |  6 | -1 |  5 |
        Then minor(A, 0, 0) = -12
        And cofactor(A, 0, 0) = -12
        And minor(A, 1, 0) = 25
        And cofactor(A, 1, 0) = -25
    */
    #[test]
    fn calculating_cofactor_of_3x3_matrix() {
        let a = Matrix3::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    /*
    Scenario: Calculating the determinant of a 3x3 matrix
        Given the following 3x3 matrix A:
        |  1 |  2 |  6 |
        | -5 |  8 | -4 |
        |  2 |  6 |  4 |
        Then cofactor(A, 0, 0) = 56
        And cofactor(A, 0, 1) = 12
        And cofactor(A, 0, 2) = -46
        And determinant(A) = -196
    */
    #[test]
    fn calculating_determinant_of_3x3_matrix() {
        let a = Matrix3::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    /*
    Scenario: Calculating the determinant of a 4x4 matrix
        Given the following 4x4 matrix A:
        | -2 | -8 |  3 |  5 |
        | -3 |  1 |  7 |  3 |
        |  1 |  2 | -9 |  6 |
        | -6 |  7 |  7 | -9 |
        Then cofactor(A, 0, 0) = 690
        And cofactor(A, 0, 1) = 447
        And cofactor(A, 0, 2) = 210
        And cofactor(A, 0, 3) = 51
        And determinant(A) = -4071
    */
    #[test]
    fn calculating_determinant_of_4x4_matrix() {
        let a = Matrix4::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    /*
    Scenario: Testing an invertible matrix for invertibility
        Given the following 4x4 matrix A:
        |  6 |  4 |  4 |  4 |
        |  5 |  5 |  7 |  6 |
        |  4 | -9 |  3 | -7 |
        |  9 |  1 |  7 | -6 |
        Then determinant(A) = -2120
        And A is invertible
    */
    #[test]
    fn invertible_matrix_for_invertibility() {
        let a = Matrix4::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }

    /*
    Scenario: Testing a noninvertible matrix for invertibility
        Given the following 4x4 matrix A:
        | -4 |  2 | -2 | -3 |
        |  9 |  6 |  2 |  6 |
        |  0 | -5 |  1 | -5 |
        |  0 |  0 |  0 |  0 |
        Then determinant(A) = 0
        And A is not invertible
    */
    #[test]
    fn noninvertible_matrix_for_invertibility() {
        let a = Matrix4::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    /*
    Scenario: Calculating the inverse of a matrix
        Given the following 4x4 matrix A:
            | -5 |  2 |  6 | -8 |
            |  1 | -5 |  1 |  8 |
            |  7 |  7 | -6 | -7 |
            |  1 | -3 |  7 |  4 |
        And B ← inverse(A)
        Then determinant(A) = 532
        And cofactor(A, 2, 3) = -160
        And B[3,2] = -160/532
        And cofactor(A, 3, 2) = 105
        And B[2,3] = 105/532
        And B is the following 4x4 matrix:
            |  0.21805 |  0.45113 |  0.24060 | -0.04511 |
            | -0.80827 | -1.45677 | -0.44361 |  0.52068 |
            | -0.07895 | -0.22368 | -0.05263 |  0.19737 |
            | -0.52256 | -0.81391 | -0.30075 |  0.30639 |
    */
    #[test]
    fn calculating_inverse_of_a_matrix() {
        let a = Matrix4::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse();
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert!((b[(3, 2)] - (-160.0 / 532.0)).abs() < 1e-5);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert!((b[(2, 3)] - (105.0 / 532.0)).abs() < 1e-5);
        let expected = Matrix4::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        for row in 0..4 {
            for col in 0..4 {
                assert!((b[(row, col)] - expected[(row, col)]).abs() < 1e-5);
            }
        }
    }

    /*
    Scenario: Calculating the inverse of another matrix
        Given the following 4x4 matrix A:
        |  8 | -5 |  9 |  2 |
        |  7 |  5 |  6 |  1 |
        | -6 |  0 |  9 |  6 |
        | -3 |  0 | -9 | -4 |
        Then inverse(A) is the following 4x4 matrix:
        | -0.15385 | -0.15385 | -0.28205 | -0.53846 |
        | -0.07692 |  0.12308 |  0.02564 |  0.03077 |
        |  0.35897 |  0.35897 |  0.43590 |  0.92308 |
        | -0.69231 | -0.69231 | -0.76923 | -1.92308 |
    */
    #[test]
    fn calculating_inverse_of_another_matrix() {
        let a = Matrix4::from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let expected = Matrix4::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        let inv = a.inverse();
        for row in 0..4 {
            for col in 0..4 {
                assert!((inv[(row, col)] - expected[(row, col)]).abs() < 1e-5);
            }
        }
    }

    /*
    Scenario: Calculating the inverse of a third matrix
        Given the following 4x4 matrix A:
        |  9 |  3 |  0 |  9 |
        | -5 | -2 | -6 | -3 |
        | -4 |  9 |  6 |  4 |
        | -7 |  6 |  6 |  2 |
        Then inverse(A) is the following 4x4 matrix:
        | -0.04074 | -0.07778 |  0.14444 | -0.22222 |
        | -0.07778 |  0.03333 |  0.36667 | -0.33333 |
        | -0.02901 | -0.14630 | -0.10926 |  0.12963 |
        |  0.17778 |  0.06667 | -0.26667 |  0.33333 |
    */
    #[test]
    fn calculating_inverse_of_third_matrix() {
        let a = Matrix4::from([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let expected = Matrix4::from([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        let inv = a.inverse();
        for row in 0..4 {
            for col in 0..4 {
                assert!((inv[(row, col)] - expected[(row, col)]).abs() < 1e-5);
            }
        }
    }

    /*
    Scenario: Multiplying a product by its inverse
        Given the following 4x4 matrix A:
            |  3 | -9 |  7 |  3 |
            |  3 | -8 |  2 | -9 |
            | -4 |  4 |  4 |  1 |
            | -6 |  5 | -1 |  1 |
        And the following 4x4 matrix B:
            |  8 |  2 |  2 |  2 |
            |  3 | -1 |  7 |  0 |
            |  7 |  0 |  5 |  4 |
            |  6 | -2 |  0 |  5 |
        And C ← A * B
        Then C * inverse(B) = A
    */
    #[test]
    fn multiplying_product_by_its_inverse() {
        let a = Matrix4::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix4::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a * b;
        let b_inv = b.inverse();
        let result = c * b_inv;
        for row in 0..4 {
            for col in 0..4 {
                assert!((result[(row, col)] - a[(row, col)]).abs() < 1e-5);
            }
        }
    }
}
