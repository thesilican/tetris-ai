use std::convert::TryInto;

use crate::Replay;
use common::model::{Board, GameMove, Piece, BOARD_HEIGHT, BOARD_WIDTH, MOVES_4F_NH};
use rand::{prelude::StdRng, seq::SliceRandom};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(into = "TestCaseSer")]
pub struct TestCase {
    pub board: Board,
    pub pieces: [Piece; 8],
    pub expected: usize,
}
impl TestCase {
    pub fn from_replay(mut rng: &mut StdRng, replay: &Replay) -> Vec<TestCase> {
        println!("Generating test cases from {}...", replay.name);
        replay
            .keyframes()
            .iter()
            .filter_map(|keyframe| {
                let board = keyframe.start.board;
                let mut child_states = keyframe.start.child_states(&MOVES_4F_NH);
                child_states.shuffle(&mut rng);
                let mut pieces = std::iter::once(keyframe.end.current_piece)
                    .chain(
                        child_states
                            .into_iter()
                            .filter_map(|child_state| {
                                let mut game = child_state.game;
                                game.make_move(GameMove::SoftDrop);
                                if game.current_piece == keyframe.end.current_piece {
                                    None
                                } else {
                                    Some(game.current_piece)
                                }
                            })
                            .take(7),
                    )
                    .enumerate()
                    .collect::<Vec<_>>();
                pieces.shuffle(&mut rng);
                let expected = pieces.iter().position(|(i, _)| *i == 0).unwrap();
                Some(TestCase {
                    board,
                    pieces: pieces
                        .into_iter()
                        .map(|(_, x)| x)
                        .collect::<Vec<_>>()
                        .try_into()
                        .ok()?,
                    expected,
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize)]
struct TestCaseSer {
    input: Vec<i8>,
    expected: Vec<i8>,
}
impl From<TestCase> for TestCaseSer {
    fn from(value: TestCase) -> Self {
        let mut input = Vec::<i8>::with_capacity(24 * 10);
        // Board: Bits 0..240
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                if value.board.get(i, j) {
                    input.push(1);
                } else {
                    input.push(0);
                }
            }
        }
        // Pieces: Bits 240..560
        for p in 0..8 {
            let piece = value.pieces[p];
            let shape = piece.get_bit_shape(None, None);
            for row in shape {
                for j in 0..10 {
                    if *row & (1 << j) != 0 {
                        input.push(1);
                    } else {
                        input.push(0);
                    }
                }
            }
        }
        // Expected
        let mut expected = vec![0; 10];
        expected[value.expected] = 1;
        TestCaseSer { input, expected }
    }
}
