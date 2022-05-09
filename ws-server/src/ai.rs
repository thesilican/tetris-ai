use common::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Deserialize)]
struct WsReq {
    nonce: i32,
    game: Game,
}
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum WsRes {
    Success {
        success: bool,
        moves: Vec<GameMove>,
        score: Option<f64>,
        nonce: i32,
    },
    Fail {
        success: bool,
        reason: String,
        nonce: i32,
    },
}

// Wrapper around common::Ai that handles nonces
pub struct WsAi {
    inner: Box<dyn Ai>,
    expected: Option<Game>,
}
impl WsAi {
    pub fn evaluate(&mut self, req: &str) -> Result<String, GenericErr> {
        const FAIL_ON_DISCREPENCY: bool = false;
        let start = Instant::now();
        let req = serde_json::from_str::<WsReq>(req)?;
        let nonce = req.nonce;

        let res = self.inner.evaluate(&req.game);
        let ws_res = match res {
            AiRes::Success { moves, score } => {
                let mut game = req.game;
                let discrepency = match self.expected {
                    Some(expected) => expected.board != game.board,
                    None => false,
                };
                if discrepency && FAIL_ON_DISCREPENCY {
                    println!(
                        "Board Discrepency!\nExpected: \n{}\nGot: \n{}\n",
                        self.expected.unwrap(),
                        game
                    );
                    self.expected = None;
                    WsRes::Fail {
                        success: false,
                        reason: "board discrepency".to_string(),
                        nonce,
                    }
                } else {
                    for game_move in moves.iter() {
                        game.make_move(*game_move);
                    }
                    self.expected = Some(game);
                    println!(
                        "{}\nMoves: {:?}\nScore: {}\nElapsed: {:?}\n",
                        game,
                        moves,
                        score
                            .map(|x| format!("{}", x))
                            .unwrap_or("None".to_string()),
                        start.elapsed()
                    );
                    WsRes::Success {
                        success: true,
                        moves,
                        score,
                        nonce,
                    }
                }
            }
            AiRes::Fail { reason } => {
                println!("{}\nEvaluation Failed\nReason: {}\n", req.game, reason);
                WsRes::Fail {
                    success: false,
                    reason,
                    nonce,
                }
            }
        };
        Ok(serde_json::to_string(&ws_res).unwrap())
    }
}

pub fn get_ai() -> WsAi {
    // let ai = SimpleAi::new();
    let ai = deep_bot::DeepAi::new(3, 10);
    WsAi {
        expected: None,
        inner: Box::new(ai),
    }
}
