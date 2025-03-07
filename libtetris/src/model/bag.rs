use crate::model::PieceType;
use rand::seq::SliceRandom;
use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::collections::VecDeque;

/// A bag of Tetris pieces that can be pulled from
#[derive(Debug, Clone)]
pub enum Bag {
    Fixed {
        sequence: Vec<PieceType>,
        queue: VecDeque<PieceType>,
    },
    Rng7 {
        rng: XorShiftRng,
        queue: VecDeque<PieceType>,
    },
}

impl Bag {
    /// Generate a new fixed sequence bag
    pub fn new_fixed(sequence: &[PieceType]) -> Self {
        Bag::Fixed {
            sequence: sequence.to_vec(),
            queue: VecDeque::new(),
        }
    }

    /// Generate a new 7-bag
    pub fn new_rng7(seed: u64) -> Self {
        Bag::Rng7 {
            rng: XorShiftRng::seed_from_u64(seed),
            queue: VecDeque::new(),
        }
    }

    /// Dequeue the next piece from the bag
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
                    arr.shuffle(rng);
                    queue.extend(arr);
                }
                queue.pop_front().unwrap()
            }
        }
    }
}
