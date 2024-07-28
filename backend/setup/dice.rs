use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Dice {
    n_dice: u32,
    n_faces: u32,
    
    seed: u64,

    n_throws: u32,

    pub draw: Option<u32>,
}


impl Dice {
    pub fn new(n_dice: u32, n_faces: u32, seed: u64) -> Self {
        Dice {n_dice, n_faces, seed, n_throws: 0, draw: None}
    }

    pub fn throw(mut self) -> Self {

        let n_dice = self.n_dice;
        let n_faces = self.n_faces;

        let seed = self.seed;

        let mut rng = StdRng::seed_from_u64(seed);

        let draw: u32 = (0..n_dice).map(|_| rng.gen_range(1..=n_faces)).sum();

        Dice {
            n_dice,
            n_faces,
            seed: seed + 1,
            n_throws: self.n_throws,
            draw: Some(draw),
        }
    }
}