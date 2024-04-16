mod tree;

use common::*;
use std::collections::BinaryHeap;
use tree::{Node, Tree};

struct ScoredNode(Node);
impl PartialEq for ScoredNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.score == other.0.score
    }
}
impl Eq for ScoredNode {}
impl PartialOrd for ScoredNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.score.partial_cmp(&other.0.score)
    }
}
impl Ord for ScoredNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.score.total_cmp(&other.0.score)
    }
}

pub struct DeepAi {
    pub depth: usize,
    pub take: usize,
    pub step: usize,
    tree: Tree,
}
impl DeepAi {
    pub fn new(depth: usize, take: usize) -> Self {
        assert!(depth >= 1);
        DeepAi {
            depth,
            take,
            step: 0,
            tree: Tree::new(),
        }
    }
}
impl Ai for DeepAi {
    fn evaluate(&mut self, game: &Game) -> AiResult {
        if game.hold.is_none() {
            return Ok(AiEval {
                moves: vec![GameMove::Hold],
                score: None,
            });
        }
        self.tree
            .probe_queue(self.step, game.queue.iter().copied())
            .map_err(|e| format!("error probing queue: {e}"))?;
        let children = match game.children() {
            Ok(children) => children,
            Err(_) => {
                return Ok(AiEval {
                    moves: vec![GameMove::HardDrop],
                    score: None,
                })
            }
        };
        let mut best_score = f32::NEG_INFINITY;
        let mut best_node = None;
        for child in children {
            let node = Node::new(child.game, self.step + 1);
            let score = match self.dfs(&node, 0) {
                Some(score) => score,
                None => continue,
            };
            if best_node.is_none() || score > best_score {
                best_node = Some(child);
                best_score = score;
            }
        }
        self.step += 1;
        match best_node {
            Some(child) => Ok(AiEval {
                moves: child.moves().collect(),
                score: Some(best_score as f64),
            }),
            None => Err("No valid moves".to_string()),
        }
    }
}
impl DeepAi {
    fn dfs(&mut self, node: &Node, depth: usize) -> Option<f32> {
        if depth == self.depth {
            return Some(node.score);
        }
        let mut heap = self
            .tree
            .get(node)
            .ok()?
            .into_iter()
            .copied()
            .map(ScoredNode)
            .collect::<BinaryHeap<ScoredNode>>();
        let mut max = f32::NEG_INFINITY;
        for _ in 0..self.take {
            let Some(node) = heap.pop() else {
                break;
            };
            if let Some(score) = self.dfs(&node.0, depth + 1) {
                max = max.max(score);
            }
        }
        Some(max)
    }
}
