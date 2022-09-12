use rand::Rng;

pub fn random_f32() -> f32 {
    rand::thread_rng().gen::<f32>()
}
