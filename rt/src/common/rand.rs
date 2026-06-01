use std::sync::{LazyLock, RwLock};
use rand::rngs::{StdRng};
use rand::{Rng, SeedableRng};


struct  Rand {
    r_gen: StdRng,
}

impl Rand {
    pub fn new() -> Self {
        Self {
            r_gen: StdRng::from_entropy(),
        }
    }

    pub fn generate(&mut self) -> f64 {
        self.r_gen.r#gen::<f64>()
    }
}

