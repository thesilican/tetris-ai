use common::api::*;
use common::model::*;
use rayon::prelude::*;
use std::ops::Neg;

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
            Some((score, moves)) => AiRes::Success {
                score: Some(score.into()),
                moves: moves.to_vec(),
            },
            None => AiRes::Fail {
                reason: "No valid moves".to_string(),
            },
        }
    }
}
impl DeepAi {
    fn score(&self, game: &Game) -> f32 {
        game.board
            .height_map
            .iter()
            .map(|x| (*x as f32).powi(2))
            .sum::<f32>()
            .neg()
    }

    fn dfs(&self, game: &Game, depth: usize) -> Option<(f32, &'static [GameMove])> {
        if depth == 0 {
            return Some((self.score(game), &[]));
        }
        let child_states = game.child_states(&MOVES_1F);
        let mut child_states = child_states
            .par_iter()
            .map(|child_state| (self.score(&child_state.game), *child_state))
            .collect::<Vec<_>>();
        child_states.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        child_states
            .par_iter()
            .take(self.take)
            .filter_map(
                |(_, child_state)| match self.dfs(&child_state.game, depth - 1) {
                    Some((score, _)) => Some((score, child_state.moves)),
                    None => None,
                },
            )
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }
}
