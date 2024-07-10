use crate::{
    model::{Bag, Game},
    Action, LockInfo,
};
use serde::{Deserialize, Serialize};
use std::{
    thread,
    time::{Duration, Instant},
};

/// The result of an Ai evaluation, returns a list of game actions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "crate::serde::SerializedEvaluation")]
#[serde(into = "crate::serde::SerializedEvaluation")]
pub enum Evaluation {
    Success {
        actions: Vec<Action>,
        score: Option<f64>,
    },
    Fail {
        message: String,
    },
}

/// An object that can evaluate Tetris game states
pub trait Ai {
    /// Evaluate a given tetris board
    fn evaluate(&mut self, game: &Game) -> Evaluation;

    /// Prints out a pretty demo for an Ai
    fn demo(&mut self) {
        let mut bag = Bag::new_rng7(0);
        let mut game = Game::from_bag(&mut bag);
        println!("{game}");
        loop {
            let start = Instant::now();
            let res = self.evaluate(&game);
            let elapsed = start.elapsed();

            match res {
                Evaluation::Success { actions, score } => {
                    let mut top_out = false;
                    for &action in &actions {
                        if let Action::HardDrop = action {
                            let info = game.hard_drop();
                            if let Some(LockInfo { top_out: true, .. }) = info {
                                println!("TOP OUT");
                                top_out = true;
                                break;
                            }
                        } else {
                            game.apply(action);
                        }
                    }
                    let score = match score {
                        Some(score) => &format!("{score:0.2}"),
                        None => "-",
                    };
                    println!();
                    println!("{game}");
                    println!("{actions:?}");
                    println!("Evaluated in {elapsed:?}");
                    println!("Evaluation score: {score}");
                    if top_out {
                        break;
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
        let children = game.children_fast();
        if children.len() == 0 {
            // Immediately hard drop to reset the piece position
            return Evaluation::Success {
                actions: vec![Action::HardDrop],
                score: None,
            };
        }
        let mut best_child = None;
        let mut best_height = i32::MAX;
        let mut best_holes = i32::MAX;
        for child in children.iter().rev() {
            let height = child
                .game
                .board
                .height_map()
                .iter()
                .map(|&x| {
                    let x = x as i32;
                    x * x
                })
                .sum();
            let holes = child.game.board.holes().iter().map(|&x| x as i32).sum();
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
