use super::BAG_LEN;
use crate::model::PieceType;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::StdRng,
    SeedableRng,
};
use std::{collections::VecDeque, convert::TryInto, iter::FromIterator};

/// Represents a shuffleable 7-bag
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bag {
    arr: [PieceType; BAG_LEN],
    rng: StdRng,
}
impl Bag {
    pub fn new(rng_seed: u64) -> Self {
        Bag {
            arr: PieceType::all().try_into().unwrap(),
            rng: StdRng::seed_from_u64(rng_seed),
        }
    }
    pub fn shuffle(&mut self) {
        let arr = &mut self.arr;
        let mut rng = &mut self.rng;
        for i in (1..arr.len()).rev() {
            let j = Uniform::new(0, i).sample(&mut rng);
            arr.swap(i, j);
        }
    }
    pub fn pieces(&self) -> &[PieceType] {
        &self.arr
    }
}

#[derive(Debug, Clone)]
pub struct Stream {
    queue: VecDeque<PieceType>,
}
impl Stream {
    pub fn new() -> Self {
        Stream {
            queue: VecDeque::new(),
        }
    }
    pub fn enqueue(&mut self, piece: PieceType) {
        self.queue.push_back(piece)
    }
    pub fn dequeue(&mut self) -> Option<PieceType> {
        self.queue.pop_front()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
impl FromIterator<PieceType> for Stream {
    fn from_iter<T: IntoIterator<Item = PieceType>>(iter: T) -> Self {
        Stream {
            queue: iter.into_iter().collect::<VecDeque<_>>(),
        }
    }
}
