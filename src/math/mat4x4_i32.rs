use super::constants::*;

use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

pub type Vec4D = [i32; FOUR];
pub type Vec16D = [i32; SIXTEEN];
pub type Mat4x4 = [Vec4D; FOUR];

#[derive(Copy, Clone)]
pub struct Matrix4x4 {
    inner: Mat4x4,
}

impl Debug for Matrix4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n----------------------------");
        writeln!(f, "{:?}", self.inner[0]);
        writeln!(f, "{:?}", self.inner[1]);
        writeln!(f, "{:?}", self.inner[2]);
        writeln!(f, "{:?}", self.inner[3]);
        writeln!(f, "----------------------------");

        Ok(())
    }
}

impl Matrix4x4 {
    pub fn new_row_major() -> Self {
        let mut result = Self::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                result.inner[i][j] = (i * FOUR + j) as i32;
            }
        }

        return result;
    }

    pub fn new_col_major() -> Self {
        let mut result = Self::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                result.inner[i][j] = (j * FOUR + i) as i32;
            }
        }

        return result;
    }

    fn new(
        n00: i32,
        n01: i32,
        n02: i32,
        n03: i32,
        n10: i32,
        n11: i32,
        n12: i32,
        n13: i32,
        n20: i32,
        n21: i32,
        n22: i32,
        n23: i32,
        n30: i32,
        n31: i32,
        n32: i32,
        n33: i32,
    ) -> Self {
        Self {
            inner: [
                [n00, n10, n20, n30],
                [n01, n11, n21, n31],
                [n02, n12, n22, n32],
                [n03, n13, n23, n33],
            ],
        }
    }

    /// column major matrix
    fn at(&self, i: usize, j: usize) -> i32 {
        assert!(i >= ZERO && i <= FOUR);
        assert!(j >= ZERO && j <= FOUR);

        return self.inner[j][i];
    }

    fn col_at(&self, i: usize) -> &Vec4D {
        assert!(i >= ZERO && i <= FOUR);

        return &self.inner[i];
    }
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl From<Vec16D> for Matrix4x4 {
    fn from(v: Vec16D) -> Self {
        let mut mat = Matrix4x4::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                mat.inner[j][i] = v[i * FOUR + j];
            }
        }

        return mat;
    }
}

impl From<i32> for Matrix4x4 {
    fn from(v: i32) -> Self {
        let mut mat = Matrix4x4::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                mat.inner[i][j] = v;
            }
        }

        return mat;
    }
}

impl Add for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, other: Self) -> Self::Output {
        let mut result: Matrix4x4 = Default::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                result.inner[i][j] = self.inner[i][j] + other.inner[i][j];
            }
        }

        return result;
    }
}

impl Sub for Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, other: Self) -> Self::Output {
        let mut result: Matrix4x4 = Default::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                result.inner[i][j] = self.inner[i][j] - other.inner[i][j];
            }
        }

        return result;
    }
}

