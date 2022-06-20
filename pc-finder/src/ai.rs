use crate::{PcBoard, PcTable};
use common::*;
use std::collections::VecDeque;

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

#[derive(Debug, Clone, Copy)]
struct PcChild<'a> {
    game: PcGame,
    hold: bool,
    pc_moves: &'a [GameMove],
}

#[derive(Debug)]
pub struct PcFinderAi {
    table: PcTable,
    simple_ai: SimpleAi,
}
impl PcFinderAi {
    pub fn new() -> Self {
        PcFinderAi {
            table: PcTable::load_static(),
            simple_ai: SimpleAi::new(),
        }
    }
}
impl Ai for PcFinderAi {
    fn evaluate(&mut self, game: &Game) -> AiRes {
        fn rec<'a>(game: PcGame, table: &'a PcTable) -> Option<PcChild<'a>> {
            let ancestors = game.children(table).collect::<Vec<_>>();
            for &ancestor in ancestors.iter() {
                if ancestor.game.board == PcBoard::default() {
                    return Some(ancestor);
                }
            }
            let mut counts = ancestors
                .iter()
                .enumerate()
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            struct Frame {
                game: PcGame,
                depth: usize,
                parent: usize,
            }
            let mut queue = ancestors
                .iter()
                .enumerate()
                .map(|(i, child)| Frame {
                    game: child.game,
                    depth: 1,
                    parent: i,
                })
                .collect::<VecDeque<_>>();
            // BFS through children
            while let Some(frame) = queue.pop_front() {
                for child in frame.game.children(table) {
                    counts[frame.parent] += 1;
                    if child.game.board == PcBoard::default() {
                        return Some(ancestors[frame.parent]);
                    }
                    queue.push_back(Frame {
                        game: child.game,
                        depth: frame.depth + 1,
                        parent: frame.parent,
                    })
                }
            }
            match counts.iter().enumerate().max_by_key(|(_, &x)| x) {
                Some((i, _)) => Some(ancestors[i]),
                None => None,
            }
        }
        let pc_game = match PcGame::from_game(*game) {
            Ok(pc_game) => pc_game,
            Err(_) => return self.simple_ai.evaluate(game),
        };
        let res = rec(pc_game, &self.table);
        match res {
            Some(child) => {
                let mut moves = Vec::new();
                if child.hold {
                    moves.push(GameMove::Hold);
                }
                moves.extend(child.pc_moves);
                AiRes::Success {
                    moves,
                    score: Some(0.0),
                }
            }
            None => return self.simple_ai.evaluate(game),
        }
    }
}
