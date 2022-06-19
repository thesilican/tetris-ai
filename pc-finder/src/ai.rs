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
        enum Res<'a> {
            NotFound,
            Found {
                hold: bool,
                pc_moves: &'a [GameMove],
            },
        }
        fn rec(game: PcGame, depth: i32, table: &PcTable) -> Res {
            // Iterate through children
            for should_hold in [true, false] {
                let mut game = game.clone();
                if should_hold {
                    if let Err(_) = game.swap_hold() {
                        continue;
                    }
                }
                let next = match game.queue.pop_front() {
                    Some(piece) => piece,
                    None => continue,
                };
                let queue = game.queue;
                let hold = game.hold;
                let current = game.current;
                let leaves = table.leaves(game.board, current);
                for leaf in leaves {
                    if leaf.board() == PcBoard::default() {
                        return Res::Found {
                            hold: should_hold,
                            pc_moves: leaf.moves(),
                        };
                    }
                    let game = PcGame {
                        board: leaf.board(),
                        current: next,
                        hold,
                        queue,
                    };
                    if let Res::Found { .. } = rec(game, depth + 1, table) {
                        return Res::Found {
                            hold: should_hold,
                            pc_moves: leaf.moves(),
                        };
                    }
                }
            }
            Res::NotFound
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
        match res {
            Res::NotFound => AiRes::Fail {
                reason: "unable to find pc solution".to_string(),
            },
            Res::Found { hold, pc_moves } => {
                let mut moves = Vec::new();
                if hold {
                    moves.push(GameMove::Hold);
                }
                moves.extend(pc_moves);
                AiRes::Success {
                    moves,
                    score: Some(0.0),
                }
            }
        }
    }
}
