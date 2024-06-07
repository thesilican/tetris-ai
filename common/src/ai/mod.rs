use crate::{
    model::{Bag, Game},
    ChildError, GameAction, LockInfo,
};
use serde::{Deserialize, Serialize};
use std::{
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "crate::serde::SerializedEvaluation")]
#[serde(into = "crate::serde::SerializedEvaluation")]
pub enum Evaluation {
    Success {
        actions: Vec<GameAction>,
        score: Option<f64>,
    },
    Fail {
        message: String,
    },
}

pub trait Ai {
    fn evaluate(&mut self, game: &Game) -> Evaluation;

    // Prints out demo for the Ai
    fn demo(&mut self) {
        let mut bag = Bag::new_rng7(0);
        let mut game = Game::from_bag(&mut bag);
        println!("{game}");
        'l: loop {
            let start = Instant::now();
            let res = self.evaluate(&game);
            let elapsed = start.elapsed();

            match res {
                Evaluation::Success { actions, .. } => {
                    for action in actions {
                        if let GameAction::HardDrop = action {
                            let info = game.hard_drop();
                            if let Some(LockInfo { top_out: true, .. }) = info {
                                println!("TOP OUT");
                                break 'l;
                            }
                        } else {
                            game.apply_action(action);
                        }
                        println!("{game}\nEvaluated in {elapsed:?}");
                    }
                }
                Evaluation::Fail { message } => {
                    println!("Evaluation failed: {message}");
                    break;
                }
            }
            thread::sleep(Duration::from_millis(50));
            game.refill_queue(&mut bag);
        }
    }
}

/// A very simple ai, useful for testing.
/// Should never top out...
#[derive(Debug)]
pub struct SimpleAi;

impl SimpleAi {
    pub fn new() -> Self {
        SimpleAi
    }
}

impl Ai for SimpleAi {
    fn evaluate(&mut self, game: &Game) -> Evaluation {
        let children = match game.children() {
            Ok(children) => children,
            Err(ChildError::PieceTooLow | ChildError::AgainstWall) => {
                return Evaluation::Success {
                    actions: vec![GameAction::HardDrop],
                    score: None,
                }
            }
        };
        let mut best_child = None;
        let mut best_height = u32::MAX;
        let mut best_holes = u32::MAX;
        for child in children.iter().rev() {
            let height = child.game.board.height_map().iter().map(|&x| x * x).sum();
            let holes = child.game.board.holes().iter().sum();
            if height < best_height || (height == best_height && holes < best_holes) {
                best_height = height;
                best_holes = holes;
                best_child = Some(child);
            }
        }
        match best_child {
            Some(child) => Evaluation::Success {
                actions: child.actions().collect(),
                score: Some(children.len() as f64),
            },
            None => Evaluation::Fail {
                message: String::from("no valid game actions"),
            },
        }
    }
}

impl Default for SimpleAi {
    fn default() -> Self {
        SimpleAi
    }
}
