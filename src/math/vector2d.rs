pub struct Vector2D {
    x: f32,
    y: f32,
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
        let vec = Vector2D::create(x, y);

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
    }

    #[test]
    fn from_tup_test() {
        let mut rng = rand::thread_rng();

        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        let vec: Vector2D = (x, y).into();

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
    }

    #[test]
    fn from_arr_borrowed_test() {
        let mut rng = rand::thread_rng();

        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        let vec: Vector2D = (&[x, y]).into();

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
    }

    #[test]
    fn from_arr_owned_test() {
        let mut rng = rand::thread_rng();

        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        let vec: Vector2D = [x, y].into();

        assert_eq!(x, vec.x);
        assert_eq!(y, vec.y);
    }
}
