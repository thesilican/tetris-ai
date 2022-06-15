use crate::{PcBoard, PcTable};
use common::*;

pub struct PcFinderAi {
    table: PcTable,
}
impl PcFinderAi {
    pub fn new() -> Self {
        PcFinderAi {
            table: PcTable::load_static(),
        }
    }
}
impl Ai for PcFinderAi {
    fn evaluate(&mut self, game: &Game) -> AiRes {
        use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
        static COUNT: AtomicUsize = AtomicUsize::new(0);
        COUNT.store(0, Relaxed);

        struct Res {
            score: f64,
            moves: Vec<GameMove>,
        }
        fn rec(game: Game, depth: i32, table: &PcTable) -> Res {
            COUNT.fetch_add(1, Relaxed);
            let pc_board = match PcBoard::try_from(game.board) {
                Ok(x) => x,
                Err(_) => {
                    return Res {
                        score: f64::NEG_INFINITY,
                        moves: vec![],
                    }
                }
            };

            let mut total_score = 0.0;
            let mut best_score = f64::NEG_INFINITY;
            let mut best_moves = Vec::new();
            for hold in [true, false] {
                let mut game = game.clone();
                if hold {
                    let res = game.make_move(GameMove::Hold);
                    if let GameMoveRes::Fail = res {
                        let count = table.leaves_all(pc_board).count();
                        return Res {
                            score: count as f64,
                            moves: vec![],
                        };
                    }
                }
                if game.queue_pieces.len() == 0 {
                    let count = table.leaves_all(pc_board).count();
                    return Res {
                        score: count as f64,
                        moves: vec![],
                    };
                }
                let piece = game.current_piece.piece_type;
                let children = table.leaves(pc_board, piece);
                for child in children {
                    let mut game = game.clone();
                    let moves = child.moves();
                    for &game_move in moves {
                        game.make_move(game_move);
                    }
                    let res = rec(game, depth + 1, table);
                    if res.score > best_score {
                        best_score = res.score;
                        best_moves.clear();
                        if hold {
                            best_moves.push(GameMove::Hold);
                            best_moves.extend(moves);
                        }
                    }
                    total_score += res.score;
                }
            }
            Res {
                score: total_score,
                moves: best_moves,
            }
        }
        let Res { score, moves } = rec(*game, 0, &self.table);
        println!("{}", COUNT.load(Relaxed));
        if score == f64::NEG_INFINITY {
            AiRes::Fail {
                reason: "Unable to evaulate child board".to_string(),
            }
        } else {
            AiRes::Success {
                score: Some(score),
                moves,
            }
        }
    }
}
