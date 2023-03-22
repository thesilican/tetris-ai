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

// /// Represents a shuffleable 7-bag
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Bag {
//     arr: [PieceType; BAG_LEN],
//     rng: StdRng,
// }
// impl Bag {
//     pub fn new(rng_seed: u64) -> Self {
//         Bag {
//             arr: PieceType::all().collect::<Vec<_>>().try_into().unwrap(),
//             rng: StdRng::seed_from_u64(rng_seed),
//         }
//     }
//     pub fn shuffle(&mut self) {
//         let arr = &mut self.arr;
//         let mut rng = &mut self.rng;
//         for i in (1..arr.len()).rev() {
//             let j = Uniform::new(0, i).sample(&mut rng);
//             arr.swap(i, j);
//         }
//     }
//     pub fn pieces(&self) -> &[PieceType] {
//         &self.arr
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Stream {
//     queue: VecDeque<PieceType>,
// }
// impl Stream {
//     pub fn new() -> Self {
//         Stream {
//             queue: VecDeque::new(),
//         }
//     }
//     pub fn enqueue(&mut self, piece: PieceType) {
//         self.queue.push_back(piece)
//     }
//     pub fn dequeue(&mut self) -> Option<PieceType> {
//         self.queue.pop_front()
//     }
//     pub fn len(&self) -> usize {
//         self.queue.len()
//     }
// }
// impl FromIterator<PieceType> for Stream {
//     fn from_iter<T: IntoIterator<Item = PieceType>>(iter: T) -> Self {
//         Stream {
//             queue: iter.into_iter().collect::<VecDeque<_>>(),
//         }
//     }
// }
