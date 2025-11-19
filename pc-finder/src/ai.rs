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

impl<'a> PcChild<'a> {
    pub fn actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        if self.hold {
            actions.push(Action::Hold);
        }
        actions.extend_from_slice(self.pc_moves);
        actions
    }
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
        let mut pc_game = match PcGame::from_game(*game) {
            Ok(pc_game) => pc_game,
            Err(_) => return self.simple_ai.evaluate(game),
        };

        // Trick: slightly extend the queue with likely pieces to get more
        // accurate results
        for piece_type in PieceType::ALL {
            let found = pc_game.queue.iter().find(|&p| p == piece_type).is_some();
            if !found && pc_game.queue.len() != PIECE_QUEUE_MAX_LEN {
                pc_game.queue.enqueue(piece_type);
            }
        }

        let children = pc_game.children(&self.table).collect::<Vec<_>>();
        for &child in children.iter() {
            if child.game.board == PcBoard::default() {
                return Evaluation::Success {
                    actions: child.actions(),
                    score: 0.0,
                };
            }
        }

        // Determine child with most paths
        let mut counts = children
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        struct Frame {
            game: PcGame,
            depth: usize,
            ancestor: usize,
        }
        let mut queue = children
            .iter()
            .enumerate()
            .map(|(i, child)| Frame {
                game: child.game,
                depth: 1,
                ancestor: i,
            })
            .collect::<VecDeque<_>>();

        // BFS through children
        while let Some(frame) = queue.pop_front() {
            for child in frame.game.children(&self.table) {
                counts[frame.ancestor] += 1;
                if child.game.board == PcBoard::default() {
                    return Evaluation::Success {
                        actions: children[frame.ancestor].actions(),
                        score: frame.depth as f32,
                    };
                }
                queue.push_back(Frame {
                    game: child.game,
                    depth: frame.depth + 1,
                    ancestor: frame.ancestor,
                })
            }
        }
        let best_child = counts
            .iter()
            .enumerate()
            .max_by_key(|&(_, x)| x)
            .map(|(i, _)| children[i]);
        match best_child {
            Some(best_child) => Evaluation::Success {
                actions: best_child.actions(),
                score: 10.0,
            },
            None => self.simple_ai.evaluate(game),
        }
    }
}
