use anyhow::{bail, Result};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use common::{Board, Game, Piece, PieceType};
use smallvec::SmallVec;

#[derive(Clone, Copy)]
pub struct Node {
    board: Board,
    active: Piece,
    hold: Option<PieceType>,
    can_hold: bool,
    step: usize,
    pub score: f32,
}
impl Node {
    pub fn new(game: Game, step: usize) -> Self {
        Node {
            board: game.board,
            active: game.active,
            hold: game.hold,
            can_hold: game.can_hold,
            step,
            score: Node::calculate_score(&game),
        }
    }
    fn calculate_score(game: &Game) -> f32 {
        let height_map = game.board.height_map();

        // Board height
        let board_height = height_map.iter().map(|&x| (x * x) as f32).sum::<f32>();

        // Board Bumpiness
        let bumpiness = height_map
            .windows(2)
            .map(|x| (x[0] as f32 - x[1] as f32).abs())
            .sum::<f32>();

        // Board holes
        let holes = game.board.holes().iter().sum::<u32>() as f32;

        // Free right column
        let right_col = if (0..10).all(|i| !game.board.get(9, i)) {
            1.0
        } else {
            0.0
        };
        (-0.2 * board_height) + (-0.1 * bumpiness) + (-10.0 * holes) + (0.0 * right_col)
    }
    fn to_game(&self, queue: &[PieceType]) -> Result<Game> {
        if self.step >= queue.len() {
            bail!(
                "step {} greater than queue length {}",
                self.step,
                queue.len()
            );
        }
        Ok(Game::from_parts(
            self.board,
            self.active,
            self.hold,
            &queue[self.step..],
            self.can_hold,
        ))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
            && self.active == other.active
            && self.hold == other.hold
            && self.can_hold == other.can_hold
            && self.step == other.step
    }
}
impl Eq for Node {}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.active.hash(state);
        self.hold.hash(state);
        self.can_hold.hash(state);
        self.step.hash(state);
    }
}

pub struct Tree {
    nodes: HashMap<Node, SmallVec<[Node; 100]>>,
    queue: Vec<PieceType>,
}
impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: HashMap::new(),
            queue: Vec::new(),
        }
    }
    pub fn probe_queue(
        &mut self,
        step: usize,
        pieces: impl IntoIterator<Item = PieceType>,
    ) -> Result<()> {
        let mut i = step;
        for piece in pieces {
            if i < self.queue.len() {
                if piece != self.queue[i] {
                    bail!("queue inconsistency at step {i}");
                }
            } else if i == self.queue.len() {
                self.queue.push(piece);
            } else {
                bail!(
                    "queue jumped to step {i}, currently length {}",
                    self.queue.len()
                )
            }
            i += 1;
        }
        Ok(())
    }
    pub fn get(&mut self, node: &Node) -> Result<&[Node]> {
        if !self.nodes.contains_key(node) {
            self.insert(node)?;
        }
        Ok(self.nodes.get(node).unwrap())
    }
    pub fn insert(&mut self, node: &Node) -> Result<()> {
        let game = node.to_game(&self.queue)?;
        let children = game.children()?;
        let mut nodes = SmallVec::new();
        for child in children {
            let node = Node::new(child.game, node.step + 1);
            nodes.push(node);
        }
        self.nodes.insert(*node, nodes);
        Ok(())
    }
}
