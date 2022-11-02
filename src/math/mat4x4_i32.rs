use super::constants::*;
use std::fmt::Debug;

pub type Vec4D = [i32; FOUR];
pub type Vec16D = [i32; SIXTEEN];
pub type Mat4x4 = [Vec4D; FOUR];

#[derive(Debug)]
pub struct Matrix4x4 {
    inner: Mat4x4,
}

impl Matrix4x4 {
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
}
