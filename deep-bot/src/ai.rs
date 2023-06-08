use common::*;

use crate::{Node, NodeTree};

pub struct DeepAi {
    depth: usize,
    take: usize,
    step: usize,
    cache: NodeTree,
}
impl DeepAi {
    pub fn new(depth: usize, take: usize) -> Self {
        assert!(depth >= 1);
        DeepAi {
            depth,
            take,
            step: 0,
            cache: NodeTree::new(),
        }
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
}
impl Ai for DeepAi {
    fn evaluate(&mut self, game: &Game) -> AiResult {
        self.cache.probe_queue(self.step, &game.queue);
        let children = game.child_states(&PERMS_1F);
        let mut best_score = f32::NEG_INFINITY;
        let mut best_child = None;
        self.step += 1;
        for child in children {
            let key = Node::new(child.game);
            let score = self.dfs(&key, 0);
            if best_child == None || score > best_score {
                best_child = Some(child);
                best_score = score;
            }
        }
        match best_child {
            Some(child) => AiResult::Success {
                moves: child.moves().collect(),
                score: Some(best_score as f64),
            },
            None => AiResult::Fail {
                reason: "No valid moves".to_string(),
            },
        }
    }
}
impl DeepAi {
    fn dfs(&mut self, node: &Node, depth: usize) -> f32 {
        if depth == self.depth {
            return node.score;
        }
        let mut children = self.cache.get(node, self.step + depth).unwrap().to_vec();
        if children.len() == 0 {
            return f32::NEG_INFINITY;
        }
        children.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        children
            .into_iter()
            .rev()
            .take(self.take)
            .map(|node| {
                let score = self.dfs(&node, depth + 1);
                score
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}
