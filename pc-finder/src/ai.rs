use crate::{PcBoard, PcTable};
use common::*;
use tinyvec::TinyVec;

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
    pub fn children<'a>(&self, table: &'a PcTable) -> TinyVec<[PcChild<'a>; 4]> {
        let mut children = TinyVec::new();
        for should_hold in [true, false] {
            let mut game = self.clone();
            if should_hold {
                let hold = match game.hold {
                    Some(piece) => piece,
                    None => match game.queue.pop_front() {
                        Some(piece) => piece,
                        None => continue,
                    },
                };
                game.hold = Some(game.current);
                game.current = hold;
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
                let game = PcGame {
                    board: leaf.board(),
                    current: next,
                    hold,
                    queue,
                };
                children.push(PcChild {
                    game,
                    hold: should_hold,
                    moves: leaf.moves(),
                });
            }
        }
        children
    }
}

#[derive(Debug, Clone, Default)]
struct PcChild<'a> {
    game: PcGame,
    hold: bool,
    moves: &'a [GameMove],
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
            score: f64,
            hold: bool,
            moves: &'a [GameMove],
        }
        fn rec(game: PcGame, depth: i32, table: &PcTable) -> Res {
            let children = game.children(table);
            let mut res = Res {
                score: f64::NEG_INFINITY,
                hold: false,
                moves: &[],
            };
            for child in children {
                // println!("{}", child.game.board);
                if child.game.board == PcBoard::default() {
                    return Res {
                        score: f64::INFINITY,
                        hold: child.hold,
                        moves: child.moves,
                    };
                }
                let Res { score, .. } = rec(child.game, depth + 1, table);
                if score > res.score {
                    res.score = score;
                    res.hold = child.hold;
                    res.moves = child.moves;
                }
            }
            res
        }
        let pc_game = match PcGame::from_game(*game) {
            Ok(pc_game) => pc_game,
            Err(_) => {
                return AiRes::Fail {
                    reason: "Not a pc game board".to_string(),
                }
            }
        };
        let Res { score, moves, hold } = rec(pc_game, 0, &self.table);
        if score == f64::NEG_INFINITY {
            AiRes::Fail {
                reason: "Unable to find PC solution".to_string(),
            }
        } else {
            let mut moves_ret = Vec::new();
            if hold {
                moves_ret.push(GameMove::Hold);
            }
            moves_ret.extend(moves);
            AiRes::Success {
                score: Some(score),
                moves: moves_ret,
            }
        }
    }
}
