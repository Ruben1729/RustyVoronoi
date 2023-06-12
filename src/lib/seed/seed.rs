use rand::Rng;

#[derive(Clone)]
pub struct Seed {
    pub x: f64,
    pub y: f64
}

impl Seed {
    pub fn new(width: f64, height: f64) -> Self{
        let mut rng = rand::thread_rng();

        Seed {
            x: rng.gen_range(0.0..width),
            y: rng.gen_range(0.0..height)
        }
    }
}
