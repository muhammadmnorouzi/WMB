pub struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}

impl From<(f32, f32, f32)> for Vector3D {
    fn from(tup: (f32, f32, f32)) -> Self {
        Self {
            x: tup.0,
            y: tup.1,
            z: tup.2,
        }
    }
}

impl From<&[f32; 3]> for Vector3D {
    fn from(arr: &[f32; 3]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

impl From<[f32; 3]> for Vector3D {
    fn from(arr: [f32; 3]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

impl Default for Vector3D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl Vector3D {
    pub fn create(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn create_test() {
        let mut rng = rand::thread_rng();

        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        let z = rng.gen::<f32>();
        let vec = Vector3D::create(x, y, z);

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
        assert_eq!(z, vec.z);
    }

    #[test]
    fn from_tup_test() {
        let mut rng = rand::thread_rng();

        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        let z = rng.gen::<f32>();
        let vec: Vector3D = (x, y, z).into();

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
        assert_eq!(z, vec.z);
    }

    #[test]
    fn from_arr_borrowed_test() {
        let mut rng = rand::thread_rng();

        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        let z = rng.gen::<f32>();
        let vec: Vector3D = (&[x, y, z]).into();

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
        assert_eq!(z, vec.z);
    }

    #[test]
    fn from_arr_owned_test() {
        let mut rng = rand::thread_rng();

        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        let z = rng.gen::<f32>();
        let vec: Vector3D = [x, y, z].into();

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
        assert_eq!(z, vec.z);
    }
}
