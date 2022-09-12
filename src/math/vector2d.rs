use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

#[derive(Debug)]
pub struct Vector2D {
    x: f32,
    y: f32,
}

impl PartialEq for Vector2D {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x <= f32::EPSILON) && (self.y - other.y <= f32::EPSILON)
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, right: Self) -> Self::Output {
        Vector2D::create(self.x + right.x, self.y + right.y)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, right: Self) {
        self.x += right.x;
        self.y += right.y;
    }
}

impl Add<f32> for Vector2D {
    type Output = Vector2D;

    fn add(self, scalar: f32) -> Self::Output {
        Vector2D::create(self.x + scalar, self.y + scalar)
    }
}

impl AddAssign<f32> for Vector2D {
    fn add_assign(&mut self, scalar: f32) {
        self.x += scalar;
        self.y += scalar;
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector2D::create(self.x * scalar, self.y * scalar)
    }
}

impl Mul<f32> for &Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector2D::create(self.x * scalar, self.y * scalar)
    }
}

impl MulAssign<f32> for Vector2D {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f32> for Vector2D {
    type Output = Vector2D;

    fn div(self, right: f32) -> Self::Output {
        let right = 1.0f32 / right;
        self * right
    }
}

impl Div<f32> for &Vector2D {
    type Output = Vector2D;

    fn div(self, right: f32) -> Self::Output {
        let right = 1.0f32 / right;
        self * right
    }
}

impl DivAssign<f32> for Vector2D {
    fn div_assign(&mut self, scalar: f32) {
        let scalar = 1.0f32 / scalar;
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl From<(f32, f32)> for Vector2D {
    fn from(tup: (f32, f32)) -> Self {
        Self { x: tup.0, y: tup.1 }
    }
}

impl From<&[f32; 2]> for Vector2D {
    fn from(arr: &[f32; 2]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
        }
    }
}

impl From<[f32; 2]> for Vector2D {
    fn from(arr: [f32; 2]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
        }
    }
}

impl Default for Vector2D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl Vector2D {
    pub fn create(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: &Vector2D) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn magnitude(&self) -> f32 {
        return self.dot(self).sqrt();
    }

    pub fn normalize(&self) -> Vector2D {
        self / self.magnitude()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::random::random_f32;

    #[test]
    fn create_test() {
        let x = random_f32();
        let y = random_f32();
        let vec = Vector2D::create(x, y);

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
    }

    #[test]
    fn from_tup_test() {
        let x = random_f32();
        let y = random_f32();
        let vec: Vector2D = (x, y).into();

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
    }

    #[test]
    fn from_arr_borrowed_test() {
        let x = random_f32();
        let y = random_f32();
        let vec: Vector2D = (&[x, y]).into();

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
    }

    #[test]
    fn from_arr_owned_test() {
        let x = random_f32();
        let y = random_f32();
        let vec: Vector2D = [x, y].into();

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
    }

    #[test]
    fn add_test() {
        let v1 = Vector2D::create(random_f32(), random_f32());
        let v2 = Vector2D::create(random_f32(), random_f32());

        assert_eq!(Vector2D::create(v1.x + v2.x, v1.y + v2.y), v1 + v2);
    }

    #[test]
    fn add_assign_test() {
        let v1 = Vector2D::create(random_f32(), random_f32());
        let v2 = Vector2D::create(random_f32(), random_f32());
        let expected = Vector2D::create(v1.x + v2.x, v1.y + v2.y);

        let mut v1_mut = v1;
        v1_mut += v2;

        assert_eq!(expected, v1_mut);
    }

    #[test]
    fn add_scalar_test() {
        let v = Vector2D::create(random_f32(), random_f32());
        let scalar = random_f32();
        let expected = Vector2D::create(v.x + scalar, v.y + scalar);

        assert_eq!(expected, v + scalar);
    }

    #[test]
    fn add_assign_scalar_test() {
        let x = random_f32();
        let y = random_f32();
        let mut v = Vector2D::create(x, y);
        let scalar = random_f32();
        let expected = Vector2D::create(x + scalar, v.y + scalar);

        v += scalar;

        assert_eq!(expected, v);
    }

    #[test]
    fn mul_scalar_test() {
        let v = Vector2D::create(random_f32(), random_f32());
        let scalar = random_f32();
        let expected = Vector2D::create(v.x * scalar, v.y * scalar);

        assert_eq!(expected, v * scalar);
    }

    #[test]
    fn mul_scalar_borrowed_test() {
        let v = &Vector2D::create(random_f32(), random_f32());
        let scalar = random_f32();
        let expected = Vector2D::create(v.x * scalar, v.y * scalar);

        assert_eq!(expected, v * scalar);
    }

    #[test]
    fn mul_assign_scalar_test() {
        let x = random_f32();
        let y = random_f32();
        let mut v = Vector2D::create(x, y);
        let scalar = random_f32();
        let expected = Vector2D::create(x * scalar, v.y * scalar);

        v *= scalar;

        assert_eq!(expected, v);
    }

    #[test]
    fn div_scalar_test() {
        let v = Vector2D::create(random_f32(), random_f32());
        let scalar = random_f32();
        let scalar = 1.0f32 / scalar;
        let expected = Vector2D::create(v.x / scalar, v.y / scalar);

        assert_eq!(expected, v / scalar);
    }

    #[test]
    fn div_scalar_borrowed_test() {
        let v = &Vector2D::create(random_f32(), random_f32());
        let scalar = random_f32();
        let scalar = 1.0f32 / scalar; //this is how we operate
        let expected = Vector2D::create(v.x / scalar, v.y / scalar);

        assert_eq!(expected, v / scalar);
    }

    #[test]
    fn div_assign_scalar_test() {
        let x = random_f32();
        let y = random_f32();
        let mut v = Vector2D::create(x, y);
        let scalar = random_f32();
        let expected = Vector2D::create(x / scalar, v.y / scalar);

        v /= scalar;

        assert_eq!(expected, v);
    }

    #[test]
    fn dot_test() {
        let v1 = Vector2D::create(random_f32(), random_f32());
        let v2 = Vector2D::create(random_f32(), random_f32());
        let expected = (v1.x * v2.x) + (v1.y * v2.y);

        assert_eq!(v1.dot(&v2), expected);
    }

    #[test]
    fn magnitude_test() {
        let v1 = &Vector2D::create(random_f32(), random_f32());
        let expected = v1.dot(v1).sqrt();

        assert_eq!(v1.magnitude(), expected);
    }

    #[test]
    fn normalize_test() {
        let v1 = &Vector2D::create(random_f32(), random_f32());
        let expected = v1 / v1.magnitude();

        assert_eq!(v1.normalize(), expected);
    }
}
