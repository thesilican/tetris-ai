mod optimizer;
mod param;
mod tree;

use libtetris::*;
pub use optimizer::*;
pub use param::*;
pub use tree::*;

pub struct TreeAi {
    pub depth: usize,
    pub take: usize,
    pub step: usize,
    tree: Tree,
}

impl TreeAi {
    pub fn new(depth: usize, take: usize, params: Params) -> Self {
        assert!(depth >= 1);
        TreeAi {
            depth,
            take,
            step: 0,
            tree: Tree::new(params, depth, take),
        }
    }
}

impl Ai for TreeAi {
    fn evaluate(&mut self, game: &Game) -> Evaluation {
        // The AI always expects there to be a hold piece
        if game.hold.is_none() {
            return Evaluation::Success {
                actions: vec![Action::Hold],
                score: 0.,
            };
        }

        // Regularly perform garbage collection
        const COMPACTIFY_STEP: usize = 20;
        if self.step > COMPACTIFY_STEP {
            self.step -= COMPACTIFY_STEP;
            self.tree.compactify(COMPACTIFY_STEP);
        }

        let result = self.tree.extend_queue(self.step, game.queue);
        if result.is_err() {
            return Evaluation::Fail {
                message: "Failed to extend queue".to_string(),
            };
        }

        let children = game.children(Fin::Simple1);

        let mut best_score = f32::NEG_INFINITY;
        let mut best_child = None;
        for child in children {
            let edge_score = self.tree.params.eval_edge(&child.lock_info);
            let Ok(node_score) = self.tree.dfs_game(&child.game, self.step + 1) else {
                return Evaluation::Fail {
                    message: "Queue not sufficiently long".to_string(),
                };
            };
            let score = edge_score + node_score;
            if best_child.is_none() || score > best_score {
                best_child = Some(child);
                best_score = score;
            }
        }

        self.step += 1;

        match best_child {
            Some(child) => Evaluation::Success {
                actions: child.actions().collect(),
                score: best_score,
            },
            None => Evaluation::Fail {
                message: "No valid moves".to_string(),
            },
        }
    }
}
