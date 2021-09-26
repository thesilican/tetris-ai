use crate::Replay;
use common::model::{Board, GameAction, BOARD_HEIGHT, BOARD_WIDTH, MOVES_4F_NH};
use rand::{prelude::StdRng, seq::SliceRandom};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(into = "TestCaseSer")]
pub struct TestCase {
    pub board: Board,
    pub label: bool,
}
impl TestCase {
    pub fn from_replay(mut rng: &mut StdRng, replay: &Replay) -> Vec<TestCase> {
        println!("Generating test cases from {}...", replay.name);
        // Turn keyframes into test cases
        let mut test_cases = Vec::new();
        for keyframe in replay.keyframes() {
            // Generate "good" case from keyframe end
            let good_case = {
                let mut game = keyframe.end;
                game.apply_action(GameAction::Lock);
                TestCase {
                    board: game.board,
                    label: true,
                }
            };
            test_cases.push(good_case);
            // Generate "bad" test cases from child_states
            let mut child_states = keyframe.start.child_states(&MOVES_4F_NH);
            child_states.shuffle(&mut rng);
            let bad_cases = child_states.iter().take(1).map(|child_state| {
                let mut game = child_state.game;
                game.apply_action(GameAction::Lock);
                TestCase {
                    board: game.board,
                    label: false,
                }
            });
            test_cases.extend(bad_cases);
        }
        test_cases
    }
}

#[derive(Debug, Clone, Serialize)]
struct TestCaseSer {
    input: Vec<i8>,
    label: i8,
}
impl From<TestCase> for TestCaseSer {
    fn from(value: TestCase) -> Self {
        let mut input = Vec::with_capacity(BOARD_WIDTH * BOARD_HEIGHT);
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
        // Expected
        let label = if value.label { 1 } else { 0 };

        TestCaseSer { input, label }
    }
}
