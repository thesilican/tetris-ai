use crate::{PcBoard, PcTable};
use common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn children<'a, 'b>(
        &'a self,
        table: &'b PcTable,
    ) -> impl Iterator<Item = PcChild<'b>> + 'b {
        let game = self.clone();
        [false, true]
            .into_iter()
            .filter_map(move |should_hold| {
                let mut game = game.clone();
                if should_hold {
                    let hold = match game.hold {
                        Some(piece) => piece,
                        None => match game.queue.pop_front() {
                            Some(piece) => piece,
                            None => return None,
                        },
                    };
                    game.hold = Some(game.current);
                    game.current = hold;
                }
                let dropped = game.current;
                let current = match game.queue.pop_front() {
                    Some(piece) => piece,
                    None => return None,
                };
                let hold = game.hold;
                let queue = game.queue;
                let iter = table.leaves(game.board, dropped).map(move |leaf| PcChild {
                    game: PcGame {
                        board: leaf.board(),
                        current,
                        hold,
                        queue,
                    },
                    hold: should_hold,
                    pc_moves: leaf.moves(),
                });
                Some(iter)
            })
            .flatten()
    }
}

struct PcChild<'a> {
    game: PcGame,
    hold: bool,
    pc_moves: &'a [GameMove],
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
        fn rec<'a>(game: PcGame, depth: i32, table: &'a PcTable) -> Res<'a> {
            let mut total_score: i32 = 0;
            let mut best_res = Res {
                score: i32::MIN,
                hold: false,
                pc_moves: &[],
            };
            // Iterate through children
            for child in game.children(table) {
                if child.game.board == PcBoard::default() {
                    return Res {
                        score: i32::MAX,
                        hold: child.hold,
                        pc_moves: child.pc_moves,
                    };
                }
                let res = rec(child.game, depth + 1, table);
                total_score = total_score.saturating_add(res.score).saturating_add(1);
                if res.score > best_res.score {
                    best_res.score = res.score;
                    best_res.hold = child.hold;
                    best_res.pc_moves = child.pc_moves;
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
