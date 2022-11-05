use super::constants::*;
use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

pub type Vec3D = [i32; THREE];
pub type Vec9D = [i32; NINE];
pub type Mat3x3 = [Vec3D; THREE];

#[derive(Clone, Copy)]
pub struct Matrix3x3 {
    inner: Mat3x3,
}

impl Debug for Matrix3x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n----------------------------");
        writeln!(f, "{:?}", self.inner[0]);
        writeln!(f, "{:?}", self.inner[1]);
        writeln!(f, "{:?}", self.inner[2]);
        writeln!(f, "----------------------------");

        Ok(())
    }
}

impl Matrix3x3 {
    pub fn new_row_major() -> Self {
        let mut result = Self::default();

        for i in 0..THREE {
            for j in 0..THREE {
                result.inner[i][j] = (i * THREE + j) as i32;
            }
        }

        return result;
    }

    pub fn new_col_major() -> Self {
        let mut result = Self::default();

        for i in 0..THREE {
            for j in 0..THREE {
                result.inner[i][j] = (j * THREE + i) as i32;
            }
        }

        return result;
    }

    fn new(
        n00: i32,
        n01: i32,
        n02: i32,
        n10: i32,
        n11: i32,
        n12: i32,
        n20: i32,
        n21: i32,
        n22: i32,
    ) -> Self {
        Matrix3x3 {
            inner: [[n00, n10, n20], [n01, n11, n21], [n02, n12, n22]],
        }
    }

    /// column major matrix
    fn at(&self, i: usize, j: usize) -> i32 {
        assert!(i >= ZERO && i <= THREE);
        assert!(j >= ZERO && j <= THREE);

        return self.inner[j][i];
    }

    fn col_at(&self, i: usize) -> &Vec3D {
        assert!(i >= ZERO && i <= THREE);

        return &self.inner[i];
    }
}

impl Default for Matrix3x3 {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl From<Vec9D> for Matrix3x3 {
    fn from(v: Vec9D) -> Self {
        let mut mat = Matrix3x3::default();

        for i in 0..THREE {
            for j in 0..THREE {
                mat.inner[j][i] = v[i * THREE + j];
            }
        }

        return mat;
    }
}

impl From<i32> for Matrix3x3 {
    fn from(v: i32) -> Self {
        let mut mat = Matrix3x3::default();

        for i in 0..THREE {
            for j in 0..THREE {
                mat.inner[i][j] = v;
            }
        }

        return mat;
    }
}

impl Add for Matrix3x3 {
    type Output = Matrix3x3;

    fn add(self, other: Self) -> Self::Output {
        let mut result: Matrix3x3 = Default::default();

        for i in 0..THREE {
            for j in 0..THREE {
                result.inner[i][j] = self.inner[i][j] + other.inner[i][j];
            }
        }

        return result;
    }
}

impl Sub for Matrix3x3 {
    type Output = Matrix3x3;

    fn sub(self, other: Self) -> Self::Output {
        let mut result: Matrix3x3 = Default::default();

        for i in 0..THREE {
            for j in 0..THREE {
                result.inner[i][j] = self.inner[i][j] - other.inner[i][j];
            }
        }

        return result;
    }
}

impl Mul<i32> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, scalar: i32) -> Self::Output {
        let mut result: Matrix3x3 = Default::default();

        for i in 0..THREE {
            for j in 0..THREE {
                result.inner[i][j] = self.inner[i][j] * scalar;
            }
        }