impl Mul<i32> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, scalar: i32) -> Self::Output {
        let mut result: Matrix4x4 = Default::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                result.inner[i][j] = self.inner[i][j] * scalar;
            }
        }

        return result;
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: Matrix4x4) -> Self::Output {
        let n00: i32 = self.inner[0][0] * other.inner[0][0]
            + self.inner[1][0] * other.inner[0][1]
            + self.inner[2][0] * other.inner[0][2]
            + self.inner[3][0] * other.inner[0][3];

        let n01: i32 = self.inner[0][0] * other.inner[1][0]
            + self.inner[1][0] * other.inner[1][1]
            + self.inner[2][0] * other.inner[1][2]
            + self.inner[3][0] * other.inner[1][3];

        let n02: i32 = self.inner[0][0] * other.inner[2][0]
            + self.inner[1][0] * other.inner[2][1]
            + self.inner[2][0] * other.inner[2][2]
            + self.inner[3][0] * other.inner[2][3];

        let n03: i32 = self.inner[0][0] * other.inner[3][0]
            + self.inner[1][0] * other.inner[3][1]
            + self.inner[2][0] * other.inner[3][2]
            + self.inner[3][0] * other.inner[3][3];

        let n10: i32 = self.inner[0][1] * other.inner[0][0]
            + self.inner[1][1] * other.inner[0][1]
            + self.inner[2][1] * other.inner[0][2]
            + self.inner[3][1] * other.inner[0][3];

        let n11: i32 = self.inner[0][1] * other.inner[1][0]
            + self.inner[1][1] * other.inner[1][1]
            + self.inner[2][1] * other.inner[1][2]
            + self.inner[3][1] * other.inner[1][3];

        let n12: i32 = self.inner[0][1] * other.inner[2][0]
            + self.inner[1][1] * other.inner[2][1]
            + self.inner[2][1] * other.inner[2][2]
            + self.inner[3][1] * other.inner[2][3];

        let n13: i32 = self.inner[0][1] * other.inner[3][0]
            + self.inner[1][1] * other.inner[3][1]
            + self.inner[2][1] * other.inner[3][2]
            + self.inner[3][1] * other.inner[3][3];

        let n20: i32 = self.inner[0][2] * other.inner[0][0]
            + self.inner[1][2] * other.inner[0][1]
            + self.inner[2][2] * other.inner[0][2]
            + self.inner[3][2] * other.inner[0][3];

        let n21: i32 = self.inner[0][2] * other.inner[1][0]
            + self.inner[1][2] * other.inner[1][1]
            + self.inner[2][2] * other.inner[1][2]
            + self.inner[3][2] * other.inner[1][3];

        let n22: i32 = self.inner[0][2] * other.inner[2][0]
            + self.inner[1][2] * other.inner[2][1]
            + self.inner[2][2] * other.inner[2][2]
            + self.inner[3][2] * other.inner[2][3];

        let n23: i32 = self.inner[0][2] * other.inner[3][0]
            + self.inner[1][2] * other.inner[3][1]
            + self.inner[2][2] * other.inner[3][2]
            + self.inner[3][2] * other.inner[3][3];

        let n30: i32 = self.inner[0][3] * other.inner[0][0]
            + self.inner[1][3] * other.inner[0][1]
            + self.inner[2][3] * other.inner[0][2]
            + self.inner[3][3] * other.inner[0][3];

        let n31: i32 = self.inner[0][3] * other.inner[1][0]
            + self.inner[1][3] * other.inner[1][1]
            + self.inner[2][3] * other.inner[1][2]
            + self.inner[3][3] * other.inner[1][3];

        let n32: i32 = self.inner[0][3] * other.inner[2][0]
            + self.inner[1][3] * other.inner[2][1]
            + self.inner[2][3] * other.inner[2][2]
            + self.inner[3][3] * other.inner[2][3];

        let n33: i32 = self.inner[0][3] * other.inner[3][0]
            + self.inner[1][3] * other.inner[3][1]
            + self.inner[2][3] * other.inner[3][2]
            + self.inner[3][3] * other.inner[3][3];

        Self::new(
            n00, n01, n02, n03, n10, n11, n12, n13, n20, n21, n22, n23, n30, n31, n32, n33,
        )
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

#[cfg(test)]
mod tests {
    use crate::math::random::random_i32;

    use super::*;

    fn random_vec16d() -> Vec16D {
        let mut v: Vec16D = Default::default();

        for i in 0..SIXTEEN {
            v[i] = random_i32();
        }

        return v;
    }

    fn random_tuple_16d_i32() -> (
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
    ) {
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
            random_i32(),
            random_i32(),
            random_i32(),
            random_i32(),
            random_i32(),
            random_i32(),
            random_i32(),
        );
    }

    fn random_mat4x4() -> Matrix4x4 {
        let mut mat = Matrix4x4::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                mat.inner[i][j] = random_i32() % 1_000_000;
            }
        }

        return mat;
    }

    #[test]
    fn new_test() {
        let v = random_tuple_16d_i32();

        let (n00, n01, n02, n03, n10, n11, n12, n13, n20, n21, n22, n23, n30, n31, n32, n33) = v;

        let mat = Matrix4x4::new(
            n00, n01, n02, n03, n10, n11, n12, n13, n20, n21, n22, n23, n30, n31, n32, n33,
        );

        assert_eq!(mat.at(0, 0), v.0);
        assert_eq!(mat.at(0, 1), v.1);
        assert_eq!(mat.at(0, 2), v.2);
        assert_eq!(mat.at(0, 3), v.3);
        assert_eq!(mat.at(1, 0), v.4);
        assert_eq!(mat.at(1, 1), v.5);
        assert_eq!(mat.at(1, 2), v.6);
        assert_eq!(mat.at(1, 3), v.7);
        assert_eq!(mat.at(2, 0), v.8);
        assert_eq!(mat.at(2, 1), v.9);
        assert_eq!(mat.at(2, 2), v.10);
        assert_eq!(mat.at(2, 3), v.11);
        assert_eq!(mat.at(3, 0), v.12);
        assert_eq!(mat.at(3, 1), v.13);
        assert_eq!(mat.at(3, 2), v.14);
        assert_eq!(mat.at(3, 3), v.15);
    }

    #[test]
    fn from_i32_test() {
        let n = random_i32();
        let mat = Matrix4x4::from(n);

        assert!(mat.inner.iter().all(|r| r.iter().all(|&e| e == n)))
    }

    #[test]
    fn from_vec16d_test() {
        let v = random_vec16d();
        let mat = Matrix4x4::from(v);

        for i in 0..FOUR {
            for j in 0..FOUR {
                assert_eq!(mat.at(i, j), v[i * FOUR + j]);
            }
        }
    }

    #[test]
    fn col_at_test() {
        let v = random_vec16d();
        let c1 = [v[ZERO], v[FOUR], v[FOUR * 2], v[FOUR * 3]];
        let c2 = [v[ZERO + 1], v[FOUR + 1], v[FOUR * 2 + 1], v[FOUR * 3 + 1]];
        let c3 = [v[ZERO + 2], v[FOUR + 2], v[FOUR * 2 + 2], v[FOUR * 3 + 2]];
        let c4 = [v[ZERO + 3], v[FOUR + 3], v[FOUR * 2 + 3], v[FOUR * 3 + 3]];

        let mat = Matrix4x4::from(v);

        assert_eq!(&c1, mat.col_at(0));
        assert_eq!(&c2, mat.col_at(1));
        assert_eq!(&c3, mat.col_at(2));
        assert_eq!(&c4, mat.col_at(3));
    }

    #[test]
    fn add_test() {
        let mat1 = random_mat4x4();
        let mat2 = random_mat4x4();
        let result = mat1 + mat2;

        let mut expected = Matrix4x4::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                expected.inner[i][j] = mat1.inner[i][j] + mat2.inner[i][j];
            }
            println!();
        }

        assert_eq!(result, expected);
    }

    #[test]
    fn sub_test() {
        let mat1 = random_mat4x4();
        let mat2 = random_mat4x4();
        let result = mat1 - mat2;

        let mut expected = Matrix4x4::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                expected.inner[i][j] = mat1.inner[i][j] - mat2.inner[i][j];
            }
            println!();
        }

        assert_eq!(result, expected);
    }

    #[test]
    fn scalar_mul_test() {
        let mat1 = random_mat4x4();
        let scalar = random_i32() % 100 + 1;
        let result = mat1 * scalar;

        let mut expected = Matrix4x4::default();

        for i in 0..FOUR {
            for j in 0..FOUR {
                expected.inner[i][j] = mat1.inner[i][j] * scalar;
            }
            println!();
        }

        assert_eq!(result, expected);
    }
}
