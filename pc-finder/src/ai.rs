use crate::{PcBoard, PcTable};
use anyhow::Result;
use libtetris::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PcGame {
    board: PcBoard,
    current: PieceType,
    hold: Option<PieceType>,
    queue: PieceQueue,
}

impl PcGame {
    pub fn from_game(game: Game) -> Result<Self> {
        let board = PcBoard::try_from(game.board)?;
        Ok(PcGame {
            board,
            current: game.active.piece_type,
            hold: game.hold,
            queue: game.queue,
        })
    }

    pub fn children<'a>(&self, table: &'a PcTable) -> impl Iterator<Item = PcChild<'a>> + 'a {
        let game = *self;
        [false, true]
            .into_iter()
            .filter_map(move |should_hold| {
                let mut game = game;
                if should_hold {
                    let hold = match game.hold {
                        Some(piece) => piece,
                        None => match game.queue.dequeue() {
                            Some(piece) => piece,
                            None => return None,
                        },
                    };
                    game.hold = Some(game.current);
                    game.current = hold;
                }
                let dropped = game.current;
                let current = match game.queue.dequeue() {
                    Some(piece) => piece,
                    None => return None,
                };
                let hold = game.hold;
                let queue = game.queue;
                let iter = table
                    .children(game.board, dropped)
                    .map(move |leaf| PcChild {
                        game: PcGame {
                            board: leaf.board(),
                            current,
                            hold,
                            queue,
                        },
                        hold: should_hold,
                        pc_moves: leaf.actions(),
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
    pc_moves: &'a [Action],
}

#[derive(Debug, Default)]
pub struct PcFinderAi {
    table: PcTable,
    simple_ai: SimpleAi,
}

impl PcFinderAi {
    pub fn new(pc_table: PcTable) -> Self {
        PcFinderAi {
            table: pc_table,
            simple_ai: SimpleAi::new(),
        }
    }
}

impl Ai for PcFinderAi {
    fn evaluate(&mut self, game: &Game) -> Evaluation {
        fn rec(game: PcGame, table: &PcTable) -> Option<PcChild<'_>> {
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
            counts
                .iter()
                .enumerate()
                .max_by_key(|(_, &x)| x)
                .map(|(i, _)| ancestors[i])
        }
        let pc_game = match PcGame::from_game(*game) {
            Ok(pc_game) => pc_game,
            Err(_) => return self.simple_ai.evaluate(game),
        };
        let res = rec(pc_game, &self.table);
        match res {
            Some(child) => {
                let mut actions = Vec::new();
                if child.hold {
                    actions.push(Action::Hold);
                }
                actions.extend(child.pc_moves);
                Evaluation::Success {
                    actions,
                    score: 0.0,
                }
            }
            None => self.simple_ai.evaluate(game),
        }
    }
}
