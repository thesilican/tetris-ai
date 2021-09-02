// use crate::{Replay, TransitionChain};
// use common::model::{Board, Piece, BOARD_HEIGHT, BOARD_WIDTH};
// use rand::{rngs::StdRng, SeedableRng};
// use serde::Serialize;

// #[derive(Debug, Clone, Copy, Serialize)]
// #[serde(into = "TestCaseSerialized")]
// struct TestCase {
//     pub board: Board,
//     pub pieces: [Piece; 8],
//     pub expected: usize,
// }

// #[derive(Debug, Clone, Serialize)]
// struct TestCaseSerialized {
//     input: Vec<i8>,
//     expected: Vec<i8>,
// }
// impl From<TestCase> for TestCaseSerialized {
//     fn from(value: TestCase) -> Self {
//         let mut input = Vec::<i8>::with_capacity(24 * 10);
//         // Board: Bits 0..240
//         for i in 0..BOARD_WIDTH {
//             for j in 0..BOARD_HEIGHT {
//                 if value.board.get(i, j) {
//                     input.push(1);
//                 } else {
//                     input.push(0);
//                 }
//             }
//         }
//         // Pieces: Bits 240..560
//         for p in 0..8 {
//             let piece = value.pieces[p];
//             let shape = piece.get_bit_shape(None, None);
//             for row in shape {
//                 for j in 0..10 {
//                     if *row & (1 << j) != 0 {
//                         input.push(1);
//                     } else {
//                         input.push(0);
//                     }
//                 }
//             }
//         }
//         // Expected
//         let mut expected = vec![0; 10];
//         expected[value.expected] = 1;
//         TestCaseSerialized { input, expected }
//     }
// }

// fn transition_chain_to_test_cases(transitions: TransitionChain, seed: u64) -> Vec<TestCase> {
//     let mut test_cases = Vec::new();
//     let mut rand = StdRng::seed_from_u64(seed);
//     for transition in transitions.transitions {
//         let start = transition.start;
//         let mut children = vec![transition.end];
//         // Child state
//         let child_states = start.child_states(moves_list);
//         for i in 0..7 {}
//     }
//     test_cases
// }