        return result;
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Mul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, other: Matrix3x3) -> Self::Output {
        let n00: i32 = self.inner[0][0] * other.inner[0][0]
            + self.inner[1][0] * other.inner[0][1]
            + self.inner[2][0] * other.inner[0][2];

        let n01: i32 = self.inner[0][0] * other.inner[1][0]
            + self.inner[1][0] * other.inner[1][1]
            + self.inner[2][0] * other.inner[1][2];

        let n02: i32 = self.inner[0][0] * other.inner[2][0]
            + self.inner[1][0] * other.inner[2][1]
            + self.inner[2][0] * other.inner[2][2];

        let n10: i32 = self.inner[0][1] * other.inner[0][0]
            + self.inner[1][1] * other.inner[0][1]
            + self.inner[2][1] * other.inner[0][2];

        let n11: i32 = self.inner[0][1] * other.inner[1][0]
            + self.inner[1][1] * other.inner[1][1]
            + self.inner[2][1] * other.inner[1][2];

        let n12: i32 = self.inner[0][1] * other.inner[2][0]
            + self.inner[1][1] * other.inner[2][1]
            + self.inner[2][1] * other.inner[2][2];

        let n20: i32 = self.inner[0][2] * other.inner[0][0]
            + self.inner[1][2] * other.inner[0][1]
            + self.inner[2][2] * other.inner[0][2];

        let n21: i32 = self.inner[0][2] * other.inner[1][0]
            + self.inner[1][2] * other.inner[1][1]
            + self.inner[2][2] * other.inner[1][2];

        let n22: i32 = self.inner[0][2] * other.inner[2][0]
            + self.inner[1][2] * other.inner[2][1]
            + self.inner[2][2] * other.inner[2][2];

        Self::new(n00, n01, n02, n10, n11, n12, n20, n21, n22)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::random::random_i32;

    use super::*;

    fn random_vec9d() -> Vec9D {
        let mut v: Vec9D = Default::default();

        for i in 0..NINE {
            v[i] = random_i32();
        }

        return v;
    }

    fn random_tuple_9d_i32() -> (i32, i32, i32, i32, i32, i32, i32, i32, i32) {
        return (
            random_i32(),
            random_i32(),
            random_i32(),
            random_i32(),
            random_i32(),
            random_i32(),
            random_i32(),
            random_i32(),
            random_i32(),
        );
    }

    fn random_mat3x3() -> Matrix3x3 {
        let mut mat = Matrix3x3::default();

        for i in 0..THREE {
            for j in 0..THREE {
                mat.inner[i][j] = random_i32() % 1_000_000;
            }
        }

        return mat;
    }

    #[test]
    fn new_test() {
        let v = random_tuple_9d_i32();
        let (n00, n01, n02, n10, n11, n12, n20, n21, n22) = v;
        let mat = Matrix3x3::new(n00, n01, n02, n10, n11, n12, n20, n21, n22);

        assert_eq!(mat.at(0, 0), v.0);
        assert_eq!(mat.at(0, 1), v.1);
        assert_eq!(mat.at(0, 2), v.2);
        assert_eq!(mat.at(1, 0), v.3);
        assert_eq!(mat.at(1, 1), v.4);
        assert_eq!(mat.at(1, 2), v.5);
        assert_eq!(mat.at(2, 0), v.6);
        assert_eq!(mat.at(2, 1), v.7);
        assert_eq!(mat.at(2, 2), v.8);
    }

    #[test]
    fn from_i32_test() {
        let n = random_i32();
        let mat = Matrix3x3::from(n);

        assert!(mat.inner.iter().all(|r| r.iter().all(|&e| e == n)))
    }

    #[test]
    fn from_vec9d_test() {
        let v = random_vec9d();
        let mat = Matrix3x3::from(v);

        for i in 0..THREE {
            for j in 0..THREE {
                assert_eq!(mat.at(i, j), v[i * THREE + j]);
            }
        }
    }

    #[test]
    fn col_at_test() {
        let v = random_vec9d();
        let c1 = [v[ZERO], v[THREE], v[THREE * 2]];
        let c2 = [v[ZERO + 1], v[THREE + 1], v[THREE * 2 + 1]];
        let c3 = [v[ZERO + 2], v[THREE + 2], v[THREE * 2 + 2]];

        let mat = Matrix3x3::from(v);

        assert_eq!(&c1, mat.col_at(0));
        assert_eq!(&c2, mat.col_at(1));
        assert_eq!(&c3, mat.col_at(2));
    }

    #[test]
    fn add_test() {
        let mat1 = random_mat3x3();
        let mat2 = random_mat3x3();
        let result = mat1 + mat2;

        let mut expected = Matrix3x3::default();

        for i in 0..THREE {
            for j in 0..THREE {
                expected.inner[i][j] = mat1.inner[i][j] + mat2.inner[i][j];
            }
            println!();
        }

        assert_eq!(result, expected);
    }

    #[test]
    fn sub_test() {
        let mat1 = random_mat3x3();
        let mat2 = random_mat3x3();
        let result = mat1 - mat2;

        let mut expected = Matrix3x3::default();

        for i in 0..THREE {
            for j in 0..THREE {
                expected.inner[i][j] = mat1.inner[i][j] - mat2.inner[i][j];
            }
            println!();
        }

        assert_eq!(result, expected);
    }

    #[test]
    fn scalar_mul_test() {
        let mat1 = random_mat3x3();
        let scalar = random_i32() % 100 + 1;
        let result = mat1 * scalar;

        let mut expected = Matrix3x3::default();

        for i in 0..THREE {
            for j in 0..THREE {
                expected.inner[i][j] = mat1.inner[i][j] * scalar;
            }
            println!();
        }

        assert_eq!(result, expected);
    }
}
