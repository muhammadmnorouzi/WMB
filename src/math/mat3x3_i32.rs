use std::fmt::Debug;

const ZERO: usize = 0;
const THREE: usize = 3;
const NINE: usize = 9;

pub type Vec3D = [i32; 3];
pub type Vec9D = [i32; 9];
pub type Mat3x3 = [Vec3D; 3];

#[derive(Debug)]
pub struct Matrix3x3 {
    inner: Mat3x3,
}

impl Matrix3x3 {
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
}
