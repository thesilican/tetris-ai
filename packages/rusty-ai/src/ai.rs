use crate::aiweights::AIWeights;
use crate::threading::ThreadPool;
use common::api::ai::TetrisAI;
use common::api::ai::TetrisAIRes;
use common::model::consts::BOARD_WIDTH;
use common::model::consts::PIECE_NUM_ROTATION;
use common::model::game::Game;
use common::model::game::GameDropRes;
use common::model::game::GameMove;
use common::model::game::GameMoveRes;
use common::model::piece::Piece;
use std::collections::HashSet;

pub struct AIEval {
    pub score: f32,
}

pub struct AIEvalFinal {
    pub score: f32,
    pub moves: Vec<GameMove>,
}
impl From<AIEvalFinal> for TetrisAIRes {
    fn from(eval: AIEvalFinal) -> TetrisAIRes {
        TetrisAIRes::Success {
            moves: eval.moves,
            score: Some(eval.score.into()),
        }
    }
}

pub struct RustyAI {
    pub eval_depth: i32,
    pub weights: AIWeights,
    thread_pool: ThreadPool<AIEval>,
}
impl RustyAI {
    pub fn new(weights: &AIWeights, eval_depth: i32, thread_count: usize) -> Self {
        RustyAI {
            eval_depth,
            weights: weights.clone(),
            thread_pool: ThreadPool::new(thread_count),
        }
    }

    fn gen_moves(current_piece: &Piece, hold_piece: &Piece) -> Vec<Vec<GameMove>> {
        let mut res = Vec::new();
        for final_rotation in 0..PIECE_NUM_ROTATION {
            for hold in 0..2 {
                let piece = if hold == 0 { current_piece } else { hold_piece };
                for rotation in 0..PIECE_NUM_ROTATION {
                    let (left, right) = piece.get_shift_bounds(Some(rotation));
                    for shift in (-*left)..*right {
                        let mut moves = Vec::new();
                        match rotation {
                            0 => (),
                            1 => moves.push(GameMove::RotateRight),
                            2 => moves.push(GameMove::Rotate180),
                            3 => moves.push(GameMove::RotateLeft),
                            _ => unreachable!(),
                        }
                        for _ in 0..shift.abs() {
                            if shift > 0 {
                                moves.push(GameMove::ShiftRight);
                            } else {
                                moves.push(GameMove::ShiftLeft);
                            }
                        }
                        match final_rotation {
                            0 => (),
                            1 => {
                                moves.push(GameMove::SoftDrop);
                                moves.push(GameMove::RotateRight)
                            }
                            2 => {
                                moves.push(GameMove::SoftDrop);
                                moves.push(GameMove::Rotate180)
                            }
                            3 => {
                                moves.push(GameMove::SoftDrop);
                                moves.push(GameMove::RotateLeft)
                            }
                            _ => unreachable!(),
                        }
                        res.push(moves);
                    }
                }
            }
        }
        res
    }

    fn board_score(weights: &AIWeights, game: &Game) -> f32 {
        let mut score = 0.0;
        for i in 0..BOARD_WIDTH {
            score += (game.board.holes[i as usize] as f32) * weights.values[(5 + i) as usize];
        }
        for i in 0..BOARD_WIDTH {
            score += (game.board.height_map[i as usize] as f32) * weights.values[(15 + i) as usize];
        }
        for i in 0..(BOARD_WIDTH - 1) {
            let diff = game.board.height_map[(i + 1) as usize] - game.board.height_map[i as usize];
            score += (diff as f32) * weights.values[(25 + i) as usize];
        }
        score
    }
    fn drop_score(weights: &AIWeights, drop: &GameDropRes, game: &Game) -> f32 {
        let mut score = 0.0;
        if game.board.matrix[0] == 0 {
            score += weights.values[0];
        }
        match drop.lines_cleared {
            0 => (),
            1 => score += weights.values[1],
            2 => score += weights.values[2],
            3 => score += weights.values[3],
            4 => score += weights.values[4],
            _ => unreachable!(),
        };
        score
    }

