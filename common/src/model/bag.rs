use crate::model::PieceType;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::StdRng,
    SeedableRng,
};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum Bag {
    Fixed {
        sequence: Vec<PieceType>,
        queue: VecDeque<PieceType>,
    },
    Rng7 {
        rng: StdRng,
        queue: VecDeque<PieceType>,
    },
}

impl Bag {
    pub fn new_fixed(sequence: &[PieceType]) -> Self {
        Bag::Fixed {
            sequence: sequence.to_vec(),
            queue: VecDeque::new(),
        }
    }

    pub fn new_rng7(seed: u64) -> Self {
        Bag::Rng7 {
            rng: StdRng::seed_from_u64(seed),
            queue: VecDeque::new(),
        }
    }

    pub fn next(&mut self) -> PieceType {
        match self {
            Bag::Fixed { sequence, queue } => {
                if queue.is_empty() {
                    queue.extend(sequence.iter().copied());
                }
                queue.pop_front().unwrap()
            }
            Bag::Rng7 { rng, queue } => {
                if queue.is_empty() {
                    let mut arr = PieceType::ALL;
                    for i in (1..arr.len()).rev() {
                        let j = Uniform::new(0, i).sample(rng);
                        arr.swap(i, j);
                    }
                    queue.extend(arr);
                }
                queue.pop_front().unwrap()
            }
        }
    }
}
