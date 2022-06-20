use crate::{PcBoard, PcTable};
use common::*;

#[derive(Debug, Clone, Copy, Default)]
struct PcGame {
    board: PcBoard,
    current: PieceType,
    hold: Option<PieceType>,
    queue: ArrDeque<PieceType, GAME_MAX_QUEUE_LEN>,
}
impl PcGame {
    pub fn from_game(game: Game) -> GenericResult<Self> {
        let board = PcBoard::try_from(game.board)?;
        Ok(PcGame {
            board,
            current: game.current_piece.piece_type,
            hold: game.hold_piece,
            queue: game.queue_pieces,
        })
    }
    pub fn swap_hold(&mut self) -> Result<(), ()> {
        let hold = match self.hold {
            Some(piece) => piece,
            None => match self.queue.pop_front() {
                Some(piece) => piece,
                None => return Err(()),
            },
        };
        self.hold = Some(self.current);
        self.current = hold;
        Ok(())
    }
    pub fn hard_drop(&mut self) -> Result<(), ()> {
        match self.queue.pop_front() {
            Some(piece) => {
                self.current = piece;
                Ok(())
            }
            None => Err(()),
        }
    }
}

#[derive(Debug)]
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
        struct Res<'a> {
            score: i32,
            hold: bool,
            pc_moves: &'a [GameMove],
        }
        fn rec(game: PcGame, depth: i32, table: &PcTable) -> Res {
            let mut total_score: i32 = 0;
            let mut best_res = Res {
                score: i32::MIN,
                hold: false,
                pc_moves: &[],
            };
            // Iterate through children
            for should_hold in [true, false] {
                let mut game = game.clone();
                if should_hold {
                    if let Err(_) = game.swap_hold() {
                        continue;
                    }
                }
                let current = game.current;
                if let Err(_) = game.hard_drop() {
                    continue;
                }
                let leaves = table.leaves(game.board, current);
                for leaf in leaves {
                    if leaf.board() == PcBoard::default() {
                        return Res {
                            score: i32::MAX,
                            hold: should_hold,
                            pc_moves: leaf.moves(),
                        };
                    }
                    let game = PcGame {
                        board: leaf.board(),
                        current: game.current,
                        hold: game.hold,
                        queue: game.queue,
                    };
                    let res = rec(game, depth + 1, table);
                    if res.score == i32::MAX {
                        return Res {
                            score: i32::MAX,
                            hold: should_hold,
                            pc_moves: leaf.moves(),
                        };
                    }
                    total_score = total_score.saturating_add(res.score).saturating_add(1);
                    if res.score > best_res.score {
                        best_res.score = res.score;
                        best_res.hold = should_hold;
                        best_res.pc_moves = leaf.moves();
                    }
                }
            }
            Res {
                score: total_score,
                ..best_res
            }
        }
        let pc_game = match PcGame::from_game(*game) {
            Ok(pc_game) => pc_game,
            Err(_) => {
                return AiRes::Fail {
                    reason: "Not a pc game board".to_string(),
                }
            }
        };
        let res = rec(pc_game, 0, &self.table);
        if res.score == i32::MIN {
            AiRes::Fail {
                reason: "unable to find pc solution".to_string(),
            }
        } else {
            let mut moves = Vec::new();
            if res.hold {
                moves.push(GameMove::Hold);
            }
            moves.extend(res.pc_moves);
            AiRes::Success {
                moves,
                score: Some(0.0),
            }
        }
    }
}
