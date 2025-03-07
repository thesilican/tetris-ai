use anyhow::{bail, Result};
use core::f32;
use libtetris::{Board, Fin, Game, Piece, PieceQueue, PieceType};
use smallvec::SmallVec;
use std::{
    collections::{BinaryHeap, HashMap},
    hash::{Hash, Hasher},
};

use crate::param::Params;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub board: Board,
    pub active: Piece,
    pub hold: Option<PieceType>,
    pub can_hold: bool,
    pub step: usize,
    pub score: f32,
}

impl Node {
    pub fn new(game: Game, step: usize, score: f32) -> Self {
        Node {
            board: game.board,
            active: game.active,
            hold: game.hold,
            can_hold: game.can_hold,
            step,
            score,
        }
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

#[derive(Debug, Clone, Copy)]
pub struct Edge(Node, f32);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.0.score + self.1 == other.0.score + other.1
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.0.score + self.1).total_cmp(&(other.0.score + other.1)))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Tree {
    pub edges: HashMap<Node, SmallVec<[Edge; 64]>>,
    pub queue: Vec<PieceType>,
    pub params: Params,
    pub dfs_depth: usize,
    pub dfs_take: usize,
}

impl Tree {
    pub fn new(params: Params, dfs_depth: usize, dfs_take: usize) -> Self {
        Tree {
            edges: HashMap::new(),
            queue: Vec::new(),
            params,
            dfs_depth,
            dfs_take,
        }
    }

    pub fn clear(&mut self) {
        self.edges.clear();
        self.queue.clear();
    }

    fn insert(&mut self, node: &Node) -> Result<()> {
        let game = node.to_game(&self.queue)?;
        let children = game.children(Fin::Simple1);
        let mut nodes = SmallVec::new();
        for child in children {
            let score = self.params.eval_node(&game.board);
            let node = Node::new(child.game, node.step + 1, score);
            let score = self.params.eval_edge(&child.lock_info);
            nodes.push(Edge(node, score));
        }
        self.edges.insert(*node, nodes);
        Ok(())
    }

    fn children(&mut self, node: &Node) -> Result<&[Edge]> {
        if !self.edges.contains_key(node) {
            self.insert(node)?;
        }
        Ok(self.edges.get(node).unwrap())
    }

    fn dfs(&mut self, node: &Node, depth: usize) -> Result<f32> {
        if depth == self.dfs_depth {
            return Ok(node.score);
        }

        let nodes = self.children(node)?;

        let mut heap = nodes.into_iter().copied().collect::<BinaryHeap<Edge>>();

        let mut max = f32::NEG_INFINITY;
        for _ in 0..self.dfs_take {
            let Some(Edge(node, edge_score)) = heap.pop() else {
                break;
            };
            let score = node.score + edge_score + self.dfs(&node, depth + 1)?;
            max = max.max(score);
        }
        Ok(max)
    }

    pub fn dfs_game(&mut self, game: &Game, step: usize) -> Result<f32> {
        let score = self.params.eval_node(&game.board);
        let node = Node::new(*game, step, score);
        self.dfs(&node, 1)
    }

    pub fn extend_queue(&mut self, step: usize, pieces: PieceQueue) -> Result<()> {
        let mut i = step;
        for piece in pieces.iter() {
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

    pub fn compactify(&mut self, steps: usize) {
        for _ in 0..steps {
            self.queue.remove(0);
        }

        let mut new_edges = HashMap::new();
        for (key, val) in &self.edges {
            if key.step < steps {
                continue;
            }
            let mut new_key = key.clone();
            new_key.step -= steps;
            let mut new_val = SmallVec::new();
            for edge in val {
                let mut new_edge = edge.clone();
                new_edge.0.step -= steps;
                new_val.push(new_edge);
            }
            new_edges.insert(new_key, new_val);
        }
        self.edges = new_edges;
    }
}