    fn evaluate_recursive(
        game: &mut Game,
        memo: &mut HashSet<(i32, Game)>,
        weights: &AIWeights,
        depth: i32,
    ) -> AIEval {
        // TODO: update recursive code
        if depth == 0 || game.queue_pieces.len() == 0 {
            let score = RustyAI::board_score(weights, game);
            return AIEval { score };
        }

        let mut best_score = -f32::INFINITY;
        let moves_list =
            RustyAI::gen_moves(&game.current_piece, &game.hold_piece.as_ref().unwrap());
        for moves in moves_list {
            for game_move in moves {
                game.make_move(&game_move);
            }
            let (drop_res, undo_info) = match game.make_move(&GameMove::HardDrop) {
                GameMoveRes::SuccessDrop(drop_res, undo_info) => (drop_res, undo_info),
                // Should never fail since queue len >= 1
                _ => unreachable!(),
            };
            if drop_res.top_out {
                game.undo_move(undo_info).unwrap();
                continue;
            }
            let memo_key = (depth, game.clone());
            if memo.contains(&memo_key) {
                game.undo_move(undo_info).unwrap();
                continue;
            } else {
                memo.insert(memo_key);
            }

            let drop_score = RustyAI::drop_score(&weights, &drop_res, &game);
            let AIEval { score: eval_score } =
                RustyAI::evaluate_recursive(game, memo, &weights, depth - 1);
            let score = drop_score + eval_score;
            if score > best_score {
                best_score = score;
            }
            game.undo_move(undo_info).unwrap();
        }

        AIEval { score: best_score }
    }

    fn evaluate(&mut self, mut game: &mut Game, depth: i32) -> TetrisAIRes {
        assert!(depth >= 1);
        assert!(game.queue_pieces.len() >= 1);
        if game.hold_piece.is_none() {
            return TetrisAIRes::Success {
                score: Some(0.0),
                moves: vec![GameMove::Hold],
            };
        }
        // TODO: Fix game hash to ignore piece
        let mut memo = HashSet::<(i32, Game)>::new();
        let mut best_score = -f32::INFINITY;
        let mut best_moves = vec![];
        let moves_list =
            RustyAI::gen_moves(&game.current_piece, &game.hold_piece.as_ref().unwrap());

        if self.thread_pool.get_thread_count() == 0 {
            // Run without threads
            for moves in moves_list {
                for game_move in moves.iter() {
                    game.make_move(game_move);
                }
                let (drop_res, undo_info) = match game.make_move(&GameMove::HardDrop) {
                    GameMoveRes::SuccessDrop(drop_res, undo_info) => (drop_res, undo_info),
                    // Should never fail since queue len >= 1
                    _ => unreachable!(),
                };
                if drop_res.top_out {
                    game.undo_move(undo_info).unwrap();
                    continue;
                }
                let memo_key = (depth, game.clone());
                if memo.contains(&memo_key) {
                    game.undo_move(undo_info).unwrap();
                    continue;
                } else {
                    memo.insert(memo_key);
                }

                let drop_score = RustyAI::drop_score(&self.weights, &drop_res, &game);
                let AIEval { score: eval_score } =
                    RustyAI::evaluate_recursive(&mut game, &mut memo, &self.weights, depth - 1);
                let score = drop_score + eval_score;
                if score > best_score {
                    best_score = score;
                    best_moves = moves;
                }
                game.undo_move(undo_info).unwrap();
            }
        } else {
            // Create jobs
            let mut jobs = Vec::new();
            for moves in moves_list.iter() {
                let weights = self.weights.clone();
                let mut game = game.clone();
                let moves = moves.clone();
                jobs.push(move || {
                    for game_move in moves.iter() {
                        game.make_move(game_move);
                    }
                    let drop_res = match game.make_move(&GameMove::HardDrop) {
                        GameMoveRes::SuccessDrop(drop_res, _) => drop_res,
                        // Should never fail since queue len >= 1
                        _ => unreachable!(),
                    };
                    if drop_res.top_out {
                        return AIEval {
                            score: -f32::INFINITY,
                        };
                    }
                    let mut memo = HashSet::<(i32, Game)>::new();

                    let drop_score = RustyAI::drop_score(&weights, &drop_res, &game);
                    let AIEval { score: eval_score } =
                        RustyAI::evaluate_recursive(&mut game, &mut memo, &weights, depth - 1);
                    AIEval {
                        score: drop_score + eval_score,
                    }
                });
            }
            let results = self.thread_pool.run_jobs(jobs);

            for (i, res) in results.iter().enumerate() {
                if res.score > best_score {
                    best_score = res.score;
                    best_moves = moves_list[i].clone();
                }
            }
        }

        if best_score == -f32::INFINITY {
            TetrisAIRes::Fail {
                reason: String::from("No valid moves"),
            }
        } else {
            // best_move doesn't include HardDrop
            let best_moves = best_moves
                .into_iter()
                .chain(std::iter::once(GameMove::HardDrop))
                .collect();
            TetrisAIRes::Success {
                score: Some(best_score.into()),
                moves: best_moves,
            }
        }
    }
}
impl TetrisAI for RustyAI {
    fn api_evaluate(&mut self, game: &mut Game) -> TetrisAIRes {
        self.evaluate(game, self.eval_depth)
    }
}
