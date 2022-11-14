use common::*;

pub struct DeepAi {
    depth: usize,
    take: usize,
}
impl DeepAi {
    pub fn new(depth: usize, take: usize) -> Self {
        assert!(depth >= 1);
        DeepAi { depth, take }
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
}
impl Ai for DeepAi {
    fn evaluate(&mut self, game: &Game) -> AiRes {
        let result = self.dfs(game, self.depth);
        match result {
            Some((score, Some(child))) => AiRes::Success {
                score: Some(score.into()),
                moves: child.moves().collect(),
            },
            _ => AiRes::Fail {
                reason: "No valid moves".to_string(),
            },
        }
    }
}
impl DeepAi {
    fn score(&self, game: &Game) -> f32 {
        let height_map = game.board.height_map();

        // Board height
        let board_height = height_map
            .iter()
            .map(|&x| {
                let x = x as f32;
                x.powi(2)
            })
            .sum::<f32>();

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
        (-1. * board_height) + (-1.0 * bumpiness) + (-10.0 * holes) + (10.0 * right_col)
    }

    fn dfs(&self, game: &Game, depth: usize) -> Option<(f32, Option<ChildState>)> {
        if depth == 0 {
            return Some((self.score(game), None));
        }
        let child_states = game.child_states(&MOVES_1F);
        let mut child_states = child_states
            .iter()
            .filter(|child_state| !child_state.game.board.topped_out())
            .map(|child_state| (self.score(&child_state.game), *child_state))
            .collect::<Vec<_>>();
        child_states.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        child_states
            .iter()
            .take(self.take)
            .filter_map(|&(score, child_state)| {
                // Special case for perfect clears
                if child_state.game.board.matrix[0] == 0 {
                    return Some((f32::INFINITY, Some(child_state)));
                }
                match self.dfs(&child_state.game, depth - 1) {
                    Some((dfs_score, _)) => Some((score + dfs_score, Some(child_state))),
                    None => None,
                }
            })
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }
}
