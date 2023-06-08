use anyhow::Result;
use common::{ArrDeque, Board, Game, Piece, PieceType, PERMS_1F};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Copy)]
pub struct Node {
    pub board: Board,
    pub active: Piece,
    pub hold: Option<PieceType>,
    pub can_hold: bool,
    // Below is cached, do not use for comparison/hashing
    pub score: f32,
}
impl Node {
    pub fn new(game: Game) -> Self {
        Node {
            board: game.board,
            active: game.active,
            hold: game.hold,
            can_hold: game.can_hold,
            score: Node::calculate_score(game),
        }
    }
    pub fn calculate_score(game: Game) -> f32 {
        let height_map = game.board.height_map();

        // Board height
        let board_height = height_map.iter().map(|&x| x as f32).sum::<f32>();

        // Board Bumpiness
        let bumpiness = height_map
            .windows(2)
            .map(|x| (x[0] - x[1]).abs() as f32)
            .sum::<f32>();

        // Board holes
        let mut holes = 0.0;
        for i in 0..10 {
            let height = height_map[i] as usize;
            let mut block = false;
            for j in (0..height).rev() {
                if game.board.get(i, j) {
                    block = true;
                } else {
                    if block {
                        holes += 1.0;
                    }
                }
            }
        }

        // Free right column
        let right_col = if (0..10).all(|i| !game.board.get(9, i)) {
            1.0
        } else {
            0.0
        };
        (-0.1 * board_height) + (-1.0 * bumpiness) + (-1.0 * holes) + (0.0 * right_col)
    }
    pub fn create_game(&self, queue: &[PieceType], step: usize) -> Game {
        if step >= queue.len() {
            panic!("step greater than queue length");
        }
        let queue = &queue[step..];
        Game::from_parts(self.board, self.active, self.hold, queue, self.can_hold)
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
            && self.active == other.active
            && self.hold == other.hold
            && self.can_hold == other.can_hold
    }
}
impl Eq for Node {}
impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.active.hash(state);
        self.hold.hash(state);
        self.can_hold.hash(state);
    }
}

pub struct NodeTree {
    queue: Vec<PieceType>,
    map: HashMap<Node, Vec<Node>>,
}
impl NodeTree {
    pub fn new() -> Self {
        NodeTree {
            map: HashMap::new(),
            queue: Vec::new(),
        }
    }
    pub fn get(&mut self, node: &Node, step: usize) -> Result<&[Node]> {
        if !self.map.contains_key(node) {
            self.generate(node, step);
        }
        Ok(self.map.get(node).unwrap())
    }
    fn generate(&mut self, node: &Node, step: usize) {
        let game = node.create_game(&self.queue, step);
        let children = game.child_states(&PERMS_1F);
        let values = children.into_iter().map(|x| Node::new(x.game)).collect();
        self.map.insert(*node, values);
    }
    // Pushes new value to the queue at a certain step,
    // ensuring that the queue is consistent
    pub fn probe_queue<const N: usize>(
        &mut self,
        step: usize,
        queue: &ArrDeque<PieceType, N>,
    ) -> bool {
        if step > self.queue.len() {
            false
        } else {
            for (a, b) in self.queue[step..].iter().zip(queue.iter()) {
                if a != b {
                    return false;
                }
            }
            let start = self.queue.len() - step;
            self.queue.extend(queue.iter().skip(start));
            true
        }
    }
}
