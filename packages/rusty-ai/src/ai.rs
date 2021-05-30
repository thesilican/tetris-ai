use crate::ai_weights::AIWeights;
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
use std::collections::HashMap;

const EVAL_AMOUNT: [usize; 6] = [1, 2, 3, 4, 5, 10];

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

    fn gen_moves(game: &Game) -> Vec<Vec<GameMove>> {
        let mut res = Vec::with_capacity(300);
        for final_rotation in 0..PIECE_NUM_ROTATION {
            for hold in 0..2 {
                let piece = if hold == 0 {
                    game.current_piece.piece_type
                } else {
                    game.hold_piece.unwrap()
                };
                for rotation in 0..PIECE_NUM_ROTATION {
                    let (left, right) = Piece::info_shift_bounds(&piece, rotation as i8);
                    for shift in (-*left)..=*right {
                        let mut moves = Vec::with_capacity(8);
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
                                moves.push(GameMove::RotateRight);
                            }
                            2 => {
                                moves.push(GameMove::SoftDrop);
                                moves.push(GameMove::Rotate180);
                            }
                            3 => {
                                moves.push(GameMove::SoftDrop);
                                moves.push(GameMove::RotateLeft);
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

    fn gen_child_states(game: &Game) -> impl Iterator<Item = (Game, GameDropRes, Vec<GameMove>)> {
        let mut res = HashMap::new();
        let moves_list = RustyAI::gen_moves(game);
        for moves in moves_list {
            let mut child_game = game.clone();
            for game_move in moves.iter() {
                child_game.make_move(game_move);
            }
            let drop_res = match child_game.make_move(&GameMove::HardDrop) {
                GameMoveRes::SuccessDrop(drop_res) => drop_res,
                _ => panic!(),
            };
            if drop_res.top_out {
                continue;
            }
            // Remove duplicates
            let hold = child_game.current_piece != game.current_piece;
            let key = (child_game.board.matrix.clone(), hold);
            if !res.contains_key(&key) {
                res.insert(key, (child_game, drop_res, moves));
            }
        }
        res.into_iter().map(|(_, val)| val)
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

    fn evaluate_recursive(game: &Game, weights: &AIWeights, depth: i32) -> AIEval {
        if depth == 0 || game.queue_pieces.len() == 0 {
            let score = RustyAI::board_score(weights, game);
            return AIEval { score };
        }

        // Only evaluate the top N games, to cut down on duplicates
        let mut games = Vec::new();
        for (game, drop_res, _) in RustyAI::gen_child_states(game) {
            let drop_score = RustyAI::drop_score(&weights, &drop_res, &game);
            let board_score = RustyAI::board_score(&weights, &game);
            let score = drop_score + board_score;
            games.push((score, drop_score, game));
        }
        games.sort_by(|a, b| (b.0).partial_cmp(&a.0).unwrap());
        let amount = EVAL_AMOUNT[depth as usize];
        let top_games = games.into_iter().take(amount);

        let mut best_score = -f32::INFINITY;
        for (_, drop_score, game) in top_games {
            let AIEval { score: eval_score } =
                RustyAI::evaluate_recursive(&game, &weights, depth - 1);
            let score = drop_score + eval_score;
            if score > best_score {
                best_score = score;
            }
        }

        AIEval { score: best_score }
    }

    fn evaluate(&mut self, game: &Game, depth: i32) -> TetrisAIRes {
        assert!(depth >= 1);
        assert!(game.queue_pieces.len() >= 1);
        if game.hold_piece.is_none() {
            return TetrisAIRes::Success {
                score: Some(0.0),
                moves: vec![GameMove::Hold],
            };
        }

        let mut games = Vec::new();

        for (game, drop_res, moves) in RustyAI::gen_child_states(game) {
            let drop_score = RustyAI::drop_score(&self.weights, &drop_res, &game);
            let board_score = RustyAI::board_score(&self.weights, &game);
            let score = drop_score + board_score;
            games.push((score, drop_score, game, moves));
        }
        games.sort_by(|a, b| (b.0).partial_cmp(&a.0).unwrap());
        let amount = EVAL_AMOUNT[depth as usize];
        let top_games = games.into_iter().take(amount);

        let mut best_score = -f32::INFINITY;
        let mut best_moves = vec![];
        for (_, drop_score, game, moves) in top_games {
            let AIEval { score: eval_score } =
                RustyAI::evaluate_recursive(&game, &self.weights, depth - 1);
            let score = drop_score + eval_score;
            if score > best_score {
                best_score = score;
                best_moves = moves;
            }
        }

        // if self.thread_pool.get_thread_count() == 0 {
        //     // Run without threads
        //     for (game, drop_res, moves) in RustyAI::gen_child_states(game) {
        //         let drop_score = RustyAI::drop_score(&self.weights, &drop_res, &game);
        //         let AIEval { score: eval_score } =
        //             RustyAI::evaluate_recursive(&game, &self.weights, depth - 1);
        //         let score = drop_score + eval_score;
        //         if score > best_score {
        //             best_score = score;
        //             best_moves = moves;
        //         }
        //     }
        // } else {
        //     // Create jobs
        //     let mut jobs = Vec::new();
        //     let mut moves_list = Vec::new();
        //     for (game, drop_res, moves) in RustyAI::gen_child_states(game) {
        //         let weights = self.weights.clone();
        //         jobs.push(move || {
        //             let drop_score = RustyAI::drop_score(&weights, &drop_res, &game);
        //             let AIEval { score: eval_score } =
        //                 RustyAI::evaluate_recursive(&game, &weights, depth - 1);
        //             AIEval {
        //                 score: drop_score + eval_score,
        //             }
        //         });
        //         moves_list.push(moves);
        //     }

        //     let results = self.thread_pool.run_jobs(jobs);

        //     for (i, res) in results.iter().enumerate() {
        //         if res.score > best_score {
        //             best_score = res.score;
        //             best_moves = moves_list[i].clone();
        //         }
        //     }
        // }

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
    fn api_evaluate(&mut self, game: &Game) -> TetrisAIRes {
        self.evaluate(game, self.eval_depth)
    }
}
